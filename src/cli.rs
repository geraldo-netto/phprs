//! Native file and process adapter for syntax checks and the selected execution.
use phprs::contracts::{DEFAULT_LIMITS, DEFAULT_OPTIONS, Diagnostic, ParseOptions, SyntaxParser};
use phprs::execution_contracts::{ExecutionOptions, SourceExecutor};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn run(
    parser: &dyn SyntaxParser,
    executor: &dyn SourceExecutor,
    args: impl Iterator<Item = OsString>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> u8 {
    let result = input_command(args).and_then(|command| process(parser, executor, command, stdout));
    match result {
        Ok(()) => 0,
        Err(message) => {
            let _ = writeln!(stderr, "phprs: {message}");
            1
        }
    }
}

enum Command {
    Check(PathBuf),
    Execute(PathBuf),
}

fn input_command(mut args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    let first = args.next().ok_or_else(usage)?;
    let command = if first == "--check" {
        Command::Check(PathBuf::from(args.next().ok_or_else(usage)?))
    } else if first.as_encoded_bytes().starts_with(b"-") {
        return Err(usage());
    } else {
        Command::Execute(PathBuf::from(first))
    };
    if args.next().is_some() {
        return Err(usage());
    }
    Ok(command)
}

fn usage() -> String {
    "usage: phprs [--check] <file.php>".into()
}

fn process(
    parser: &dyn SyntaxParser,
    executor: &dyn SourceExecutor,
    command: Command,
    stdout: &mut impl Write,
) -> Result<(), String> {
    let path = match &command {
        Command::Check(path) | Command::Execute(path) => path,
    };
    let file = std::fs::File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let source = read_source(file)?;
    let options = ParseOptions {
        skip_shebang: true,
        ..DEFAULT_OPTIONS
    };
    let result = match command {
        Command::Check(_) => {
            parser
                .parse(&source, options)
                .map_err(|error| diagnostic(path, error))?;
            stdout.write_all(b"Syntax OK\n")
        }
        Command::Execute(_) => {
            let output = executor
                .execute(
                    &source,
                    ExecutionOptions {
                        parse: options,
                        output_bytes: DEFAULT_LIMITS.source_bytes,
                    },
                )
                .map_err(|error| diagnostic(path, error))?;
            stdout.write_all(&output)
        }
    };
    result
        .and_then(|()| stdout.flush())
        .map_err(|error| format!("stdout: {error}"))
}

fn diagnostic(path: &std::path::Path, error: Diagnostic) -> String {
    format!(
        "{}: byte {}: {} ({:?})",
        path.display(),
        error.span.start,
        error.message,
        error.kind
    )
}

fn read_source(file: impl Read) -> Result<Vec<u8>, String> {
    let capacity = DEFAULT_LIMITS.source_bytes + 1;
    let mut source = source_buffer(capacity)?;
    file.take(capacity as u64)
        .read_to_end(&mut source)
        .map_err(|error| format!("read: {error}"))?;
    Ok(source)
}

fn source_buffer(capacity: usize) -> Result<Vec<u8>, String> {
    let mut source = Vec::new();
    source
        .try_reserve_exact(capacity)
        .map_err(|error| format!("source allocation: {error}"))?;
    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use phprs::PhpParser;
    use phprs::contracts::{Diagnostic, ErrorKind, ParseOptions, SyntaxTree};

    fn run(
        parser: &dyn SyntaxParser,
        args: impl Iterator<Item = OsString>,
        stdout: &mut impl Write,
        stderr: &mut impl Write,
    ) -> u8 {
        super::run(
            parser,
            &phprs::EchoExecutor { parser },
            args,
            stdout,
            stderr,
        )
    }

    struct InjectedFailure;

    impl SyntaxParser for InjectedFailure {
        fn parse<'source>(
            &self,
            source: &'source [u8],
            options: ParseOptions,
        ) -> Result<SyntaxTree<'source>, Diagnostic> {
            assert_eq!(source, include_bytes!("../examples/hello.php"));
            assert_eq!(options.limits.source_bytes, DEFAULT_LIMITS.source_bytes);
            assert_eq!(options.limits.tokens, DEFAULT_LIMITS.tokens);
            assert_eq!(options.limits.nodes, DEFAULT_LIMITS.nodes);
            assert_eq!(options.limits.stack, DEFAULT_LIMITS.stack);
            assert!(!options.short_open_tags);
            assert!(options.skip_shebang);
            Err(Diagnostic {
                kind: ErrorKind::Allocation,
                span: 0..0,
                message: "injected parser allocation failure",
            })
        }
    }

    #[test]
    fn cli_uses_injected_parser_phprs_037() {
        let args = [
            OsString::from("--check"),
            OsString::from("examples/hello.php"),
        ];
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        assert_eq!(
            run(&InjectedFailure, args.into_iter(), &mut stdout, &mut stderr),
            1
        );
        assert!(stdout.is_empty());
        assert_eq!(
            stderr,
            b"phprs: examples/hello.php: byte 0: injected parser allocation failure (Allocation)\n"
        );
    }

    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("injected write failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("injected read failure"))
        }
    }

    #[test]
    fn cli_arguments_phprs_018() {
        for args in [
            vec![],
            vec!["--unknown"],
            vec!["--check"],
            vec!["--check", "a", "b"],
            vec!["a", "b"],
        ] {
            let mut stderr = Vec::new();
            assert_eq!(
                run(
                    &PhpParser,
                    args.into_iter().map(OsString::from),
                    &mut Vec::new(),
                    &mut stderr
                ),
                1
            );
            assert!(stderr.starts_with(b"phprs: usage:"));
        }
    }

    #[test]
    fn cli_file_and_parse_errors_phprs_018() {
        for path in ["tests/fixtures/missing.php", "tests/fixtures/invalid.php"] {
            let args = [OsString::from("--check"), OsString::from(path)];
            let mut stderr = Vec::new();
            assert_eq!(
                run(&PhpParser, args.into_iter(), &mut Vec::new(), &mut stderr),
                1
            );
            assert!(stderr.starts_with(b"phprs:"));
        }
    }

    #[test]
    fn cli_output_failure_phprs_018() {
        let args = [
            OsString::from("--check"),
            OsString::from("examples/hello.php"),
        ];
        let mut stderr = Vec::new();
        assert_eq!(
            run(&PhpParser, args.into_iter(), &mut Broken, &mut stderr),
            1
        );
        assert!(String::from_utf8(stderr).unwrap().contains("stdout"));
        assert_eq!(
            run(&PhpParser, std::iter::empty(), &mut Broken, &mut Broken),
            1
        );
    }

    #[test]
    fn cli_success_checks_without_execution_phprs_018() {
        let args = [
            OsString::from("--check"),
            OsString::from("examples/hello.php"),
        ];
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        assert_eq!(
            run(&PhpParser, args.into_iter(), &mut stdout, &mut stderr),
            0
        );
        assert_eq!(stdout, b"Syntax OK\n");
        assert!(stderr.is_empty());
    }

    #[test]
    fn read_fault_and_bound_phprs_018() {
        assert!(read_source(Broken).unwrap_err().contains("read:"));
        assert!(
            source_buffer(usize::MAX)
                .unwrap_err()
                .contains("source allocation:")
        );
        assert!(source_buffer(0).unwrap().is_empty());
        let source = read_source(std::io::repeat(b' ')).unwrap();
        assert_eq!(source.len(), DEFAULT_LIMITS.source_bytes + 1);
        assert_eq!(
            PhpParser.parse(&source, DEFAULT_OPTIONS).unwrap_err().kind,
            phprs::contracts::ErrorKind::Limit
        );
    }

    struct FlushFailure;

    impl Write for FlushFailure {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Err(std::io::Error::other("injected flush failure"))
        }
    }

    #[test]
    fn cli_execution_flush_failure_phprs_025() {
        let mut stderr = Vec::new();
        assert_eq!(
            run(
                &PhpParser,
                [OsString::from("examples/hello.php")].into_iter(),
                &mut FlushFailure,
                &mut stderr,
            ),
            1
        );
        assert_eq!(stderr, b"phprs: stdout: injected flush failure\n");
    }

    #[test]
    fn cli_execution_file_and_syntax_errors_phprs_025() {
        for path in ["tests/fixtures/missing.php", "tests/fixtures/invalid.php"] {
            let mut stdout = Vec::new();
            let mut stderr = Vec::new();
            assert_eq!(
                run(
                    &PhpParser,
                    [OsString::from(path)].into_iter(),
                    &mut stdout,
                    &mut stderr
                ),
                1
            );
            assert!(stdout.is_empty());
            assert!(stderr.starts_with(b"phprs:"));
        }
    }

    #[test]
    fn cli_execution_write_failure_phprs_025() {
        let mut stderr = Vec::new();
        assert_eq!(
            run(
                &PhpParser,
                [OsString::from("examples/hello.php")].into_iter(),
                &mut Broken,
                &mut stderr
            ),
            1
        );
        assert_eq!(stderr, b"phprs: stdout: injected write failure\n");
    }

    struct InjectedExecutor;

    impl SourceExecutor for InjectedExecutor {
        fn execute(&self, source: &[u8], options: ExecutionOptions) -> Result<Vec<u8>, Diagnostic> {
            assert_eq!(source, include_bytes!("../examples/hello.php"));
            assert!(options.parse.skip_shebang);
            assert_eq!(options.output_bytes, DEFAULT_LIMITS.source_bytes);
            Ok(vec![0, 255])
        }
    }

    #[test]
    fn cli_uses_execution_interface_phprs_025() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        assert_eq!(
            super::run(
                &InjectedFailure,
                &InjectedExecutor,
                [OsString::from("examples/hello.php")].into_iter(),
                &mut stdout,
                &mut stderr
            ),
            0
        );
        assert_eq!(stdout, [0, 255]);
        assert!(stderr.is_empty());
    }
}
