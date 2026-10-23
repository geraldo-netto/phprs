//! Declaration-specific modifier contracts over the borrowed syntax view.
use super::NodeView;
use crate::contracts::{Diagnostic, SyntaxKind};
use std::ops::Range;

fn admitted_member(view: &NodeView<'_>, children: Range<usize>) -> Option<&'static [&'static str]> {
    let indices = view.edges.get(children)?;
    let declaration = view.nodes.get(*indices.get(2)?)?;
    match declaration.kind {
        SyntaxKind::Token("T_FUNCTION") => Some(&[
            "T_PUBLIC",
            "T_PROTECTED",
            "T_PRIVATE",
            "T_STATIC",
            "T_ABSTRACT",
            "T_FINAL",
        ]),
        SyntaxKind::Token("T_CONST") => Some(&["T_PUBLIC", "T_PROTECTED", "T_PRIVATE", "T_FINAL"]),
        _ => None,
    }
}

fn admitted(
    rule: &str,
    view: &NodeView<'_>,
    children: Range<usize>,
) -> Option<&'static [&'static str]> {
    if rule == "hook" {
        return Some(&["T_FINAL"]);
    }
    if rule == "parameter" {
        return Some(&[
            "T_PUBLIC",
            "T_PROTECTED",
            "T_PRIVATE",
            "T_PUBLIC_SET",
            "T_PROTECTED_SET",
            "T_PRIVATE_SET",
            "T_READONLY",
            "T_FINAL",
        ]);
    }
    admitted_member(view, children)
}

fn modifier_children(
    rule: &str,
    view: &NodeView<'_>,
    children: Range<usize>,
) -> Option<Range<usize>> {
    let indices = view.edges.get(children)?;
    let wrapper = *indices.get(1)?;
    if rule == "parameter" {
        return Some(view.nodes.get(wrapper)?.children.clone());
    }
    let modifiers = view.one_child(wrapper)?;
    Some(view.nodes.get(modifiers)?.children.clone())
}

fn check(view: &NodeView<'_>, children: Range<usize>, allowed: &[&str]) -> Result<(), Diagnostic> {
    let mut next = Some((children.clone(), view.nodes.len()));
    while let Some((range, bound)) = next {
        let step = view
            .modifier_step(range, bound)
            .map_err(|()| view.diagnostic(children.clone(), "invalid modifier structure"))?;
        let Some(step) = step else { break };
        if !allowed.contains(&step.name) {
            return Err(view.diagnostic(children, "modifier is invalid for this declaration"));
        }
        next = step.next;
    }
    Ok(())
}

fn missing_type(view: &NodeView<'_>, children: Range<usize>) -> bool {
    view.edges
        .get(children)
        .and_then(|indices| indices.get(2))
        .and_then(|&index| view.nodes.get(index))
        .is_some_and(|node| node.kind == SyntaxKind::Rule("type_opt") && node.children.is_empty())
}

fn validate_type(
    rule: &str,
    view: &NodeView<'_>,
    children: Range<usize>,
) -> Result<(), Diagnostic> {
    if !missing_type(view, children.clone()) {
        return Ok(());
    }
    let Some(modifiers) = modifier_children(rule, view, children) else {
        return Ok(());
    };
    check(
        view,
        modifiers,
        &[
            "T_PUBLIC",
            "T_PROTECTED",
            "T_PRIVATE",
            "T_STATIC",
            "T_ABSTRACT",
            "T_FINAL",
            "T_READONLY",
        ],
    )
}

pub(super) fn validate(
    rule: &str,
    view: &NodeView<'_>,
    children: Range<usize>,
) -> Result<(), Diagnostic> {
    validate_type(rule, view, children.clone())?;
    let Some(allowed) = admitted(rule, view, children.clone()) else {
        return Ok(());
    };
    let Some(modifiers) = modifier_children(rule, view, children) else {
        return Ok(());
    };
    check(view, modifiers, allowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_declaration_views_phprs_101() {
        let view = NodeView {
            source: b"",
            nodes: &[],
            edges: &[],
        };
        assert!(check(&view, 0..1, &[]).is_err());
        assert!(check(&view, 0..0, &[]).is_ok());
        assert!(validate("member", &view, 0..1).is_ok());
    }
}
