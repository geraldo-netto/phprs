# phprs

A PHP implementation in Rust, with a long-term goal of becoming a production
replacement for PHP, targeting PHP 8.x.x. The current stage is PHP syntax support.
The working tree implements a byte-oriented lexer, a parser derived from the
PHP 8.5.0 grammar,
and a syntax-checking CLI. Execution remains deferred.

The current grammar tables were derived from PHP implementation source. That
approach is superseded by the owner's independent-implementation requirement
(PHPRS-090). They must be replaced with an independently authored language
contract based only on specifications and observable behavior; no clean-room
separation is claimed. Safety and speed remain measured acceptance goals.

## Check a file

```sh
cargo run --offline -- --check examples/hello.php
```

For the checked-in example:

```php
<?php
echo "Hello, world!\n";
```

The checker prints exactly `Syntax OK` followed by a newline, leaves stderr
empty, and exits zero. It parses the file without running its statements.
Missing or extra arguments, file/read errors, rejected syntax, exhausted parser
limits, and stdout failures return status 1 with a diagnostic on stderr when
that stream is writable. Syntax diagnostics report byte offsets.

The parser handles tags and HTML, expressions, statements, callable and object
syntax, types, attributes, enums, interpolation, heredoc/nowdoc and namespaces.
Its tables preserve the upstream grammar's productions and precedence. This is
syntax acceptance, not PHP semantic validation, name resolution, compilation,
or runtime compatibility. Tests cover representative language families and a
bounded invalid-input corpus; they do not establish equivalence for all inputs.
PHP 8.x.x is the target family, with PHP 8.5.0 as the single syntax baseline.
The table integration is implemented; scanner extensions remain in progress
for property comments. Asymmetric visibility and void casts are implemented
and covered by exact-token, position and malformed-boundary tests. The local PHP
8.3.6 oracle validates shared syntax only. New PHP 8.5 fixtures are specified
against the pinned grammar; native 8.5 comparison remains outstanding.
Production acceptance versions and application coverage remain undecided.
There are no compatibility-only shims for previous phprs APIs or PHP versions
outside the target. Syntax supported by PHP 8 remains part of the language.

## Contracts and ownership

`src/contracts.rs` defines source spans, syntax nodes, diagnostics and per-call
limits independently of parser storage algorithms or host APIs. Its focused
`SyntaxParser` interface accepts source bytes and options and returns a concrete
syntax tree or a diagnostic. `PhpParser` implements that interface with the
checked-in grammar; the CLI depends only on the interface. `main` selects the
implementation and connects host arguments and output streams. Callers use the
interface directly.
Contract tests use an independent implementation, and CLI fault tests inject a
parser without depending on generated grammar tables.
The tree borrows the source and owns flat node/edge vectors; it cannot outlive
that source. Every non-root node has one parent; child spans lie within parent
spans. EOF is excluded from tree storage. Trivia is retained in source bytes,
not as separate nodes. Quoted token spelling is available through source spans;
escapes are checked where required but values are not evaluated.

The default limits are 1,048,576 source bytes, 262,144 tokens, 2,000,000 nodes,
and 65,536 entries in each parser/lexer stack. Long and echo tags are enabled;
short tags are opt-in through `ParseOptions`. Library parsing preserves leading
`#!` text by default, so PHP opening tags on that line remain significant.
`skip_shebang: true` discards an initial `#!` line through LF, or through EOF if
LF is absent. The CLI explicitly selects this file mode. All source bytes,
including a discarded preamble, count toward the source byte limit. The CLI
reads at most that limit plus one byte to detect oversized files.

`PhpParser` is stateless; its borrowed interface introduces no heap allocation or
retained references. A result borrows only its input source, so it can outlive the
parser instance. Each parse owns its lexer frames, heredoc contexts and parser
workspace. Contexts are referenced by indices while active; nested heredocs settle
in stack order before an index can be reused. Parsing is synchronous, with no
callbacks or background work. On success the tree moves to the caller; temporary workspaces
are dropped. On rejection or allocation failure all invocation-owned vectors
are dropped. The CLI owns the file and source buffer for the call. Flat storage
avoids recursive destruction on deeply nested input. Tests cover tree ownership,
stack limits, malformed bytes, truncation and out-of-order heredoc closure.
Every successful generated/truncated parse is checked for parent/span
conservation; deliberate orphan, escaping-span and unused-edge controls exercise
that checker. These bounded checks do not prove unrestricted resource safety.

## Offline verification

Builds use one Rust package with no third-party dependencies. With the installed
Rust tools, PHP 8.3.6, Python 3 and GNU Bison:

```sh
cargo test --offline
cargo fmt --check
cargo clippy --offline --all-targets -- -D warnings
cargo llvm-cov --offline --all-targets --json --output-path target/coverage.json
python3 tools/check_coverage.py
python3 -m unittest discover -s tests -p 'test_*.py'
```

The Python suite consumes the fresh LLVM coverage report, so run those commands
in order. Raw-source differential tests use local PHP's `token_get_all` with
`TOKEN_PARSE`; file-preamble tests use PHP's `-l` mode. Neither executes the
checked source. See
[grammar/README.md](grammar/README.md) for source identity, licensing notices and
offline table regeneration.

Verification on 2026-10-23 passes 45 Rust tests and 17 Python tests, including
4,096 generated malformed-byte cases, every prefix of the positive fixtures,
Shared-syntax PHP acceptance comparisons and deterministic generation of
1,193 states and 624 rules. Fresh LLVM counters meet 80 percent executable-line coverage for
each measured production Rust function/closure. SonarQube complexity measurement
is unavailable locally; its required evidence remains outstanding in
[TODO.md](TODO.md). No full syntax-stage quality sign-off or production support
is claimed. Native CLI validation has been performed on Linux only.

## Later deliveries

Execution starts with the hello world example only after syntax delivery. Its
planned command is `cargo run --offline -- examples/hello.php`, with exact output
`Hello, world!` followed by a newline, empty stderr and exit zero. That execution
command is not implemented. Values, extensions, hosting and production release
criteria remain deliberately deferred in the feature tables in
[TODO.md](TODO.md).
