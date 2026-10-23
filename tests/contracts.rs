// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

//! The consumer needs only contracts; no concrete parser or generated symbol IDs.
use phprs::contracts::{
    DEFAULT_OPTIONS, Diagnostic, ParseOptions, SyntaxKind, SyntaxNode, SyntaxParser, SyntaxTree,
};

struct EmptySourceParser;

impl SyntaxParser for EmptySourceParser {
    fn parse<'source>(
        &self,
        source: &'source [u8],
        _: ParseOptions,
    ) -> Result<SyntaxTree<'source>, Diagnostic> {
        assert!(
            source.is_empty(),
            "this test double admits only empty input"
        );
        Ok(SyntaxTree {
            source,
            nodes: vec![SyntaxNode {
                kind: SyntaxKind::Rule("start"),
                span: 0..0,
                children: 0..0,
            }],
            edges: Vec::new(),
            root: 0,
        })
    }
}

#[test]
fn result_borrows_source_independently_of_parser_phprs_037() {
    let source = Vec::<u8>::new();
    let tree = {
        let parser = EmptySourceParser;
        let interface: &dyn SyntaxParser = &parser;
        interface.parse(&source, DEFAULT_OPTIONS).unwrap()
    };
    assert_eq!(tree.source.as_ptr(), source.as_ptr());
    assert_eq!(tree.nodes.len(), 1);
    assert_eq!(tree.nodes[tree.root].kind, SyntaxKind::Rule("start"));
    assert_eq!(tree.nodes[tree.root].span, 0..0);
    assert!(tree.edges.is_empty());
}
