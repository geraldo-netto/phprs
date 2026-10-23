# phprs

A PHP runtime written in Rust, with a long-term goal of becoming a production
replacement for PHP. The first delivery is deliberately small: run a PHP hello
world program.

Status: planning. No interpreter is implemented yet.

## First delivery

Run this program from a file:

```php
<?php
echo "Hello, world!\n";
```

The planned command is:

```sh
cargo run --offline -- examples/hello.php
```

Success means stdout contains exactly `Hello, world!` followed by a newline,
stderr is empty, and the process exits with status zero. Invalid syntax and
unreadable files must fail cleanly. This command becomes available when the first
delivery is implemented.

## Implementation plan

Start with one Rust package and no third-party dependencies. Keep parsing,
execution, and host file/process operations in focused modules with explicit
inputs and outputs. Parse into an independently defined statement type; execution
must not depend on parser internals or OS APIs.

For the first delivery, recognize the PHP opening tag, whitespace, one `echo`
statement, a double-quoted literal containing ordinary bytes and `\n`, and its
semicolon. Use a direct evaluator for that statement. Define accepted input and
output bounds and reject unsupported syntax explicitly. Variables, interpolation,
multiple statements, closing tags, and additional escapes come later.

Keep program bytes and output bytes distinct from host path handling. The CLI
owns the source buffer and output destination; parsed data cannot outlive its
source, and all resources settle when the invocation ends. No shared heap,
garbage collector, bytecode compiler, or JIT is needed for this delivery.

Add a permanent automated acceptance test for the exact program and output, plus
invalid-input and file-error tests. PHP 8.3.6 is available locally as an initial
comparison oracle; the production compatibility version remains a later decision.
Use offline builds and checks without fetching dependencies.

## Roadmap

[TODO.md](TODO.md) groups features by domain and records dependencies and delivery
conditions. Finish the hello world gate before expanding the language. Choose
each subsequent increment from a small executable PHP example and its tests.

Production support will eventually require a selected PHP version, application
and extension compatibility, correct resource ownership, native platform
validation, operational acceptance, and measured performance. The deferred
features are planning candidates, not implemented capabilities or fixed delivery
dates.
