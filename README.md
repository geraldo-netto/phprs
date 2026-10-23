# phprs

A PHP implementation in Rust, with a long-term goal of becoming a production
replacement for PHP, targeting PHP 8.x.x. The selected delivery is hello world
execution; the broader syntax delivery gate remains unfinished.
The working tree implements a byte-oriented lexer, an independently authored
PHP 8.5 syntax grammar, a Rust parser, a syntax-checking CLI and execution of one
echo statement containing a quoted string. Other runtime work remains deferred.
Language rules come from specifications and observable behavior;
PHP implementation sources and PHP-derived grammar extraction have been removed.
Prior source exposure is disclosed; strict clean-room separation is not claimed.
Safety and speed remain measured acceptance goals.

[LANGUAGE.md](LANGUAGE.md) defines the independent requirements.
`grammar/vocabulary.json` records language-level token classes and spellings
from documentation and public tokenization observations, without PHP token IDs.

## Run hello world

```sh
cargo run --offline -- examples/hello.php
```

The checked-in example:

```php
<?php
echo "Hello, world!\n";
```

prints exactly `Hello, world!` followed by a newline, with empty program stderr
and exit zero. Cargo may print its own build/status messages; use `cargo run
--offline --quiet -- examples/hello.php` to suppress them.

Execution accepts exactly one echo of a single- or double-quoted string, including
binary prefixes, comments and simple double-quote escapes (`\n`, `\r`, `\t`,
`\v`, `\e`, `\f`, `\\`, `\$`, `\"`). Single quotes decode only `\\` and `\'`;
unknown escapes retain their backslash. Double-quote octal, hexadecimal and
Unicode escapes, interpolation, expressions and additional statements return an
unsupported-execution diagnostic. The command never invokes PHP to execute a
program. File/read, syntax, limit and output write/flush failures return status 1.

## Check a file

```sh
cargo run --offline -- --check examples/hello.php
```

The checker prints exactly `Syntax OK` followed by a newline, leaves stderr
empty, and exits zero. It parses the file without running its statements.
Missing or extra arguments, file/read errors, rejected syntax, exhausted parser
limits, and stdout failures return status 1 with a diagnostic on stderr when
that stream is writable. Syntax diagnostics report byte offsets.

The parser handles tags and HTML, expressions, statements, callable and object
syntax, types, attributes, enums, interpolation, heredoc/nowdoc and namespaces.
Its tables are compiled from the project's own language grammar. This is syntax
acceptance, not name resolution, constant evaluation or runtime compatibility.
Tests cover representative families and bounded invalid inputs; they do not
establish equivalence for all inputs. PHP 8.x.x is the target family, with PHP 8.5
as the selected language baseline. The local PHP 8.3.6 oracle validates shared
syntax only. PHP 8.5-only fixtures follow the independent language contract;
native 8.5 comparison is deferred by owner instruction (2026-10-23).
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
`src/execution_contracts.rs` separately defines `SourceExecutor`, its options,
output limit and ownership contract. `EchoExecutor` depends on `SyntaxParser`;
its adapter selects the supported tokens using language names and source spans,
without depending on generated parser identities or node storage order. The CLI
depends on both interfaces; `main` wires their implementations.
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
in stack order before an index can be reused. Modifier validation borrows this arena only for each reduction, traverses decreasing
node indices and uses a fixed ten-entry scratch array with an explicit rejection
boundary. Hook-name validation borrows source spans and uses two fixed presence
flags; list traversal decreases node identities and retains no source references.
Parsing is synchronous, with no
callbacks or background work. On success the tree moves to the caller; temporary workspaces
are dropped. On rejection or allocation failure all invocation-owned vectors
are dropped. The CLI owns the file and source buffer for the call. Flat storage
avoids recursive destruction on deeply nested input. Tests cover tree ownership,
stack limits, malformed bytes, truncation and out-of-order heredoc closure.
Every successful generated/truncated parse is checked for parent/span
conservation; deliberate orphan, escaping-span and unused-edge controls exercise
that checker. These bounded checks do not prove unrestricted resource safety.

Each execution call owns its syntax tree and output buffer and borrows the source
only until returning. Decoding never expands the string body: it reserves at most
the smaller of its byte length and the output limit, and checks the limit before
every append. On success the output transfers to the caller; on rejection or
partial failure both the buffer and tree drop synchronously. Nothing is queued or
retained, and no callback, identity reuse or cancellation state exists. The CLI
sets the output limit to 1,048,576 bytes, writes only after evaluation succeeds,
and checks both writes and flush. A host write failure can leave partial stdout;
the process still reports failure. Tests cover all 256 literal byte values,
truncation, unsupported programs, allocation and output limits, source/output
lifetime independence and injected host failures.

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

Verification on 2026-10-23 passes 101 Rust tests and 20 Python tests, including
4,096 generated malformed-byte cases, every prefix of the positive fixtures,
Shared-syntax PHP token-class/span and acceptance comparisons and deterministic generation of
1,119 states and 637 rules. Fresh LLVM counters meet 80 percent executable-line coverage for
each measured production Rust function/closure. SonarQube complexity measurement
is unavailable locally; its required evidence remains outstanding in
[TODO.md](TODO.md). No full syntax-stage quality sign-off or production support
is claimed. Native CLI validation has been performed on Linux only.

## Later deliveries

The owner selected hello world ahead of full syntax sign-off. Values,
extensions, hosting and production release criteria remain deliberately deferred
in the feature tables in
[TODO.md](TODO.md).
