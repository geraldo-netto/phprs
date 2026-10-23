//! Independent PHP 8.5 fixtures follow the language contract, not a PHP 8.3 oracle.
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

#[test]
fn void_cast_positions_phprs_046() {
    for source in [
        "<?php (void) f();",
        "<?php ( VoId\t) $x + f();",
        "<?php for ((void) start(); $ready; (void) step()) {}",
        "<?php for ((void) start(), 1; (void) check(), $ready; (void) step(), (void) finish()) {}",
        "<?php if ($ready) (void) work();",
    ] {
        let tree = PhpParser
            .parse(source.as_bytes(), DEFAULT_OPTIONS)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        common::verify_tree(&tree);
        assert!(
            tree.nodes
                .iter()
                .any(|node| node.kind == SyntaxKind::Token("T_VOID_CAST"))
        );
    }
}

#[test]
fn void_cast_invalid_values_phprs_046() {
    for source in [
        "<?php echo (void) f();",
        "<?php $x = (void) f();",
        "<?php return (void) f();",
        "<?php $f = fn() => (void) f();",
        "<?php if ((void) f()) {}",
        "<?php for (; (void) f(); ) {}",
        "<?php for (; $ready, (void) f(); ) {}",
        "<?php f((void) g());",
        "<?php (void) (void) f();",
        "<?php (void);",
        "<?php (\nvoid) f();",
        "<?php (void\r) f();",
        "<?php (vo id) f();",
        "<?php (void\0) f();",
    ] {
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

fn binary_group(source: &str, operator: &str, left: &str, right: &str) {
    let input = format!("<?php {source};");
    let tree = PhpParser.parse(input.as_bytes(), DEFAULT_OPTIONS).unwrap();
    common::verify_tree(&tree);
    let grouped = format!("{left}{operator}{right}");
    for node in tree.nodes.iter().filter(|node| node.children.len() == 3) {
        if input[node.span.clone()] != grouped {
            continue;
        }
        let children = &tree.edges[node.children.clone()];
        assert_eq!(&input[tree.nodes[children[0]].span.clone()], left);
        assert_eq!(&input[tree.nodes[children[1]].span.clone()], operator);
        assert_eq!(&input[tree.nodes[children[2]].span.clone()], right);
        return;
    }
    panic!("missing grouping {grouped:?} in {source:?}");
}

#[test]
fn independent_operator_grouping_phprs_093() {
    for (source, operator, left, right) in [
        ("1+2*3", "+", "1", "2*3"),
        ("1*2+3", "+", "1*2", "3"),
        ("2**3**4", "**", "2", "3**4"),
        ("-2**2", "**", "2", "2"),
        ("$a=$b=$c", "=", "$a", "$b=$c"),
        ("$a??$b??$c", "??", "$a", "$b??$c"),
        ("$a.$b<<2", ".", "$a", "$b<<2"),
        ("$a.$b|>f(...)", "|>", "$a.$b", "f(...)"),
        ("$a|>f(...)|>g(...)", "|>", "$a|>f(...)", "g(...)"),
        ("$a&$b==$c", "&", "$a", "$b==$c"),
    ] {
        binary_group(source, operator, left, right);
    }
}

#[test]
fn property_comments_phprs_048_phprs_017() {
    for access in ["->", "?->"] {
        for comment in ["/**/", "/* comment */", "// comment\n", "# comment\r\n"] {
            for name in ["match", "class", "private", "function", "__PROPERTY__"] {
                let source = format!("<?php echo $o{access} {comment} {name};");
                let tree = PhpParser
                    .parse(source.as_bytes(), DEFAULT_OPTIONS)
                    .unwrap_or_else(|error| panic!("{source}: {error:?}"));
                common::verify_tree(&tree);
                assert!(
                    tree.nodes
                        .iter()
                        .any(|node| node.kind == SyntaxKind::Token("T_STRING")
                            && &source[node.span.clone()] == name)
                );
            }
        }
    }
}

#[test]
fn property_comment_boundaries_phprs_048() {
    for source in [
        "<?php echo $o-> /* unterminated",
        "<?php echo $o?-> /* unterminated",
        "<?php echo $o-> // comment\n ;",
        "<?php echo $o-> # comment\n ;",
    ] {
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
    for source in [
        "<?php echo $o-> /* c */ {$name};",
        "<?php echo \"{$o-> /* c */ match}\";",
        "<?php echo \"$o-> /* c */ match\";",
        "<?php echo $o-> /* c */ f()->x;",
    ] {
        common::verify_tree(&PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap());
    }
}

#[test]
fn setter_modifier_conflicts_phprs_057() {
    for modifiers in [
        "public(set) private(set)",
        "protected(set) private(set)",
        "private(set) private(set)",
    ] {
        let sources = [
            format!("<?php class C {{ {modifiers} int $x; }}"),
            format!("<?php class C {{ function __construct(public {modifiers} int $x) {{}} }}"),
        ];
        for source in sources {
            assert!(
                PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
                "{source}"
            );
        }
    }
}
