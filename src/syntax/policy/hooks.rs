//! Hook-name policy borrows source spans; it owns no resources or retained state.
use super::NodeView;
use crate::contracts::Diagnostic;
use std::ops::Range;

fn hook_kind(view: &NodeView<'_>, hook: usize) -> Option<usize> {
    let node = view.nodes.get(hook)?;
    let children = view.edges.get(node.children.clone())?;
    let name = view.nodes.get(*children.get(3)?)?;
    let spelling = view.source.get(name.span.clone())?;
    if spelling.eq_ignore_ascii_case(b"get") {
        return Some(0);
    }
    spelling.eq_ignore_ascii_case(b"set").then_some(1)
}

fn step(
    view: &NodeView<'_>,
    children: Range<usize>,
    bound: usize,
) -> Result<Option<(usize, Range<usize>, usize)>, ()> {
    let indices = view.edges.get(children).ok_or(())?;
    if indices.is_empty() {
        return Ok(None);
    }
    if indices.len() != 2 {
        return Err(());
    }
    let hook = indices[1];
    if hook >= bound {
        return Err(());
    }
    let kind = hook_kind(view, hook).ok_or(())?;
    let (prefix, bound) = view.next_prefix(indices, bound)?.ok_or(())?;
    Ok(Some((kind, prefix, bound)))
}

pub(super) fn validate(view: &NodeView<'_>, children: Range<usize>) -> Result<(), Diagnostic> {
    // There are two hook kinds. Prefix traversal strictly decreases node identity.
    let mut seen = [false; 2];
    let mut next = (children.clone(), view.nodes.len());
    while let Some((kind, prefix, bound)) = step(view, next.0, next.1).map_err(|()| {
        view.diagnostic(children.clone(), "invalid property hook name or structure")
    })? {
        if seen[kind] {
            return Err(view.diagnostic(children, "duplicate property hook"));
        }
        seen[kind] = true;
        next = (prefix, bound);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{SyntaxKind, SyntaxNode};

    #[test]
    fn malformed_hook_views_phprs_104() {
        let nodes = [
            SyntaxNode {
                kind: SyntaxKind::Token("T_STRING"),
                span: 0..3,
                children: 0..0,
            },
            SyntaxNode {
                kind: SyntaxKind::Rule("hook"),
                span: 0..3,
                children: 0..4,
            },
        ];
        for (range, edges) in [
            (0..1, vec![]),
            (0..1, vec![0]),
            (0..3, vec![0, 0, 0]),
            (0..2, vec![0, usize::MAX]),
            (0..2, vec![0, 0]),
            (4..6, vec![0, 0, 0, 0, usize::MAX, 1]),
        ] {
            let view = NodeView {
                source: b"get",
                nodes: &nodes,
                edges: &edges,
            };
            assert!(validate(&view, range).is_err());
        }
        let view = NodeView {
            source: b"",
            nodes: &nodes,
            edges: &[0, 0, 0, 0],
        };
        assert_eq!(hook_kind(&view, 1), None);
        assert_eq!(hook_kind(&view, usize::MAX), None);
    }
}
