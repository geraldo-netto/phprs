mod cli;

fn main() -> std::process::ExitCode {
    let status = cli::run(
        &phprs::PhpParser,
        std::env::args_os().skip(1),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    std::process::ExitCode::from(status)
}
