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

#[test]
fn pipe_forms_phprs_047() {
    for expression in [
        "$x |> f(...)",
        "$x |> f(...) |> C::g(...)",
        "$x |> $callback",
        "$x |> 'strlen'",
        "$x |> [$object, 'method']",
        "$x |> new Invokable()",
        "$x |> ($flag ? f(...) : g(...))",
        "$x |> (fn($value) => $value + 1)",
        "$x |> (static fn($value): int => $value)",
        "$x |> function($value) { return $value; }",
        "$x |> 42", // Callable validation requires runtime values.
    ] {
        let source = format!("<?php {expression};");
        let tree = PhpParser
            .parse(source.as_bytes(), DEFAULT_OPTIONS)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        common::verify_tree(&tree);
        for end in 0..source.len() {
            if let Ok(tree) = PhpParser.parse(&source.as_bytes()[..end], DEFAULT_OPTIONS) {
                common::verify_tree(&tree);
            }
        }
    }
}

#[test]
fn pipe_boundaries_phprs_047() {
    for expression in [
        "$x |> fn($value) => $value",
        "$x |> static fn($value) => $value",
        "$x |> #[A] fn($value) => $value",
        "$x |>",
        "|> f(...)",
        "$x |> |> f(...)",
        "$x | > f(...)",
        "$x |> f(...,)",
        "$x |> (fn($value) =>)",
    ] {
        let source = format!("<?php {expression};");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

fn verify_sources_and_prefixes(sources: &[&str]) {
    for source in sources {
        let tree = PhpParser
            .parse(source.as_bytes(), DEFAULT_OPTIONS)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        common::verify_tree(&tree);
        for end in 0..source.len() {
            if let Ok(tree) = PhpParser.parse(&source.as_bytes()[..end], DEFAULT_OPTIONS) {
                common::verify_tree(&tree);
            }
        }
    }
}

#[test]
fn clone_exit_call_forms_phprs_054() {
    verify_sources_and_prefixes(&[
        "<?php clone $o; clone($o); clone(); clone($o,);",
        "<?php $x = clone($o, ['a' => 1],);",
        "<?php $x = clone(object: $o, withProperties: ['a' => 1]);",
        "<?php $x = clone(...$args); $f = clone(...);",
        "<?php $x = clone($o,)->property; $x = clone($o, [])?->method();",
        "<?php $x = (clone $o)->property; $x = clone($o)->property;",
        "<?php exit; die; exit(); die(); exit(1,);",
        "<?php exit(status: 1); die(status: 'done',);",
        "<?php exit(...$args); die(...$args); $f = exit(...); $f = die(...);",
        "<?php exit(1, 2); clone($o, [], 3);", // Arity is checked outside syntax.
    ]);
}

#[test]
fn clone_exit_malformed_phprs_054() {
    for expression in [
        "clone",
        "clone(,)",
        "clone($o,,)",
        "clone(object:)",
        "clone(...,)",
        "clone(...$args,,)",
        "exit(,)",
        "exit(status:)",
        "exit(...,)",
        "die(1,,)",
        "exit 1",
        "die 'text'",
    ] {
        let source = format!("<?php {expression};");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn direct_construction_access_phprs_055() {
    verify_sources_and_prefixes(&[
        "<?php new C()->method(); new C()->property; new C()?->property;",
        "<?php new C()[0]; new C()(); new C()::X; new C()::$property;",
        "<?php new $class(1)->method(); new ($factory())()->property;",
        "<?php new $classes[0]()->method(); new $factory->className()->method();",
        "<?php new C::$class()->method();",
        "<?php new class { function __invoke() {} }();",
        "<?php new class(1) extends C { public $x; }->x;",
        "<?php (new C)->method(); new C; new $class;",
    ]);
}

#[test]
fn construction_boundaries_phprs_055() {
    for expression in [
        "new C->p",
        "new C?->p",
        "new C[0]",
        "new C::X",
        "new C->f()",
        "new C[0]()",
        "new C::class()",
        "new C()->",
        "new C()[",
        "new C()?->",
        "new ()",
        "new C(,)",
        "new class( {}",
        "new C()::",
    ] {
        let source = format!("<?php {expression};");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn attributed_constants_phprs_056() {
    verify_sources_and_prefixes(&[
        "<?php #[A] const X = 1;",
        "<?php #[A(1), B(name: 'x'),] #[\\C] const X = 1, Y = 2;",
        "<?php namespace N; #[A] const X = (int) 1.2;",
        "<?php namespace N { #[A] const X = 1; } namespace { #[B] const Y = 2; }",
        "<?php #[A] const X = static fn() => 1;",
        "<?php class C { #[A] public const int X = 1, Y = 2; }",
    ]);
}

#[test]
fn attributed_constant_boundaries_phprs_056() {
    for source in [
        "<?php #[A] const;",
        "<?php #[A] const X;",
        "<?php #[A] const X =;",
        "<?php #[A] const X = 1,;",
        "<?php const #[A] X = 1;",
        "<?php #[A] const X = 1, #[B] Y = 2;",
        "<?php #[A] define('X', 1);",
        "<?php #[A] echo 1;",
        "<?php #[A const X = 1;",
        "<?php #[] const X = 1;",
    ] {
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn property_hook_forms_phprs_049() {
    verify_sources_and_prefixes(&[
        "<?php class C { public $value {} }",
        "<?php class C { public string $value = 'x' { get { return $this->value; } set { $this->value = $value; } } }",
        "<?php class C { protected string $value { get => $this->value; set => trim($value); } }",
        "<?php class C { public string $value { set(string|Stringable $incoming) { $this->value = (string)$incoming; } } }",
        "<?php class C { public array $value { &get { return $this->value; } } }",
        "<?php class C { public array $value { &get => $this->value; } }",
        "<?php abstract class C { abstract public string $value { get; set; } }",
        "<?php interface I { public string $value { get; } }",
        "<?php class C { #[A] public string $value { #[B] final get => 'x'; #[C] set => $value; } }",
        "<?php class C { function __construct(public private(set) string $value { get => $this->value; set(string $v) => $v; }) {} }",
        "<?php class C { public string $value { get => parent::$value::get(); set { parent::$value::set($value); } } }",
        "<?php class C { public string $value { get { $name = __PROPERTY__; return $this->$name; } } }",
    ]);
}

#[test]
fn property_hook_boundaries_phprs_049() {
    for source in [
        "<?php class C { public $x { get =>; } }",
        "<?php class C { public $x { get => 1 } }",
        "<?php class C { public $x { get() } }",
        "<?php class C { public $x { set($value,) =>; } }",
        "<?php class C { public $x { set(,) {} } }",
        "<?php class C { public $x, $y { get => 1; } }",
        "<?php class C { public $x { get {} }; }",
        "<?php class C { public $x { & => 1; } }",
        "<?php class C { public $x { get: 1; } }",
        "<?php class C { public $x { get => 1; set { } }",
    ] {
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn property_magic_token_phprs_049() {
    for name in ["__PROPERTY__", "__property__", "__PrOpErTy__"] {
        let source = format!("<?php class C {{ public $x {{ get => {name}; }} }}");
        let tree = PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap();
        common::verify_tree(&tree);
        assert!(
            tree.nodes
                .iter()
                .any(|node| node.kind == SyntaxKind::Token("T_PROPERTY_C")
                    && &source[node.span.clone()] == name)
        );
    }
}

#[test]
fn hook_name_rejections_phprs_104() {
    for hooks in [
        "get; get;",
        "get; &get;",
        "&get; get;",
        "set; set;",
        "get => 1; get => 2;",
        "fetch => 1;",
        "getter;",
    ] {
        let source = format!("<?php abstract class C {{ public $x {{ {hooks} }} }}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn setter_modifiers_require_properties_phprs_101() {
    for modifier in ["public(set)", "protected(set)", "private(set)"] {
        for member in ["function f() {}", "const X=1;"] {
            let source = format!("<?php class C {{ {modifier} {member} }}");
            assert!(
                PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
                "{source}"
            );
        }
    }
}
