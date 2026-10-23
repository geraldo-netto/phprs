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

impl NodeView<'_> {
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
    if !matches!(rule, "prefix_mutation" | "reference_assignment") {
        return Ok(());
    }
    let operand = view
        .edges
        .get(children)
        .and_then(|indices| indices.last())
        .copied();
    if operand.is_some_and(|node| view.destination(node)) {
        return Ok(());
    }
    let span = operand
        .and_then(|node| view.nodes.get(node))
        .map_or(0..0, |node| node.span.clone());
    Err(Diagnostic {
        kind: ErrorKind::Syntax,
        span,
        message: "syntax requires a variable-like operand",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
