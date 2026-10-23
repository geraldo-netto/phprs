# Independent syntax grammar

The language contract is defined in [LANGUAGE.md](../LANGUAGE.md).
`independent/expressions.y`, `independent/statements.y` and
`independent/declarations.y` are authored from language documentation and public
behavior observations. `independent/precedence.y` states the binding rules.
`vocabulary.json` defines token classes and spellings; its provenance records the
observed PHP version and classes specific to the selected PHP 8.5 profile.
PHP implementation source and grammar extraction are not inputs. No strict
clean-room separation is claimed, because prior source exposure is disclosed.

The generic table compiler assembles these contracts and asks installed GNU Bison
for an XML automaton report. It rejects embedded semantic actions/code, requires
zero grammar conflicts, and emits `syntax.y`, `src/syntax/token_ids.rs` and
`src/syntax/tables.rs`. Token numbering is internal to this grammar and is not
PHP's public numeric token numbering. Only XML metadata is used; generated C is
neither compiled nor shipped. No program being checked executes.

Check generated files offline from the project root:

```sh
python3 tools/generate_parser.py --check
```

Omit `--check` to intentionally regenerate. Temporary files and subprocesses belong
to the synchronous generator invocation; its temporary-directory context removes
artifacts on success or failure. Verification on 2026-10-23 reproduces 1,113 states
and 634 rules using GNU Bison 3.8.2, Python 3.12.3 and rustfmt 1.9.0-stable from
Rust 1.98.1. Other versions have not been checked for byte-identical output.

The installed PHP executable is 8.3.6 and can compare shared syntax only.
Documentation-derived PHP 8.5 fixtures, table bounds and bounded invalid-input
checks do not replace native PHP 8.5 acceptance evidence, explicitly deferred by
the owner on 2026-10-23 (PHPRS-053). Project
licensing remains an owner decision (PHPRS-081).
