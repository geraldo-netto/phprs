//! PHP 8.5 fixtures are specified against the pinned grammar, not the PHP 8.3 oracle.
mod common;
use phprs::{
    PhpParser,
    contracts::{DEFAULT_OPTIONS, SyntaxParser},
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
