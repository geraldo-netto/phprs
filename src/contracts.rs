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

/// Per-invocation source handling, independent of file and process adapters.
#[derive(Clone, Copy, Debug)]
pub struct ParseOptions {
    pub limits: Limits,
    /// PHP short opening tags are optional; long and echo tags are always enabled.
    pub short_open_tags: bool,
    /// File mode: discard an initial #! line through LF, or through EOF if absent.
    /// The discarded bytes still count toward the source byte limit.
    /// False (the default) treats these bytes as ordinary source text.
    pub skip_shebang: bool,
}

pub const DEFAULT_OPTIONS: ParseOptions = ParseOptions {
    limits: DEFAULT_LIMITS,
    short_open_tags: false,
    skip_shebang: false,
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
    Unsupported,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: ErrorKind,
    pub span: Range<usize>,
    pub message: &'static str,
}

/// Synchronous PHP syntax parsing, without execution or host I/O.
///
/// Each call enforces `options.limits`, including on rejected input. Success
/// returns a tree satisfying the node/span/parent invariants above; failure
/// returns a diagnostic whose span is within `source` (EOF spans may be empty).
/// Source is borrowed only for this call and by its returned tree. Implementors
/// must retain no source references or invocation resources after returning;
/// owned result storage transfers to the caller. Parser identity is independent
/// of source lifetime, and repeated calls must not share invocation state.
pub trait SyntaxParser {
    fn parse<'source>(
        &self,
        source: &'source [u8],
        options: ParseOptions,
    ) -> Result<SyntaxTree<'source>, Diagnostic>;
}
