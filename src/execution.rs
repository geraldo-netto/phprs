// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

//! One echo delivery: syntax adaptation and bounded literal evaluation.
use crate::contracts::{Diagnostic, ErrorKind, SyntaxKind, SyntaxParser, SyntaxTree};
use crate::execution_contracts::{ExecutionOptions, SourceExecutor};

/// Borrows the selected parser; each invocation owns and settles its resources.
pub struct EchoExecutor<'parser> {
    pub parser: &'parser dyn SyntaxParser,
}

impl SourceExecutor for EchoExecutor<'_> {
    fn execute(&self, source: &[u8], options: ExecutionOptions) -> Result<Vec<u8>, Diagnostic> {
        let tree = self.parser.parse(source, options.parse)?;
        decode_literal(echo_literal(&tree)?, options.output_bytes)
    }
}

fn unsupported() -> Diagnostic {
    Diagnostic {
        kind: ErrorKind::Unsupported,
        span: 0..0,
        message: "execution supports one echo of a quoted string with simple escapes",
    }
}

/// Adapt language token contracts without depending on parser table identities.
fn echo_literal<'source>(tree: &SyntaxTree<'source>) -> Result<&'source [u8], Diagnostic> {
    let mut tokens = tree
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, SyntaxKind::Token(_)));
    let (Some(first), Some(second), Some(third), None) =
        (tokens.next(), tokens.next(), tokens.next(), tokens.next())
    else {
        return Err(unsupported());
    };
    let mut statement = [first, second, third];
    statement.sort_by_key(|node| node.span.start);
    if statement.map(|node| &node.kind)
        != [
            &SyntaxKind::Token("T_ECHO"),
            &SyntaxKind::Token("T_CONSTANT_ENCAPSED_STRING"),
            &SyntaxKind::Token("';'"),
        ]
    {
        return Err(unsupported());
    }
    tree.source
        .get(statement[1].span.clone())
        .ok_or_else(unsupported)
}

fn literal_body(literal: &[u8]) -> Result<(u8, &[u8]), Diagnostic> {
    let literal = match literal.first() {
        Some(b'b' | b'B') => &literal[1..],
        _ => literal,
    };
    let Some((&quote, rest)) = literal.split_first() else {
        return Err(unsupported());
    };
    if !matches!(quote, b'\'' | b'"') {
        return Err(unsupported());
    }
    let body = rest.strip_suffix(&[quote]).ok_or_else(unsupported)?;
    Ok((quote, body))
}

fn output_buffer(capacity: usize) -> Result<Vec<u8>, Diagnostic> {
    let mut output = Vec::new();
    output.try_reserve_exact(capacity).map_err(|_| Diagnostic {
        kind: ErrorKind::Allocation,
        span: 0..0,
        message: "output allocation failed",
    })?;
    Ok(output)
}

fn append(output: &mut Vec<u8>, bytes: &[u8], limit: usize) -> Result<(), Diagnostic> {
    if bytes.len() > limit - output.len() {
        return Err(Diagnostic {
            kind: ErrorKind::Limit,
            span: 0..0,
            message: "output byte limit exceeded",
        });
    }
    // decode_literal reserves min(body length, limit); decoding never expands.
    output.extend_from_slice(bytes);
    Ok(())
}

fn decode_literal(literal: &[u8], limit: usize) -> Result<Vec<u8>, Diagnostic> {
    let (quote, body) = literal_body(literal)?;
    let mut output = output_buffer(body.len().min(limit))?;
    let mut bytes = body.iter().copied();
    while let Some(byte) = bytes.next() {
        if byte != b'\\' {
            append(&mut output, &[byte], limit)?;
            continue;
        }
        let escaped = bytes.next().ok_or_else(unsupported)?;
        match simple_escape(quote, escaped)? {
            Some(value) => append(&mut output, &[value], limit)?,
            None => append(&mut output, &[b'\\', escaped], limit)?,
        }
    }
    Ok(output)
}

fn simple_escape(quote: u8, byte: u8) -> Result<Option<u8>, Diagnostic> {
    if quote == b'\'' {
        return Ok(matches!(byte, b'\'' | b'\\').then_some(byte));
    }
    if matches!(byte, b'0'..=b'7' | b'x' | b'u') {
        return Err(unsupported());
    }
    const ESCAPES: &[(u8, u8)] = &[
        (b'n', b'\n'),
        (b'r', b'\r'),
        (b't', b'\t'),
        (b'v', 0x0b),
        (b'e', 0x1b),
        (b'f', 0x0c),
        (b'\\', b'\\'),
        (b'$', b'$'),
        (b'"', b'"'),
    ];
    Ok(ESCAPES
        .iter()
        .find(|&&(spelling, _)| spelling == byte)
        .map(|&(_, value)| value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::SyntaxNode;

    #[test]
    fn malformed_literal_and_allocation_boundaries_phprs_020() {
        for literal in [b"".as_slice(), b"b", b"B", b"x", b"'", b"\"", b"'unclosed"] {
            assert_eq!(
                decode_literal(literal, 64).unwrap_err().kind,
                ErrorKind::Unsupported
            );
        }
        assert_eq!(
            decode_literal(b"\"\\\"", 64).unwrap_err().kind,
            ErrorKind::Unsupported
        );
        assert_eq!(
            output_buffer(usize::MAX).unwrap_err().kind,
            ErrorKind::Allocation
        );
    }

    #[test]
    fn syntax_adapter_uses_spans_without_storage_order_phprs_020() {
        let mut tree = SyntaxTree {
            source: b"<?php echo 'x';",
            nodes: vec![
                SyntaxNode {
                    kind: SyntaxKind::Token("';'"),
                    span: 14..15,
                    children: 0..0,
                },
                SyntaxNode {
                    kind: SyntaxKind::Token("T_CONSTANT_ENCAPSED_STRING"),
                    span: 11..14,
                    children: 0..0,
                },
                SyntaxNode {
                    kind: SyntaxKind::Token("T_ECHO"),
                    span: 6..10,
                    children: 0..0,
                },
                SyntaxNode {
                    kind: SyntaxKind::Rule("start"),
                    span: 0..15,
                    children: 0..3,
                },
            ],
            edges: vec![2, 1, 0],
            root: 3,
        };
        assert_eq!(echo_literal(&tree).unwrap(), b"'x'");
        tree.nodes[1].span = 11..usize::MAX;
        assert_eq!(
            echo_literal(&tree).unwrap_err().kind,
            ErrorKind::Unsupported
        );
    }
}
