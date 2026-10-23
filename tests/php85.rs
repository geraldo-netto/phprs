//! PHP 8.5 fixtures are specified against the pinned grammar, not the PHP 8.3 oracle.
mod common;
use phprs::{
    PhpParser,
    contracts::{DEFAULT_OPTIONS, SyntaxKind, SyntaxParser},
};

#[test]
fn php85_grammar_baseline_phprs_044() {
    let sources: &[&[u8]] = &[
        b"<?php $x = 'text' |> strlen(...);",
        b"<?php $x = new C()->method();",
        b"<?php $x = clone($object, ['x' => 1]);",
        b"<?php #[A] const X = 1;",
        b"<?php class C { public string $name { get => $this->name; } }",
    ];
    for &source in sources {
        let tree = PhpParser
            .parse(source, DEFAULT_OPTIONS)
            .unwrap_or_else(|error| panic!("{source:?}: {error:?}"));
        common::verify_tree(&tree);
    }
}

#[test]
fn asymmetric_visibility_phprs_045() {
    for (modifier, token) in [
        ("public(set)", "T_PUBLIC_SET"),
        ("protected(set)", "T_PROTECTED_SET"),
        ("private(set)", "T_PRIVATE_SET"),
        ("PrIvAtE(SeT)", "T_PRIVATE_SET"),
    ] {
        let sources = [
            format!("<?php class C {{ {modifier} int $x; }}"),
            format!("<?php class C {{ public {modifier} string $x = ''; }}"),
            format!("<?php class C {{ function __construct(public {modifier} int $x) {{}} }}"),
        ];
        for source in sources {
            let tree = PhpParser
                .parse(source.as_bytes(), DEFAULT_OPTIONS)
                .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            common::verify_tree(&tree);
            assert!(
                tree.nodes
                    .iter()
                    .any(|node| node.kind == SyntaxKind::Token(token)
                        && &source.as_bytes()[node.span.clone()] == modifier.as_bytes())
            );
        }
    }
}

#[test]
fn asymmetric_visibility_boundaries_phprs_045() {
    for suffix in [
        "( set)",
        "(set )",
        " (set)",
        "(setting)",
        "(set",
        "(\nset)",
        "(set\0)",
    ] {
        let source = format!("<?php class C {{ private{suffix} int $x; }}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
    for source in [
        "<?php $o->private(set);",
        "<?php $o?->protected(set);",
        "<?php A\\public(set);",
        "<?php \\private(set);",
        "<?php class C { private(set)int $x; }",
    ] {
        common::verify_tree(&PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap());
    }
}
