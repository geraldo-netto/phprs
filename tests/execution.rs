// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

use phprs::contracts::{DEFAULT_OPTIONS, ErrorKind, ParseOptions};
use phprs::execution_contracts::{ExecutionOptions, SourceExecutor};
use phprs::{EchoExecutor, PhpParser};

fn execute(source: &[u8], limit: usize) -> Result<Vec<u8>, phprs::contracts::Diagnostic> {
    EchoExecutor { parser: &PhpParser }.execute(
        source,
        ExecutionOptions {
            parse: DEFAULT_OPTIONS,
            output_bytes: limit,
        },
    )
}

#[test]
fn hello_output_owns_its_bytes_phprs_020() {
    let output = {
        let source = include_bytes!("../examples/hello.php").to_vec();
        let parser = PhpParser;
        let executor = EchoExecutor { parser: &parser };
        let interface: &dyn SourceExecutor = &executor;
        interface
            .execute(
                &source,
                ExecutionOptions {
                    parse: DEFAULT_OPTIONS,
                    output_bytes: 14,
                },
            )
            .unwrap()
    };
    assert_eq!(output, b"Hello, world!\n");
}

#[test]
fn quoted_strings_and_simple_escapes_phprs_020() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"<?php echo '';", b""),
        (b"<?php echo \"\";", b""),
        (
            br#"<?php echo '\n\t\x41\u{41}\101\$\"';"#,
            br#"\n\t\x41\u{41}\101\$\""#,
        ),
        (br"<?php echo 'a\'b\\c';", br"a'b\c"),
        (
            br#"<?php echo "\n\r\t\v\e\f\\\$\"\q";"#,
            b"\n\r\t\x0b\x1b\x0c\\$\"\\q",
        ),
        (b"<?php EcHo /* comment */ b'raw\x00\xff';", b"raw\x00\xff"),
        (b"<?php echo B\"binary\";", b"binary"),
        (b"<?= 'short echo' ?>\n", b"short echo"),
        (b"<?php echo 'no semicolon' ?>", b"no semicolon"),
        (
            b"<?php echo 'line\nbreak'; // trailing comment",
            b"line\nbreak",
        ),
    ];
    for &(source, expected) in cases {
        assert_eq!(
            execute(source, expected.len()).unwrap(),
            expected,
            "{source:?}"
        );
    }
}

#[test]
fn reject_other_programs_before_output_phprs_020() {
    for source in [
        b"".as_slice(),
        b"<?php ;",
        b"<?php echo 1;",
        b"<?php echo 'x', 'y';",
        b"<?php echo 'x'; echo 'y';",
        b"<?php if (false) echo 'x';",
        b"<?php echo \"$x\";",
        b"html <?php echo 'x';",
        b"<?php echo 'x' . 'y';",
        b"<?php echo ('x');",
        br#"<?php echo "\x41";"#,
        br#"<?php echo "\u{41}";"#,
        br#"<?php echo "\101";"#,
    ] {
        assert_eq!(
            execute(source, 1024).unwrap_err().kind,
            ErrorKind::Unsupported,
            "{source:?}"
        );
    }
    assert_eq!(
        execute(b"<?php echo ;", 1024).unwrap_err().kind,
        ErrorKind::Syntax
    );
}

#[test]
fn output_limit_boundaries_phprs_020() {
    assert_eq!(execute(b"<?php echo '';", 0).unwrap(), b"");
    assert_eq!(
        execute(b"<?php echo 'x';", 0).unwrap_err().kind,
        ErrorKind::Limit
    );
    assert_eq!(
        execute(b"<?php echo 'xy';", 1).unwrap_err().kind,
        ErrorKind::Limit
    );
    assert_eq!(
        execute(br#"<?php echo "\q";"#, 1).unwrap_err().kind,
        ErrorKind::Limit
    );
    assert_eq!(
        execute(br#"<?php echo "\n";"#, 0).unwrap_err().kind,
        ErrorKind::Limit
    );
    assert_eq!(execute(br#"<?php echo "\n";"#, 1).unwrap(), b"\n");
    assert_eq!(execute(b"<?php echo 'xy';", usize::MAX).unwrap(), b"xy");
}

#[test]
fn execution_preserves_parse_options_and_limits_phprs_020() {
    let source = b"#!ignored\n<?php echo 'hello';";
    let mut options = ExecutionOptions {
        parse: DEFAULT_OPTIONS,
        output_bytes: 5,
    };
    assert_eq!(
        EchoExecutor { parser: &PhpParser }
            .execute(source, options)
            .unwrap_err()
            .kind,
        ErrorKind::Unsupported
    );
    options.parse.skip_shebang = true;
    assert_eq!(
        EchoExecutor { parser: &PhpParser }
            .execute(source, options)
            .unwrap(),
        b"hello"
    );
    options.parse.limits.source_bytes = source.len() - 1;
    assert_eq!(
        EchoExecutor { parser: &PhpParser }
            .execute(source, options)
            .unwrap_err()
            .kind,
        ErrorKind::Limit
    );
    options.parse = ParseOptions {
        short_open_tags: true,
        ..DEFAULT_OPTIONS
    };
    assert_eq!(
        EchoExecutor { parser: &PhpParser }
            .execute(b"<? echo 'hello';", options)
            .unwrap(),
        b"hello"
    );
}

#[test]
fn every_literal_byte_round_trips_phprs_020() {
    for byte in 0..=255 {
        let mut source = b"<?php echo '".to_vec();
        if matches!(byte, b'\'' | b'\\') {
            source.push(b'\\');
        }
        source.push(byte);
        source.extend_from_slice(b"';");
        assert_eq!(execute(&source, 1).unwrap(), [byte]);
    }
}

#[test]
fn bounded_invalid_bytes_and_truncations_phprs_020() {
    let hello = include_bytes!("../examples/hello.php");
    for length in 0..=hello.len() {
        let result = execute(&hello[..length], 14);
        if let Ok(output) = result {
            assert_eq!(output, b"Hello, world!\n");
        }
    }
    for byte in 0..=255 {
        let source = [b"<?php echo ".as_slice(), &[byte], b";"].concat();
        assert!(execute(&source, 14).is_err(), "{source:?}");
    }
    assert_eq!(execute(hello, 14).unwrap(), b"Hello, world!\n");
}
