# PHP grammar inputs

`php-8.3.6/` contains the pinned PHP 8.3.6 parser and scanner sources and their
license notices. Their upstream identities are the `php-8.3.6` tag of `php/php-src`,
paths `Zend/zend_language_parser.y`, `Zend/zend_language_scanner.l`, `LICENSE`
and `Zend/LICENSE` (stored here as `ZEND_LICENSE`). This records the identity of
the local inputs; no remote comparison was performed during this verification.

Both Zend source files retain their copyright and Zend Engine License 2.00
headers. The PHP License 3.01 and Zend license texts are retained alongside them.
Derived grammar/tables retain these origins; the generator removes PHP semantic
C actions and comments while preserving syntax productions, tokens and
precedence. Generated line margins use spaces and omit trailing whitespace;
quoted literals and pinned source bytes remain unchanged. Consult the retained
licenses for their terms. These notices do not
select a license for phprs; that owner decision remains PHPRS-081.

SHA-256 of the local inputs:

| file in php-8.3.6/ | SHA-256 |
|---|---|
| zend_language_parser.y | `176e20bd96fea48f508b0e1f35d1c6272211823d2fcbb77a4bd289927f756d0e` |
| zend_language_scanner.l | `4191bc19d4f41f20b1da81744323394dae61a19b8808d70bcbe73724d0126ff6` |
| LICENSE | `b42e4df5e50e6ecda1047d503d6d91d71032d09ed1027ba1ef29eed26f890c5a` |
| ZEND_LICENSE | `509aeb3be64e428d7e0004ce224620ab01bf94adaacefbb449611939df56bc5b` |

The scanner is a reference for the Rust lexer, not an input to table generation.
`tools/generate_parser.py` extracts the parser grammar, uses GNU Bison's XML
report and emits `grammar/syntax.y`, `src/syntax/token_ids.rs` and
`src/syntax/tables.rs`. The generated symbol indices are internal and are not
PHP's external token numbers. Sparse symbol numbering is preserved explicitly.

From the project root, using only installed tools:

```sh
python3 tools/generate_parser.py --check
```

To intentionally regenerate instead of checking, omit `--check`. The generator
owns its temporary directory and subprocesses synchronously; temporary artifacts
are removed when the directory context exits, including on failure. No input
program executes. Verification on 2026-10-23 reproduces all checked-in generated
files, with 1,121 states, 586 rules and zero Bison grammar conflicts, using GNU
Bison 3.8.2, Python 3.12.3 and rustfmt 1.9.0-stable from Rust 1.98.1. Other tool
versions have not been validated for byte-identical output.
