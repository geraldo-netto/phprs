// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

mod cli;

fn main() -> std::process::ExitCode {
    let parser = phprs::PhpParser;
    let status = cli::run(
        &parser,
        &phprs::EchoExecutor { parser: &parser },
        std::env::args_os().skip(1),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    std::process::ExitCode::from(status)
}
