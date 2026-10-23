//! Independent syntax contracts. All offsets are byte offsets into the source.
use std::ops::Range;

/// Limits apply per invocation, including unsuccessful parsing.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub source_bytes: usize,
    pub tokens: usize,
    pub nodes: usize,
    pub stack: usize,
}

pub const DEFAULT_LIMITS: Limits = Limits {
    source_bytes: 1_048_576,
    tokens: 262_144,
    nodes: 2_000_000,
    stack: 65_536,
};

/// PHP short opening tags are optional; long and echo tags are always enabled.
#[derive(Clone, Copy, Debug)]
pub struct ParseOptions {
    pub limits: Limits,
    pub short_open_tags: bool,
}

pub const DEFAULT_OPTIONS: ParseOptions = ParseOptions {
    limits: DEFAULT_LIMITS,
    short_open_tags: false,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Token(&'static str),
    Rule(&'static str),
}

/// Children are indices into `SyntaxTree::nodes`, stored in source order.
/// Every child precedes its parent in storage and lies within the parent span.
#[derive(Debug)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub span: Range<usize>,
    pub children: Range<usize>,
}

/// Every node except the root has exactly one parent. EOF is not a tree node.
/// Flat storage avoids recursive ownership and recursive destruction.
/// The tree borrows source bytes; it cannot outlive the source buffer.
#[derive(Debug)]
pub struct SyntaxTree<'source> {
    pub source: &'source [u8],
    pub nodes: Vec<SyntaxNode>,
    pub edges: Vec<usize>,
    pub root: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    Lexical,
    Limit,
    Allocation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: ErrorKind,
    pub span: Range<usize>,
    pub message: &'static str,
}
