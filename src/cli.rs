//! Native file and process adapter. Parsing itself has no host dependencies.
use phprs::{
    contracts::{DEFAULT_LIMITS, DEFAULT_OPTIONS},
    parse,
};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn run(
    args: impl Iterator<Item = OsString>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> u8 {
    let result = input_path(args).and_then(|path| check(path, stdout));
    match result {
        Ok(()) => 0,
        Err(message) => {
            let _ = writeln!(stderr, "phprs: {message}");
            1
        }
    }
}

fn input_path(mut args: impl Iterator<Item = OsString>) -> Result<PathBuf, String> {
    if args.next().as_deref() != Some(std::ffi::OsStr::new("--check")) {
        return Err(usage());
    }
    let path = args.next().ok_or_else(usage)?;
    if args.next().is_some() {
        return Err(usage());
    }
    Ok(PathBuf::from(path))
}

fn usage() -> String {
    "usage: phprs --check <file.php>".into()
}

fn check(path: PathBuf, stdout: &mut impl Write) -> Result<(), String> {
    let file =
        std::fs::File::open(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let source = read_source(file)?;
    parse(&source, DEFAULT_OPTIONS).map_err(|error| {
        format!(
            "{}: byte {}: {} ({:?})",
            path.display(),
            error.span.start,
            error.message,
            error.kind
        )
    })?;
    writeln!(stdout, "Syntax OK").map_err(|error| format!("stdout: {error}"))
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
            vec!["file.php"],
            vec!["--check"],
            vec!["--check", "a", "b"],
        ] {
            let mut stderr = Vec::new();
            assert_eq!(
                run(
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
            assert_eq!(run(args.into_iter(), &mut Vec::new(), &mut stderr), 1);
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
        assert_eq!(run(args.into_iter(), &mut Broken, &mut stderr), 1);
        assert!(String::from_utf8(stderr).unwrap().contains("stdout"));
        assert_eq!(run(std::iter::empty(), &mut Broken, &mut Broken), 1);
    }

    #[test]
    fn cli_success_checks_without_execution_phprs_018() {
        let args = [
            OsString::from("--check"),
            OsString::from("examples/hello.php"),
        ];
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        assert_eq!(run(args.into_iter(), &mut stdout, &mut stderr), 0);
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
            parse(&source, DEFAULT_OPTIONS).unwrap_err().kind,
            phprs::contracts::ErrorKind::Limit
        );
    }
}
