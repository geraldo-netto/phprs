//! Syntax-shape checks over independent tree contracts, without parser state.
use crate::contracts::{Diagnostic, ErrorKind, SyntaxKind, SyntaxNode};
use std::ops::Range;

/// An immutable borrow of the invocation-owned tree under construction.
/// Views never escape validation. Indices must decrease during wrapper traversal;
/// malformed wrapper paths reject, so traversal is bounded by node count.
pub(super) struct NodeView<'tree> {
    pub nodes: &'tree [SyntaxNode],
    pub edges: &'tree [usize],
}

struct ModifierStep {
    name: &'static str,
    next: Option<(Range<usize>, usize)>,
}

impl NodeView<'_> {
    fn conditional(&self, children: Range<usize>) -> Option<(usize, bool)> {
        let indices = self.edges.get(children)?;
        if !matches!(indices.len(), 4 | 5) {
            return None;
        }
        let operator = self.nodes.get(indices[1])?;
        (operator.kind == SyntaxKind::Token("'?'")).then_some((indices[0], indices.len() == 5))
    }

    fn token(&self, mut index: usize) -> Option<&'static str> {
        loop {
            if let SyntaxKind::Token(name) = self.nodes.get(index)?.kind {
                return Some(name);
            }
            index = self.one_child(index)?;
        }
    }

    fn next_modifier_prefix(
        &self,
        indices: &[usize],
        bound: usize,
    ) -> Result<Option<(Range<usize>, usize)>, ()> {
        if indices.len() != 2 {
            return Ok(None);
        }
        let prefix = indices[0];
        if prefix >= bound {
            return Err(());
        }
        let node = self.nodes.get(prefix).ok_or(())?;
        Ok(Some((node.children.clone(), prefix)))
    }

    fn modifier_step(
        &self,
        children: Range<usize>,
        bound: usize,
    ) -> Result<Option<ModifierStep>, ()> {
        let indices = self.edges.get(children).ok_or(())?;
        if indices.is_empty() {
            return Ok(None);
        }
        if indices.len() > 2 {
            return Err(());
        }
        let entry = indices[indices.len() - 1];
        if entry >= bound {
            return Err(());
        }
        let name = self.token(entry).ok_or(())?;
        let next = self.next_modifier_prefix(indices, bound)?;
        Ok(Some(ModifierStep { name, next }))
    }

    fn diagnostic(&self, children: Range<usize>, message: &'static str) -> Diagnostic {
        let node = self
            .edges
            .get(children)
            .and_then(|indices| indices.last())
            .copied();
        let span = node
            .and_then(|node| self.nodes.get(node))
            .map_or(0..0, |node| node.span.clone());
        Diagnostic {
            kind: ErrorKind::Syntax,
            span,
            message,
        }
    }

    fn destination(&self, mut index: usize) -> bool {
        while let Some(node) = self.nodes.get(index) {
            if node.kind == SyntaxKind::Rule("place") {
                return true;
            }
            if !matches!(
                node.kind,
                SyntaxKind::Rule("expression" | "postfix" | "atom")
            ) {
                return false;
            }
            let Some(child) = self.one_child(index) else {
                return false;
            };
            index = child;
        }
        false
    }

    fn one_child(&self, index: usize) -> Option<usize> {
        let node = self.nodes.get(index)?;
        let children = self.edges.get(node.children.clone())?;
        if children.len() != 1 {
            return None;
        }
        let child = children[0];
        (child < index).then_some(child)
    }
}

pub(super) fn validate_rule(
    rule: &str,
    view: NodeView<'_>,
    children: Range<usize>,
) -> Result<(), Diagnostic> {
    if rule == "expression" {
        return validate_conditional(&view, children);
    }
    if matches!(
        rule,
        "member_modifiers" | "class_modifiers" | "promotion_modifiers"
    ) {
        return validate_modifiers(&view, children);
    }
    if !matches!(rule, "prefix_mutation" | "reference_assignment") {
        return Ok(());
    }
    let operand = view
        .edges
        .get(children.clone())
        .and_then(|indices| indices.last())
        .copied();
    if operand.is_some_and(|node| view.destination(node)) {
        return Ok(());
    }
    Err(view.diagnostic(children, "syntax requires a variable-like operand"))
}

fn validate_conditional(view: &NodeView<'_>, children: Range<usize>) -> Result<(), Diagnostic> {
    let Some((left, full)) = view.conditional(children.clone()) else {
        return Ok(());
    };
    let nested = view.nodes.get(left).and_then(|node| {
        (node.kind == SyntaxKind::Rule("expression"))
            .then(|| view.conditional(node.children.clone()))
            .flatten()
    });
    if nested.is_some_and(|(_, inner_full)| full || inner_full) {
        return Err(view.diagnostic(children, "ambiguous ternary chain requires parentheses"));
    }
    Ok(())
}

