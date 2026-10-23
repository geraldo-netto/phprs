use phprs::{
    PhpParser,
    contracts::{
        DEFAULT_LIMITS, DEFAULT_OPTIONS, ErrorKind, ParseOptions, SyntaxKind, SyntaxNode,
        SyntaxParser,
    },
};

const VALID: &[(&str, &[u8])] = &[
    ("empty", b""),
    ("html", b"<html>outside PHP</html>"),
    ("tag", b"<?php"),
    ("hello", include_bytes!("../examples/hello.php")),
    ("tags", b"one <?= 1 ?> two <?PHP echo 2 ?> three <?php ;"),
    ("comments", b"#!/usr/bin/php\n<?php // one\n# two\n/** three */ echo 1; // ?>html"),
    ("literals", b"<?php $x=[0,123,1_000,012,0o77,0xCA_FE,0B101,1.,.25,1.2e-3,1E+3,9223372036854775808];"),
    ("strings", br#"<?php echo '', 'a\'b', "", b'binary', B"binary", "a\"b", "escaped\$name";"#),
    ("operators", b"<?php $a=1+2*3**4; $a+=1; $a-=1; $a*=2; $a/=2; $a%=2; $a**=2; $a.=1; $a<<=1; $a>>=1; $a&=1; $a|=1; $a^=1; $a??=1; ++$a; $a--; $b=~$a; $c=!$b; $x=$a<>$b; $x=$a<=>$b;"),
    ("precedence", b"<?php $x = 1 | 2 ^ 3 & 4; $x = $a && $b || $c; $x=$a and $b xor $c or $d; $x=$a??$b; $x=$a?$b:$c; $x=$a?:$c; $x=@foo();"),
    ("comparison", b"<?php $x=$a==$b; $x=$a!=$b; $x=$a===$b; $x=$a!==$b; $x=$a<$b; $x=$a<=$b; $x=$a>$b; $x=$a>=$b; $x=$a<<2; $x=$a>>2;"),
    ("control", b"<?php if($a){echo 1;}elseif($b)echo 2;else echo 3; while($a){break;} do{continue;}while($a); for($i=0;$i<2;$i++){echo $i;} foreach($a as $k=>&$v){} switch($a){case 1:break;default:;} goto label; label:;"),
    ("alternative", b"<?php if(1):echo 1;elseif(2):echo 2;else:echo 3;endif; while(0):endwhile; for(;;):break;endfor; foreach([] as $v):endforeach; switch(1):case 1:break;default:;endswitch; declare(ticks=1):echo 1;enddeclare;"),
    ("functions", b"<?php function &f(?int &$x, string ...$args): int|string { global $a, ${$b}; static $n=0; return $x; } $f = static function($x) use (&$a,$b) { return $x; }; $g=static fn($x):int=>$x+1; f(x:1); f(...[1]); $f=f(...);"),
    ("exceptions", b"<?php try { throw new Exception(); } catch (RuntimeException|LogicException $e) {} catch (Throwable) {} finally {}"),
    ("namespaces", b"<?php namespace A\\B; use C\\D; use function C\\f; use const C\\X; use C\\{D, E as F, function f, const X}; const X=1,Y=2; function f(){return \\C\\f()+namespace\\X;}"),
    ("namespace_blocks", b"<?php namespace A { function f(){} } namespace { A\\f(); }"),
    ("objects", b"<?php #[A(1)] readonly class C extends B implements I,J { public const string X='x'; public function __construct(public int $x=1) {} final public function f():static { return $this; } } interface I extends J { public function f(); } trait T { protected int $x; }"),
    ("trait_adaptations", b"<?php class C { use A,B { A::f insteadof B; B::f as protected alias; f as private; } var $x=1,$y=2; private static $z; }"),
    ("enums", b"<?php enum Suit:string implements I { case Hearts='h'; public function f(){} } enum Plain { case A; } function enum(){} enum();"),
    ("types", b"<?php function f((A&B)|C $a, A&B $b): (A&B)|null {} abstract class C { abstract protected function f(): never; }"),
    ("anonymous", b"<?php $x=new class(1) extends C implements I { public function f(){} }; $x?->f(); $x->class; C::class; C::$p; $x->{$name}; ${$name}=1; $$name=2;"),
    ("arrays", b"<?php $a=array(1,'x'=>&$v,...$b); [$x,,$y]=$a; list($x,list($y))=$a; $a[0]=1; unset($a[0],$b,); isset($a,$b,); empty($a);"),
    ("match", b"<?php $x=match($v){1,2=>'a',default=>'b',};"),
    ("generators", b"<?php function f(){yield; yield 1; yield 'x'=>2; yield from /* c */ $a;}"),
    ("builtins", b"<?php include 'a'; include_once 'b'; require 'c'; require_once 'd'; eval(''); print 1; exit; die(); $x=(int)$a; $x=(integer)$a; $x=(float)$a; $x=(double)$a; $x=(string)$a; $x=(binary)$a; $x=(bool)$a; $x=(boolean)$a; $x=(array)$a; $x=(object)$a;"),
    ("interpolation", br#"<?php echo "$name $a[key] $a[0] $a[-1] $a[$i] $o->class $o?->x {$a['key']} ${name} ${name[0]} ${$name}";"#),
    ("nested_interpolation", br#"<?php echo "{$o->f("{$x}")}"; echo `echo $name`; echo ``;"#),
    ("heredoc", b"<?php $s=<<<TEXT\nhello $name {$o->p}\nTEXT;\n$s=<<<'RAW'\n$no_interpolation\nRAW;\n"),
    ("heredoc_indent", b"<?php $s=<<<\"TEXT\"\r\n  hello\r\n  TEXT;\n$s=<<<EMPTY\nEMPTY;"),
    ("binary_heredoc", b"<?php $s=b<<<END\nbody\nEND; $s=B<<<'RAW'\nbody\nRAW;"),
    ("magic", b"<?php echo __LINE__,__FILE__,__DIR__,__CLASS__,__TRAIT__,__METHOD__,__FUNCTION__,__NAMESPACE__;"),
    ("halt", b"<?php __halt_compiler(); arbitrary\x00\xff payload '"),
    ("binary_bytes", b"<?php $\xff='\x00\xfe'; echo $\xff;"),
];

const INVALID: &[&[u8]] = &[
    b"<?php echo;",
    b"<?php echo 'x'",
    b"<?php echo 'unclosed;",
    b"<?php echo \"unclosed;",
    b"<?php /* unclosed",
    b"<?php $x = ;",
    b"<?php if () {}",
    b"<?php function f( {}",
    b"<?php class {}",
    b"<?php echo 0x;",
    b"<?php echo 08;",
    b"<?php echo 1__2;",
    b"<?php echo 1._2;",
    b"<?php echo 1e+;",
    b"<?php echo \x00;",
    b"<?php }",
    b"<?php {",
    b"<?php echo \"${\";",
    b"<?php echo \"$a[ ]\";",
    b"<?php echo \"$a[foo bar]\";",
    b"<?php $s=<<<EOF\nmissing;",
    b"<?php $s=<<<'EOF\nEOF;",
    b"<?php $s=<<<EOF invalid\nEOF;",
    b"<?php $s=<<<EOF\nless\n  EOF;",
    b"<?php $s=<<<EOF\n \tmixed\n \tEOF;",
    b"<?php $x=(real)1;",
    b"<?php function f(){__halt_compiler();}",
    b"<?php $a=[1,2;",
    b"<?php match(1){default=>};",
    b"<?php echo 1 2;",
];

mod common;
use common::verify_tree;

#[test]
fn php83_language_families_phprs_010() {
    let mut failures = Vec::new();
    for &(name, source) in VALID {
        match PhpParser.parse(source, DEFAULT_OPTIONS) {
            Ok(tree) => verify_tree(&tree),
            Err(error) => failures.push(format!("{name}: {error:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn malformed_syntax_phprs_012() {
    for &source in INVALID {
        assert!(
            PhpParser.parse(source, DEFAULT_OPTIONS).is_err(),
            "accepted {source:?}"
        );
    }
}

#[test]
fn limits_phprs_002() {
    let source = b"<?php echo 'hello';";
    let mut limits = DEFAULT_LIMITS;
    limits.source_bytes = source.len() - 1;
    assert_eq!(
        PhpParser
            .parse(
                source,
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits = DEFAULT_LIMITS;
    limits.tokens = 0;
    assert_eq!(
        PhpParser
            .parse(
                source,
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits = DEFAULT_LIMITS;
    limits.nodes = 0;
    assert_eq!(
        PhpParser
            .parse(
                b"",
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits = DEFAULT_LIMITS;
    limits.stack = 0;
    assert_eq!(
        PhpParser
            .parse(
                b"",
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits.stack = 1;
    assert_eq!(
        PhpParser
            .parse(
                source,
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits.stack = 3;
    assert_eq!(
        PhpParser
            .parse(
                b"<?php {{{{{{",
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    limits = DEFAULT_LIMITS;
    limits.source_bytes = source.len();
    verify_tree(
        &PhpParser
            .parse(
                source,
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                },
            )
            .unwrap(),
    );
}

#[test]
fn every_fixture_truncation_is_safe_phprs_032() {
    for &(_, source) in VALID {
        for end in 0..=source.len() {
            if let Ok(tree) = PhpParser.parse(&source[..end], DEFAULT_OPTIONS) {
                verify_tree(&tree);
            }
        }
    }
}

#[test]
fn bounded_byte_inputs_are_safe_phprs_032() {
    let mut seed = 0x5eed_u64;
    for length in 0..128 {
        for _ in 0..32 {
            let mut source = b"<?php ".to_vec();
            for _ in 0..length {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                source.push((seed >> 32) as u8);
            }
            if let Ok(tree) = PhpParser.parse(&source, DEFAULT_OPTIONS) {
                verify_tree(&tree);
            }
        }
    }
}

struct OracleChild(std::process::Child);

impl Drop for OracleChild {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(Some(_))) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

fn php_accepts(source: &[u8]) -> bool {
    php_accepts_with_args(
        source,
        &[
            "-r",
            "try { token_get_all(stream_get_contents(STDIN), TOKEN_PARSE); } catch (Throwable $e) { exit(1); }",
        ],
    )
}

fn php_accepts_with_args(source: &[u8], args: &[&str]) -> bool {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let child = Command::new("php")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("local PHP oracle required");
    let mut owned = OracleChild(child);
    owned.0.stdin.take().unwrap().write_all(source).unwrap();
    owned.0.wait().unwrap().success()
}

#[test]
fn differential_fixtures_phprs_032() {
    for &(name, source) in VALID {
        assert!(php_accepts(source), "oracle rejected {name}");
    }
    for &source in INVALID {
        assert!(!php_accepts(source), "oracle accepted {source:?}");
    }
}

#[test]
fn sparse_symbol_numbers_phprs_014() {
    let tree = PhpParser.parse(b"", DEFAULT_OPTIONS).unwrap();
    assert_eq!(tree.nodes[tree.root].kind, SyntaxKind::Rule("start"));
    assert!(
        PhpParser
            .parse(b"<?php function f(){__halt_compiler();}", DEFAULT_OPTIONS)
            .is_err()
    );
}

#[test]
fn compound_ampersand_phprs_015() {
    assert!(PhpParser.parse(b"<?php $a &= 1;", DEFAULT_OPTIONS).is_ok());
}

#[test]
fn nested_string_modes_phprs_016() {
    assert!(
        PhpParser
            .parse(br#"<?php echo "{$o->f("{$x}")}";"#, DEFAULT_OPTIONS)
            .is_ok()
    );
}

#[test]
fn lexer_edge_cases_phprs_017() {
    let expressions: &[&[u8]] = &[
        br#""\u{}""#,
        br#""\u{110000}""#,
        br#""\u{ffffffffffff}""#,
        br#""\u{41}""#,
        br#""\u{d800}""#,
        br#""\u{nothex}""#,
        br#""\u{41""#,
        br#""\u41""#,
        br#"'\u{}'"#,
        br#""$a[0x10]""#,
        br#""$a[0b10]""#,
        br#""$a[0o10]""#,
        br#""$a[1_0]""#,
        br#""$a[01]""#,
        br#""$a[-0x1]""#,
        br#""$a[0x]""#,
        br#""$a[1__0]""#,
        br#""$a[1.0]""#,
        b"1_000",
        b"0_7",
        b"0_8",
        b"0xFF_FF",
        b"0b10_10",
        b"0o7_7",
        b"0x_1",
        b".1_2",
        b"._1",
        b"1._0",
        b"1.0_1",
        b"1.e2",
        b"1e1_0",
        b"0xFFFFFFFFFFFFFFFF",
        b"0x10000000000000000",
        b"18446744073709551616",
        b"$o->match",
        // Property-comment behavior is covered by the PHP 8.5 fixture PHPRS-048/017.
        b"$o->\nmatch",
        b"$o?->match",
        b"C::class",
        b"C::{foo()}",
        b"enum()",
        b"new enum",
        b"enum::X",
        b"( real )1",
        b"(\nint)1",
        // The same unset-cast input is retained in PHPRS-113 with PHP 8 lint rejection.
        b"(bool)1",
        b"( int )1",
        br#"`\u{110000}`"#,
        br#""${foo()}""#,
        br#""{$o->{'x'}}""#,
    ];
    let mut failures = Vec::new();
    for &expression in expressions {
        let source = [b"<?php echo ".as_slice(), expression, b";"].concat();
        let expected = php_accepts(&source);
        let actual = PhpParser.parse(&source, DEFAULT_OPTIONS).is_ok();
        if actual != expected {
            failures.push((expression, expected, actual));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn tree_conservation_and_spans_phprs_019() {
    for &(_, source) in VALID {
        let tree = PhpParser.parse(source, DEFAULT_OPTIONS).unwrap();
        verify_tree(&tree);
    }
}

#[test]
#[should_panic(expected = "unowned or multiply owned tree nodes")]
fn tree_checker_rejects_orphans_phprs_042() {
    let mut tree = PhpParser.parse(b"<?php echo 1;", DEFAULT_OPTIONS).unwrap();
    tree.nodes.push(SyntaxNode {
        kind: SyntaxKind::Rule("unowned"),
        span: 0..0,
        children: 0..0,
    });
    verify_tree(&tree);
}

#[test]
#[should_panic(expected = "parent")]
fn tree_checker_rejects_escaping_spans_phprs_042() {
    let mut tree = PhpParser.parse(b"<?php echo 1;", DEFAULT_OPTIONS).unwrap();
    let token = tree
        .nodes
        .iter()
        .position(|node| node.kind == SyntaxKind::Token("T_LNUMBER"))
        .unwrap();
    tree.nodes[token].span = 0..tree.source.len();
    verify_tree(&tree);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn tree_checker_rejects_unused_edges_phprs_042() {
    let mut tree = PhpParser.parse(b"<?php echo 1;", DEFAULT_OPTIONS).unwrap();
    tree.edges.push(0);
    verify_tree(&tree);
}

#[test]
fn short_tag_configuration_phprs_012() {
    let options = ParseOptions {
        short_open_tags: true,
        ..DEFAULT_OPTIONS
    };
    let source = b"<? echo 1; ?>html <?php echo 2;";
    let tree = PhpParser.parse(source, options).unwrap();
    verify_tree(&tree);
    assert!(
        tree.nodes
            .iter()
            .any(|node| node.kind == SyntaxKind::Token("T_ECHO"))
    );
    assert!(PhpParser.parse(b"<? echo ;", DEFAULT_OPTIONS).is_ok());
    assert!(PhpParser.parse(b"<? echo ;", options).is_err());
    assert!(PhpParser.parse(b"<?phpX;", options).is_ok());
    assert!(PhpParser.parse(b"<?=1?>", options).is_ok());
}

#[test]
fn deep_input_uses_bounded_heap_stacks_phprs_002() {
    let mut source = b"<?php echo ".to_vec();
    source.extend(std::iter::repeat_n(b'(', 4096));
    source.extend_from_slice(b"1");
    source.extend(std::iter::repeat_n(b')', 4096));
    source.push(b';');
    verify_tree(&PhpParser.parse(&source, DEFAULT_OPTIONS).unwrap());
    let limits = phprs::contracts::Limits {
        stack: 32,
        ..DEFAULT_LIMITS
    };
    assert_eq!(
        PhpParser
            .parse(
                &source,
                ParseOptions {
                    limits,
                    ..DEFAULT_OPTIONS
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
}

#[test]
fn heredoc_interpolation_boundaries_phprs_023() {
    let sources: &[&[u8]] = &[
        b"<?php $s=<<<OUT\n{$o->f(\"\nOUT\n\")}\nOUT;",
        b"<?php $s=<<<OUT\n  {$o->f(\"\nunindented\n\")}\n  OUT;",
        b"<?php $s=<<<OUT\n{$o->f(<<<OUT\ninner\nOUT\n)}\nOUT;",
    ];
    for &source in sources {
        assert!(php_accepts(source), "invalid independent fixture");
        assert!(
            PhpParser.parse(source, DEFAULT_OPTIONS).is_ok(),
            "{source:?}"
        );
    }
}

fn mutate_fixture(source: &[u8], step: usize) -> Vec<u8> {
    let mut changed = source.to_vec();
    let index = (step * 31 + source.len() / 2) % (source.len() + 1);
    let byte = b"};($\0\"\\"[step % 7];
    if index == source.len() {
        changed.push(byte);
        return changed;
    }
    match step % 3 {
        0 => {
            changed[index] = byte;
        }
        1 => {
            changed.remove(index);
        }
        _ => {
            changed.insert(index, byte);
        }
    }
    changed
}

#[test]
fn mutated_fixtures_match_php_phprs_032() {
    let mut failures = Vec::new();
    for &(name, source) in VALID {
        for step in 0..9 {
            let changed = mutate_fixture(source, step);
            let expected = php_accepts(&changed);
            let actual = PhpParser.parse(&changed, DEFAULT_OPTIONS).is_ok();
            if actual != expected {
                failures.push((name, step, changed, expected, actual));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn enum_keyword_prefixes_phprs_039() {
    let names: &[(&[u8], bool)] = &[
        (b"extendsA", false),
        (b"implementsA", false),
        (b"extends_", false),
        (b"implements_", false),
        (b"ExTeNdSa", false),
        (b"IMPLEMENTSX", false),
        (b"extends\xff", false),
        (b"implements\xff", false),
        (b"extends", false),
        (b"implements", false),
        (b"extend", true),
        (b"implement", true),
        (b"extendsxSuffix", false),
        (b"extendX", true),
        (b"implementX", true),
        (b"Xextends", true),
        (b"Ximplements", true),
        (b"\\extendsA", false),
        (b"X", true),
    ];
    let separators: &[&[u8]] = &[b" ", b"\t", b"\r\n", b"/**/", b"/*x*/", b"//x\n", b"#x\n"];
    for &(name, expected) in names {
        for &separator in separators {
            let source = [b"<?php enum".as_slice(), separator, name, b" {}"].concat();
            assert_eq!(php_accepts(&source), expected, "oracle: {source:?}");
            assert_eq!(
                PhpParser.parse(&source, DEFAULT_OPTIONS).is_ok(),
                expected,
                "{source:?}"
            );
        }
    }
}

#[test]
fn enum_keyword_prefix_truncations_phprs_039() {
    for name in [b"extends".as_slice(), b"implements"] {
        for end in 0..name.len() {
            let source = [b"<?php enum ".as_slice(), &name[..end], b" {}"].concat();
            let expected = php_accepts(&source);
            assert_eq!(
                PhpParser.parse(&source, DEFAULT_OPTIONS).is_ok(),
                expected,
                "{source:?}"
            );
        }
    }
}

#[test]
fn raw_source_keeps_shebang_text_phprs_043() {
    let sources: &[&[u8]] = &[
        b"#!<?php echo ;\n",
        b"#!<?=;\n",
        b"#!ignore <?php echo ;\n",
        b"#!ignore <?php echo ;",
        b"#!ignore\r<?php echo ;",
    ];
    for &source in sources {
        assert!(!php_accepts(source), "oracle: {source:?}");
        assert!(
            PhpParser.parse(source, DEFAULT_OPTIONS).is_err(),
            "{source:?}"
        );
    }
}

#[test]
fn shebang_file_mode_phprs_043() {
    let cases: &[(&[u8], bool)] = &[
        (b"#!<?php echo ;\n", true),
        (b"#!<?=;\n", true),
        (b"#!ignore <?php echo ;", true),
        (b"#!ignore\r<?php echo ;", true),
        (b"#!ignore\n<?php echo 1;", true),
        (b"#!ignore\r\n<?php echo 1;", true),
        (b"#!ignore\n<?php echo ;", false),
        (b"#!ignore\r\n<?php echo ;", false),
        (b" #!<?php echo ;\n", false),
        (b"#!ignore\n#!<?php echo ;\n", false),
        (b"<?php echo ;", false),
        (b"", true),
    ];
    let options = ParseOptions {
        skip_shebang: true,
        ..DEFAULT_OPTIONS
    };
    for &(source, expected) in cases {
        assert_eq!(
            php_accepts_with_args(source, &["-n", "-l"]),
            expected,
            "file oracle: {source:?}"
        );
        assert_eq!(
            PhpParser.parse(source, options).is_ok(),
            expected,
            "{source:?}"
        );
    }
}

#[test]
fn shebang_spans_and_bounds_phprs_043() {
    let source = b"#!runner\n<?php echo 1;";
    let raw = PhpParser.parse(source, DEFAULT_OPTIONS).unwrap();
    let html = raw
        .nodes
        .iter()
        .find(|node| node.kind == SyntaxKind::Token("T_INLINE_HTML"))
        .unwrap();
    assert_eq!(html.span, 0..9);
    let mut options = ParseOptions {
        skip_shebang: true,
        ..DEFAULT_OPTIONS
    };
    let file = PhpParser.parse(source, options).unwrap();
    assert!(
        !file
            .nodes
            .iter()
            .any(|node| node.kind == SyntaxKind::Token("T_INLINE_HTML"))
    );
    verify_tree(&raw);
    verify_tree(&file);
    options.limits.source_bytes = source.len() - 1;
    assert_eq!(
        PhpParser.parse(source, options).unwrap_err().kind,
        ErrorKind::Limit
    );
    options.limits = DEFAULT_LIMITS;
    options.limits.tokens = 0;
    verify_tree(&PhpParser.parse(b"#!ignored", options).unwrap());
    options.skip_shebang = false;
    assert_eq!(
        PhpParser.parse(b"#!ignored", options).unwrap_err().kind,
        ErrorKind::Limit
    );
}

#[test]
fn syntax_destination_rejections_phprs_097() {
    for source in [
        "<?php ++1;",
        "<?php --true;",
        "<?php ++[1];",
        "<?php ++($x);",
        "<?php $x = &1;",
        "<?php foreach([] as 1) {}",
        "<?php foreach([] as 1=>$v) {}",
        "<?php unset(1);",
        "<?php C::X = 2;",
        "<?php named = 2;",
    ] {
        assert!(!php_accepts(source.as_bytes()), "oracle: {source}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn syntax_destination_call_forms_phprs_097() {
    for source in [
        "<?php ++f();",
        "<?php f()=1;",
        "<?php unset(f());",
        "<?php foreach([] as f()){}",
        "<?php $x=&f();",
        "<?php $x=&$y+1;",
        "<?php $x=&f()+1;",
        "<?php ++$x+1;",
        "<?php $x?->p=1;",
        "<?php $x[0]=1;",
        "<?php C::$x=2;",
    ] {
        assert!(php_accepts(source.as_bytes()), "oracle: {source}");
        common::verify_tree(&PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap());
    }
}

#[test]
fn modifier_conflicts_phprs_057() {
    for modifiers in [
        "public private",
        "public protected",
        "protected private",
        "public public",
        "static static",
        "abstract final",
        "final abstract",
        "readonly readonly",
        "final final",
        "abstract abstract",
    ] {
        for body in ["$x;", "function f();"] {
            let source = format!("<?php abstract class C {{ {modifiers} {body} }}");
            assert!(!php_accepts(source.as_bytes()), "oracle: {source}");
            assert!(
                PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
                "{source}"
            );
        }
    }
    for modifiers in [
        "abstract final",
        "final abstract",
        "abstract abstract",
        "final final",
        "readonly readonly",
    ] {
        let source = format!("<?php {modifiers} class C {{}}");
        assert!(!php_accepts(source.as_bytes()), "oracle: {source}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

#[test]
fn distinct_modifiers_remain_admitted_phprs_057() {
    for source in [
        "<?php abstract readonly class C {}",
        "<?php final readonly class C {}",
        "<?php class C { public static $x; }",
        "<?php abstract class C { abstract protected function f(); }",
        "<?php class C { final protected function f() {} }",
        "<?php class C { function __construct(public readonly int $x) {} }",
    ] {
        assert!(php_accepts(source.as_bytes()), "oracle: {source}");
        common::verify_tree(&PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap());
    }
}

#[test]
fn ternary_chains_phprs_103() {
    for (expression, accepted) in [
        ("$a?:$b?:$c", true),
        ("$a?$b?:$c:$d", true),
        ("$a?$b?$c:$d:$e", true),
        ("($a?$b:$c)?$d:$e", true),
        ("$a?$b:($c?$d:$e)", true),
        ("$a?:($b?$c:$d)", true),
        ("($a?$b:$c)?:$d", true),
        ("$a?$b:$c?$d:$e", false),
        ("$a?:$b?$c:$d", false),
        ("$a?$b:$c?:$d", false),
        ("$a?:", false),
        ("$a?$b:", false),
    ] {
        let source = format!("<?php {expression};");
        assert_eq!(
            php_accepts_with_args(source.as_bytes(), &["-n", "-l"]),
            accepted,
            "{source}"
        );
        let result = PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS);
        assert_eq!(result.is_ok(), accepted, "{source}");
        if let Ok(tree) = result {
            common::verify_tree(&tree);
        }
    }
}

#[test]
fn dynamic_class_targets_phprs_055() {
    for (expression, accepted) in [
        ("new $factory->className()", true),
        ("new $classes[0]()", true),
        ("new C::$class", true),
        ("new $object::$class[0]", true),
        ("new ($factory())", true),
        ("$x instanceof C::$class", true),
        ("new C->p", false),
        ("new C[0]", false),
        ("new C::X", false),
        ("new $object::X", false),
        ("new ($factory())[0]", false),
    ] {
        let source = format!("<?php {expression};");
        assert_eq!(php_accepts(source.as_bytes()), accepted, "{source}");
        let result = PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS);
        assert_eq!(result.is_ok(), accepted, "{source}");
        if let Ok(tree) = result {
            common::verify_tree(&tree);
        }
    }
}

#[test]
fn declaration_modifier_applicability_phprs_101() {
    for member in [
        "readonly function f() {}",
        "static const X=1;",
        "abstract const X=1;",
        "readonly const X=1;",
        "public static const int X=1;",
        "protected abstract const X=1;",
    ] {
        let source = format!("<?php class C {{ {member} }}");
        assert!(!php_accepts(source.as_bytes()), "{source}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
    for member in [
        "public static function f() {}",
        "final protected function f() {}",
        "final public const X=1;",
        "protected const int X=1;",
    ] {
        let source = format!("<?php class C {{ {member} }}");
        assert!(php_accepts(source.as_bytes()), "{source}");
        common::verify_tree(&PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).unwrap());
    }
}

#[test]
fn late_static_class_targets_phprs_108() {
    for (expression, accepted) in [
        ("new static", true),
        ("new static()", true),
        ("new static::$class", true),
        ("new static::$classes[0]()", true),
        ("$x instanceof static", true),
        ("$x instanceof static::$class", true),
        ("new static->p", false),
        ("new static[0]", false),
        ("new static::X", false),
    ] {
        let source = format!("<?php {expression};");
        assert_eq!(php_accepts(source.as_bytes()), accepted, "{source}");
        let result = PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS);
        assert_eq!(result.is_ok(), accepted, "{source}");
        if let Ok(tree) = result {
            common::verify_tree(&tree);
        }
    }
}

#[test]
fn removed_casts_rejected_phprs_113() {
    for expression in ["(unset)1", "( UnSeT\t) $value", "(real)$value", "(REAL)1"] {
        let source = format!("<?php {expression};");
        assert!(
            !php_accepts_with_args(source.as_bytes(), &["-n", "-l"]),
            "{source}"
        );
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}

fn observed_token_trace(source: &[u8]) -> String {
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    let program = concat!(
        "$p=0;foreach(token_get_all(stream_get_contents(STDIN)) as $t){",
        "if(is_array($t)){$n=token_name($t[0]);$s=$t[1];}else{$n=\"'\".$t.\"'\";$s=$t;}",
        "$e=$p+strlen($s);",
        "if(!in_array($n,['T_OPEN_TAG','T_WHITESPACE','T_COMMENT','T_DOC_COMMENT'],true)){",
        "$n=match($n){'T_OPEN_TAG_WITH_ECHO'=>'T_ECHO','T_CLOSE_TAG'=>\"';'\",default=>$n};",
        "echo $n,\"\\t\",$p,\"\\t\",$e,\"\\n\";}$p=$e;}"
    );
    let child = Command::new("php")
        .args(["-r", program])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut owned = OracleChild(child);
    owned.0.stdin.take().unwrap().write_all(source).unwrap();
    let mut trace = String::new();
    owned
        .0
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut trace)
        .unwrap();
    assert!(owned.0.wait().unwrap().success());
    trace
}

fn assert_token_trace(source: &[u8]) {
    use std::fmt::Write;
    let tree = PhpParser
        .parse(source, DEFAULT_OPTIONS)
        .unwrap_or_else(|error| panic!("{source:?}: {error:?}"));
    common::verify_tree(&tree);
    let mut tokens: Vec<_> = tree
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, SyntaxKind::Token(_)))
        .collect();
    tokens.sort_by_key(|node| node.span.start);
    let mut actual = String::new();
    for node in tokens {
        if let SyntaxKind::Token(name) = node.kind {
            writeln!(actual, "{name}\t{}\t{}", node.span.start, node.span.end).unwrap();
        }
    }
    assert_eq!(actual, observed_token_trace(source), "{source:?}");
}

#[test]
fn word_operator_traces_phprs_109() {
    for &(name, source) in VALID {
        if [
            "operators",
            "precedence",
            "comparison",
            "types",
            "functions",
            "namespaces",
            "objects",
            "enums",
            "builtins",
            "magic",
        ]
        .contains(&name)
        {
            assert_token_trace(source);
        }
    }
    for source in [
        "<?php NaMeSpAcE A; FuNcTiOn F() { ReTuRn \\A\\f(); }",
        "<?php $obj->class; $obj?->private; $obj->match; C::class;",
        "<?php enum E {case A;} function enum(){} enum();",
        "<?php function f(A&B $v) { $x=$a & $b; $x=& /* c */ $v; yield /* c */ from $v; }",
        "<?php $a=(integer)1; $b=(boolean)1; $c=(double)1; $d=(binary)1;",
    ] {
        assert_token_trace(source.as_bytes());
    }
}

#[test]
fn numeric_token_boundaries_phprs_110() {
    for literal in [
        "0",
        "00",
        "0_7",
        "0o7_7",
        "0O77",
        "0b10_01",
        "0B1",
        "0xCA_FE",
        "0XFF",
        "1.",
        ".1",
        "1.e2",
        "1.0e+2",
        "1_000.2_5E-1_0",
        "0e8",
        "09.1",
        "09e2",
        "9223372036854775807",
        "9223372036854775808",
        "18446744073709551615",
        "18446744073709551616",
        "0x7fff_ffff_ffff_ffff",
        "0x8000_0000_0000_0000",
        "0777777777777777777777",
        "01000000000000000000000",
        "1e9999",
        "1e-9999",
    ] {
        let source = format!("<?php echo {literal};");
        assert_token_trace(source.as_bytes());
    }
    for digits in 1..=70 {
        let source = format!("<?php echo 0b{};", "1".repeat(digits));
        assert_token_trace(source.as_bytes());
    }
}

#[test]
fn numeric_invalid_boundaries_phprs_110() {
    for literal in [
        "08", "09", "0_8", "0o8", "0b2", "0xG", "0x", "0o", "0b", "0x_1", "0b_1", "0o_1", "1__2",
        "1_", "1e", "1e+", "1e_2", "1e+_2", "1._2", ".1_", "1e2_", "0x1_", "0o7_", "0b1_",
    ] {
        let source = format!("<?php echo {literal};");
        assert!(!php_accepts(source.as_bytes()), "{source}");
        assert!(
            PhpParser.parse(source.as_bytes(), DEFAULT_OPTIONS).is_err(),
            "{source}"
        );
    }
}
