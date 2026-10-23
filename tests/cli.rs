// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

#[test]
fn binary_runs_hello_world_phprs_030() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_phprs"))
        .arg("examples/hello.php")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"Hello, world!\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn binary_checks_without_execution_phprs_018() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_phprs"))
        .args(["--check", "examples/hello.php"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Syntax OK\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn binary_reports_syntax_errors_phprs_018() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_phprs"))
        .args(["--check", "tests/fixtures/invalid.php"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("byte 11")
    );
}

#[test]
fn binary_selects_file_preamble_mode_phprs_043() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_phprs"))
        .args(["--check", "tests/fixtures/shebang.php"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Syntax OK\n");
    assert!(output.stderr.is_empty());
}