fn incompatible(first: &str, second: &str) -> bool {
    if first == second {
        return true;
    }
    let exclusive: &[&[&str]] = &[
        &["T_PUBLIC", "T_PROTECTED", "T_PRIVATE"],
        &["T_PUBLIC_SET", "T_PROTECTED_SET", "T_PRIVATE_SET"],
        &["T_ABSTRACT", "T_FINAL"],
    ];
    exclusive
        .iter()
        .any(|group| group.contains(&first) && group.contains(&second))
}

fn validate_modifiers(view: &NodeView<'_>, children: Range<usize>) -> Result<(), Diagnostic> {
    // Ten declared modifier classes bound this call-owned scratch space.
    let mut seen: [Option<&str>; 10] = [None; 10];
    let mut count = 0;
    let mut next = Some((children.clone(), view.nodes.len()));
    while let Some((range, bound)) = next {
        let step = view
            .modifier_step(range, bound)
            .map_err(|()| view.diagnostic(children.clone(), "invalid modifier structure"))?;
        let Some(step) = step else { break };
        if seen[..count]
            .iter()
            .flatten()
            .any(|prior| incompatible(prior, step.name))
        {
            return Err(view.diagnostic(children, "conflicting declaration modifiers"));
        }
        if count == seen.len() {
            return Err(view.diagnostic(children, "modifier capacity exceeded"));
        }
        seen[count] = Some(step.name);
        count += 1;
        next = step.next;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modifier_chain(names: &[&'static str]) -> (Vec<SyntaxNode>, Vec<usize>, Range<usize>) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut previous = None;
        let mut children = 0..0;
        for name in names {
            let token = nodes.len();
            nodes.push(SyntaxNode {
                kind: SyntaxKind::Token(name),
                span: 0..1,
                children: edges.len()..edges.len(),
            });
            let start = edges.len();
            edges.extend(previous);
            edges.push(token);
            children = start..edges.len();
            previous = Some(nodes.len());
            nodes.push(SyntaxNode {
                kind: SyntaxKind::Rule("member_modifiers"),
                span: 0..1,
                children: children.clone(),
            });
        }
        (nodes, edges, children)
    }

    #[test]
    fn modifier_capacity_and_structure_phprs_057() {
        let names = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];
        for count in [0, 1, 10, 11] {
            let (nodes, edges, children) = modifier_chain(&names[..count]);
            let result = validate_rule(
                "member_modifiers",
                NodeView {
                    nodes: &nodes,
                    edges: &edges,
                },
                children,
            );
            assert_eq!(result.is_ok(), count <= 10);
        }
        for (children, edges) in [
            (0..1, vec![]),
            (0..3, vec![0, 0, 0]),
            (0..1, vec![usize::MAX]),
            (0..2, vec![1, 0]),
        ] {
            let nodes = [SyntaxNode {
                kind: SyntaxKind::Token("T_PUBLIC"),
                span: 0..1,
                children: 0..0,
            }];
            assert!(
                validate_rule(
                    "member_modifiers",
                    NodeView {
                        nodes: &nodes,
                        edges: &edges
                    },
                    children
                )
                .is_err()
            );
        }
    }

    #[test]
    fn malformed_modifier_wrappers_phprs_057() {
        let (mut nodes, edges, children) = modifier_chain(&["T_PUBLIC", "T_STATIC"]);
        nodes[2].kind = SyntaxKind::Rule("member_modifier");
        assert!(
            validate_rule(
                "member_modifiers",
                NodeView {
                    nodes: &nodes,
                    edges: &edges
                },
                children
            )
            .is_err()
        );
        let view = NodeView {
            nodes: &[],
            edges: &[],
        };
        assert!(view.next_modifier_prefix(&[0, 1], 1).is_err());
    }

    #[test]
    fn malformed_views_reject_without_cycles_phprs_097() {
        for (children, edges) in [
            (0..0, vec![]),
            (0..2, vec![0, 0]),
            (1..2, vec![0]),
            (0..1, vec![0]),
            (0..1, vec![usize::MAX]),
        ] {
            let nodes = [SyntaxNode {
                kind: SyntaxKind::Rule("expression"),
                span: 0..0,
                children,
            }];
            let view = NodeView {
                nodes: &nodes,
                edges: &edges,
            };
            assert!(!view.destination(0));
            assert!(!view.destination(usize::MAX));
            assert!(validate_rule("prefix_mutation", view, 0..edges.len()).is_err());
        }
        let view = NodeView {
            nodes: &[],
            edges: &[],
        };
        assert!(validate_rule("reference_assignment", view, 0..1).is_err());
    }
}
