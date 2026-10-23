mod lexer;
mod parser;
mod policy;
mod tables;
mod token_ids;

use crate::contracts::{Diagnostic, ErrorKind, ParseOptions, SyntaxParser, SyntaxTree};

/// Stateless implementation of the independent PHP 8.5 syntax contract.
pub struct PhpParser;

impl SyntaxParser for PhpParser {
    fn parse<'source>(
        &self,
        source: &'source [u8],
        options: ParseOptions,
    ) -> Result<SyntaxTree<'source>, Diagnostic> {
        if source.len() > options.limits.source_bytes {
            return Err(error(ErrorKind::Limit, 0, "source byte limit exceeded"));
        }
        parser::Parser::new(source, options)?.run()
    }
}

pub(super) fn error(kind: ErrorKind, offset: usize, message: &'static str) -> Diagnostic {
    Diagnostic {
        kind,
        span: offset..offset,
        message,
    }
}

pub(super) fn reserve<T>(values: &mut Vec<T>, additional: usize) -> Result<(), Diagnostic> {
    values
        .try_reserve(additional)
        .map_err(|_| error(ErrorKind::Allocation, 0, "allocation failed"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocation_failure_is_owned_phprs_002() {
        let mut values = Vec::<u8>::new();
        assert_eq!(
            reserve(&mut values, usize::MAX).unwrap_err().kind,
            ErrorKind::Allocation
        );
        assert!(values.is_empty());
        reserve(&mut values, 1).unwrap();
        values.push(7);
        assert_eq!(values, [7]);
    }
}
