# PHP grammar inputs

`php-8.5.0/` is the single grammar baseline. The four files were downloaded with
owner authorization on 2026-10-23, totaling 153,009 bytes. Only these source and
notice files were transferred; no PHP archive, binary or package was acquired.
The parser/scanner retain their Zend Engine License 2.00 headers, and the PHP
License 3.01 and Zend license texts are retained here. Consult those notices for
their terms. They do not select a project license for phprs (PHPRS-081).

| local file in php-8.5.0/ | upstream source | SHA-256 |
|---|---|---|
| zend_language_parser.y | [Zend grammar](https://raw.githubusercontent.com/php/php-src/php-8.5.0/Zend/zend_language_parser.y) | `2cc2c362823a2fcaff661a69b597884548ae88bdc9f1845132d8f1b8236a972b` |
| zend_language_scanner.l | [Zend scanner](https://raw.githubusercontent.com/php/php-src/php-8.5.0/Zend/zend_language_scanner.l) | `5e178b9e0a4f865a423a098ac97ba5fc04ef2f68503382e18599d89d9ad8d15b` |
| LICENSE | [PHP license](https://raw.githubusercontent.com/php/php-src/php-8.5.0/LICENSE) | `b42e4df5e50e6ecda1047d503d6d91d71032d09ed1027ba1ef29eed26f890c5a` |
| ZEND_LICENSE | [Zend license](https://raw.githubusercontent.com/php/php-src/php-8.5.0/Zend/LICENSE) | `509aeb3be64e428d7e0004ce224620ab01bf94adaacefbb449611939df56bc5b` |

The scanner is a reference for the Rust lexer, not an input to table generation.
`tools/generate_parser.py` extracts the grammar, removes C actions and comments,
and uses GNU Bison's XML report to emit `grammar/syntax.y`,
`src/syntax/token_ids.rs` and `src/syntax/tables.rs`. It preserves productions and
precedence, normalizes generated line margins, and preserves quoted literals.
Generated symbol indices are internal, with sparse numbering retained explicitly.
Removing C actions does not reproduce their rejection checks automatically;
PHPRS-052 tracks the audit of those checks.

Check all generated files offline from the project root:

```sh
python3 tools/generate_parser.py --check
```

Omit `--check` to intentionally regenerate. The generator owns its temporary
directory and waits for its subprocesses; temporary artifacts are removed on
success or failure. Input programs never execute. Verification on 2026-10-23
reproduces 1,193 states and 624 rules with zero Bison grammar conflicts, using
GNU Bison 3.8.2, Python 3.12.3 and rustfmt 1.9.0-stable from Rust 1.98.1. Other
tool versions have not been validated for byte-identical output.

The installed PHP executable is 8.3.6. It remains useful for shared-syntax
comparisons, but cannot verify PHP 8.5-only syntax. Grammar fixtures and table
bounds do not replace the missing native PHP 8.5 acceptance evidence (PHPRS-053).
