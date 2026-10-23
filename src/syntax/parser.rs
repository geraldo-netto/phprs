use super::{
    error,
    lexer::{Lexer, Token},
    policy::{NodeView, validate_rule},
    reserve,
    tables::*,
    token_ids::*,
};
use crate::contracts::{
    Diagnostic, ErrorKind, Limits, ParseOptions, SyntaxKind, SyntaxNode, SyntaxTree,
};
use std::ops::Range;

struct Entry {
    state: u16,
    node: usize,
}

pub(super) struct Parser<'source> {
    lexer: Lexer<'source>,
    tree: SyntaxTree<'source>,
    stack: Vec<Entry>,
    lookahead: Option<Token>,
    limits: Limits,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source [u8], options: ParseOptions) -> Result<Self, Diagnostic> {
        let limits = options.limits;
        if limits.stack == 0 {
            return Err(error(ErrorKind::Limit, 0, "parser stack limit exceeded"));
        }
        let mut stack = Vec::new();
        reserve(&mut stack, 1)?;
        stack.push(Entry {
            state: 0,
            node: usize::MAX,
        });
        Ok(Self {
            lexer: Lexer::new(source, options),
            tree: SyntaxTree {
                source,
                nodes: Vec::new(),
                edges: Vec::new(),
                root: 0,
            },
            stack,
            lookahead: None,
            limits,
        })
    }

    pub fn run(mut self) -> Result<SyntaxTree<'source>, Diagnostic> {
        loop {
            let action = self.next_action()?;
            match action {
                32767 => {
                    self.tree.root = self.stack[1].node;
                    self.tree.nodes[self.tree.root].span = 0..self.tree.source.len();
                    return Ok(self.tree);
                }
                0 => return Err(self.unexpected()),
                1.. => self.shift((action - 1) as u16)?,
                _ => self.reduce((-action - 1) as usize)?,
            }
        }
    }

    fn next_action(&mut self) -> Result<i16, Diagnostic> {
        let state = usize::from(self.stack.last().unwrap().state);
        let entries = &ACTIONS[ACTION_OFFSETS[state]..ACTION_OFFSETS[state + 1]];
        if entries.is_empty() && DEFAULTS[state] != 0 {
            return Ok(DEFAULTS[state]);
        }
        if self.lookahead.is_none() {
            self.lookahead = Some(self.lexer.next()?);
        }
        let token = self.lookahead.as_ref().unwrap().id;
        Ok(entries
            .binary_search_by_key(&token, |&(id, _)| id)
            .map_or(DEFAULTS[state], |index| entries[index].1))
    }

    fn unexpected(&self) -> Diagnostic {
        let span = self
            .lookahead
            .as_ref()
            .map_or(self.lexer.position..self.lexer.position, |token| {
                token.span.clone()
            });
        Diagnostic {
            kind: ErrorKind::Syntax,
            span,
            message: "unexpected token in PHP syntax",
        }
    }

    fn shift(&mut self, state: u16) -> Result<(), Diagnostic> {
        let token = self.lookahead.take().unwrap();
        if token.id == 0 {
            return self.push(state, usize::MAX);
        }
        let name = TOKEN_NAMES
            .iter()
            .find(|&&(id, _)| id == token.id)
            .map_or(SYMBOL_NAMES[usize::from(token.id)], |&(_, name)| name);
        let node = self.add_node(SyntaxKind::Token(name), token.span, 0..0)?;
        self.push(state, node)
    }

    fn reduce(&mut self, rule: usize) -> Result<(), Diagnostic> {
        let (symbol, length) = RULES[rule];
        let start = self.stack.len() - length;
        let children = self.reduce_children(start)?;
        let span = self.reduce_span(children.clone());
        validate_rule(
            SYMBOL_NAMES[usize::from(symbol)],
            NodeView {
                source: self.tree.source,
                nodes: &self.tree.nodes,
                edges: &self.tree.edges,
            },
            children.clone(),
        )?;
        self.reduction_effects(symbol);
        self.stack.truncate(start);
        let node = self.add_node(
            SyntaxKind::Rule(SYMBOL_NAMES[usize::from(symbol)]),
            span,
            children,
        )?;
        let state = usize::from(self.stack.last().unwrap().state);
        let entries = &GOTOS[GOTO_OFFSETS[state]..GOTO_OFFSETS[state + 1]];
        let index = entries
            .binary_search_by_key(&symbol, |&(id, _)| id)
            .unwrap();
        self.push(entries[index].1, node)
    }

    fn reduce_children(&mut self, start: usize) -> Result<Range<usize>, Diagnostic> {
        let begin = self.tree.edges.len();
        reserve(&mut self.tree.edges, self.stack.len() - start)?;
        self.tree
            .edges
            .extend(self.stack[start..].iter().map(|entry| entry.node));
        Ok(begin..self.tree.edges.len())
    }

    fn reduce_span(&self, children: Range<usize>) -> Range<usize> {
        if children.is_empty() {
            let position = self
                .lookahead
                .as_ref()
                .map_or(self.lexer.position, |token| token.span.start);
            return position..position;
        }
        let first = self.tree.edges[children.start];
        let last = self.tree.edges[children.end - 1];
        self.tree.nodes[first].span.start..self.tree.nodes[last].span.end
    }

    fn reduction_effects(&mut self, symbol: u16) {
        if SYMBOL_NAMES[usize::from(symbol)] == "file_halt" {
            self.lexer.halt();
            self.lookahead = None;
        }
    }

    fn add_node(
        &mut self,
        kind: SyntaxKind,
        span: Range<usize>,
        children: Range<usize>,
    ) -> Result<usize, Diagnostic> {
        if self.tree.nodes.len() >= self.limits.nodes {
            return Err(error(
                ErrorKind::Limit,
                span.start,
                "syntax node limit exceeded",
            ));
        }
        reserve(&mut self.tree.nodes, 1)?;
        let index = self.tree.nodes.len();
        self.tree.nodes.push(SyntaxNode {
            kind,
            span,
            children,
        });
        Ok(index)
    }

    fn push(&mut self, state: u16, node: usize) -> Result<(), Diagnostic> {
        if self.stack.len() >= self.limits.stack {
            return Err(error(
                ErrorKind::Limit,
                self.lexer.position,
                "parser stack limit exceeded",
            ));
        }
        reserve(&mut self.stack, 1)?;
        self.stack.push(Entry { state, node });
        Ok(())
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    fn valid_action(action: i16) -> bool {
        match action {
            32767 | 0 => true,
            1.. => (action as usize) <= DEFAULTS.len(),
            _ => ((-action - 1) as usize) < RULES.len(),
        }
    }

    #[test]
    fn finite_table_bounds_phprs_013() {
        assert_eq!(DEFAULTS.len(), 1113);
        assert_eq!(RULES.len(), 634);
        assert_eq!(ACTION_OFFSETS.len(), DEFAULTS.len() + 1);
        assert_eq!(GOTO_OFFSETS.len(), DEFAULTS.len() + 1);
        assert_eq!(ACTION_OFFSETS.last(), Some(&ACTIONS.len()));
        assert_eq!(GOTO_OFFSETS.last(), Some(&GOTOS.len()));
        for &action in DEFAULTS {
            assert!(valid_action(action));
        }
        for &(_, action) in ACTIONS {
            assert!(valid_action(action));
        }
        for &(symbol, state) in GOTOS {
            assert!((symbol as usize) < SYMBOL_NAMES.len());
            assert!((state as usize) < DEFAULTS.len());
        }
    }

    fn visit(state: usize, seen: &mut [bool], pending: &mut Vec<usize>) {
        if !seen[state] {
            seen[state] = true;
            pending.push(state);
        }
    }

    #[test]
    fn automaton_graph_reachability_phprs_013() {
        let mut seen = vec![false; DEFAULTS.len()];
        let mut pending = Vec::new();
        visit(0, &mut seen, &mut pending);
        while let Some(state) = pending.pop() {
            for &(_, action) in &ACTIONS[ACTION_OFFSETS[state]..ACTION_OFFSETS[state + 1]] {
                if action > 0 && action != 32767 {
                    visit((action - 1) as usize, &mut seen, &mut pending);
                }
            }
            for &(_, target) in &GOTOS[GOTO_OFFSETS[state]..GOTO_OFFSETS[state + 1]] {
                visit(target as usize, &mut seen, &mut pending);
            }
        }
        assert!(seen.iter().all(|&reached| reached));
    }

    #[test]
    fn invalid_table_control_phprs_013() {
        assert!(!valid_action(2000));
        assert!(!valid_action(-1000));
    }
}
