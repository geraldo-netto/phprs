# Independent syntax contract

Status: the PHP 8.x target and independent-implementation requirement are accepted
owner requirements. PHP 8.5 is the selected language baseline. The independent
grammar implementation is authored here from documentation and observable
behavior. Native PHP 8.5 acceptance and final quality evidence remain pending.

## Sources and scope

Author syntax rules from language documentation and observable tokenization or
syntax-checking behavior. PHP implementation files, extracted productions and
PHP-derived table artifacts are not implementation inputs. Prior source exposure
is disclosed; this project does not claim strict clean-room separation.

Documentation consulted on 2026-10-23:

- [Class basics and construction](https://www.php.net/manual/en/language.oop5.basic.php)
- [Object cloning](https://www.php.net/manual/en/language.oop5.cloning.php)
- [exit](https://www.php.net/manual/en/function.exit.php)
- [Functional operators](https://www.php.net/manual/en/language.operators.functional.php)
- [PHP 8.5 language additions](https://www.php.net/manual/en/migration85.new-features.php)
- [Operator precedence](https://www.php.net/manual/en/language.operators.precedence.php)
- [Property hooks](https://www.php.net/manual/en/language.oop5.property-hooks.php)
- [Visibility](https://www.php.net/manual/en/language.oop5.visibility.php)
- [Type juggling and casts](https://www.php.net/manual/en/language.types.type-juggling.php)
- [Attribute syntax](https://www.php.net/manual/en/language.attributes.syntax.php)
- [Expressions](https://www.php.net/manual/en/language.expressions.php)

Only syntax is implemented at this stage. Do not execute source, resolve names,
evaluate constants, infer types or provide runtime/extension behavior. Keep
syntax rejection distinct from compiler checks requiring declaration or value
information. A declared PHP 8.5 target does not establish native acceptance:
only PHP 8.3.6 is installed, and it can compare shared syntax only.

## Vocabulary and source handling

Input is a bounded byte slice, not a host string. Offsets are byte offsets.
Identifiers admit ASCII letters/underscore and high bytes at the start, followed
by those bytes or ASCII digits. Keyword recognition is case-insensitive; quoted
text, variable spelling and source bytes remain unchanged. Qualified names are
single lexical units with explicit relative/absolute classification.

Define the lexical vocabulary independently: keyword spellings, punctuation,
operators and contextual token classes are language-level data, not upstream
parser token numbers. Prefer longest admitted operator spelling. Reference and
type-intersection ampersands remain distinguishable through contextual scanning.
A property name remains an identifier even when its spelling is a keyword.

Retain source outside PHP tags as inline text. Long and echo opening tags are
always recognized; short tags are opt-in. By default a leading #! line is ordinary
source text. File mode explicitly discards it through LF, or through EOF if LF is
absent; every byte still counts toward the input limit. Comments and insignificant
whitespace need no tree nodes, but remain accessible in the borrowed source.

Cover integer radices/separators, floating-point forms, quoted and binary-prefixed
strings, interpolation, backticks, heredoc and nowdoc. Preserve contextual scanner
continuations across nested expressions. Validate escapes and heredoc indentation
without evaluating the resulting values. Stop treating bytes as syntax after a
valid outermost __halt_compiler statement; retained payload still counts as input.

## Expressions

Specify precedence independently from the documented operator table, including
right-associative exponentiation, coalescing and assignment; non-associative
comparison groups; PHP 8 concatenation, shifts and pipe precedence; and
left-grouped shorthand ternary chains. Reject unparenthesized adjacent ternaries
when either has a middle operand, as required by the documented PHP 8 language
restriction, even though the local raw-token parser delays that check until
compilation. Nested middle operands and explicit parentheses remain admitted.
Group calls, construction and member/index access explicitly. Named class targets
remain distinct from dynamic variable/property references; member access on a new
object without surrounding parentheses requires constructor parentheses. Distinguish
syntactic destination forms, destructuring patterns and value expressions.
Variable-like syntax can include calls or nullsafe access; compiler-only
writability restrictions remain outside parsing. Literal/constant destinations
and invalid prefix-mutation/reference operands must be rejected.

Support arrays, references, unpacking, callable arguments, named arguments,
first-class callables, closures, arrow functions, generators, match expressions,
include/require/eval and language-defined expression forms. PHP 8.5 pipe and
clone-call syntax belong to the current language contract. Pipe operands may be
expressions; an arrow-function operand must be parenthesized, while callable
validity depends on runtime values and stays outside syntax. Void casts are admitted
where a value is discarded, not as value-producing expressions. Constant
expression legality that requires compile-time evaluation remains outside syntax.

## Statements and declarations

Cover ordinary and alternative control-flow syntax, blocks, labels/goto, loops,
switch/match, exceptions, return/throw, echo, global/static declarations, unset,
declare and empty statements. Validate required delimiters, operand/argument
presence, statement terminators and nesting. Namespace/import declarations and
compiler-halt placement have explicit source-level boundaries.

Cover named/anonymous functions, reference returns, captures, parameter lists,
promotion, variadics, defaults and return types. Define nullable, union,
intersection and parenthesized intersection-in-union types explicitly.

Cover classes, interfaces, traits, enums, inheritance/implementation clauses,
members, constants, properties, methods, trait adaptations and attributes.
Property-hook forms include ordinary and arrow bodies, declarations without a
body, reference access, parameters, attributes and promoted properties. Recognize
asymmetric set visibility and __PROPERTY__ without copying engine productions.
Specify modifier rejection as a separate language policy, with independently
chosen tests and clear syntax-versus-compilation boundaries.

## Results, ownership and limits

The public SyntaxParser interface and result schema live in src/contracts.rs.
A successful tree borrows source only, owns its node/edge storage and can outlive
the parser instance. Every non-root node has exactly one parent; every child
precedes its parent in storage and has an enclosing parent span. EOF has no node.
The root is the start rule and spans the entire admitted source.

The parser owns all invocation workspaces, lexical continuations and temporary
contexts. Admission is bounded by source bytes, tokens, nodes and stack entries.
Use explicit bounded heap stacks so admitted nesting does not overflow the native
call stack. Transfer the tree on success; release temporary storage on success,
rejection and partial allocation failure. Source borrows never escape their
owner's lifetime. No background work or native callbacks survive a parse.

Retain per-function coverage, invalid-input and truncation tests, conservation
checks and deliberately faulty controls. Source/schema conformance, bounded
models and native evidence have different scopes; none is unrestricted proof.

## Acceptance and performance evidence

An independent grammar must preserve the existing permanent regressions and
normal automated suite; unsupported or unverified syntax stays visible in TODO.md.
Check authored grammar reproducibility without importing PHP source. Validate
PHP 8.5-only fixtures against a native 8.5 oracle when available, without executing
them. Missing tooling or native evidence is not a passed gate.

Measure syntax throughput and resource use only on stated, reproducible workloads.
Compare equivalent operations and versions before claiming an advantage over PHP.
The design goals are bounded, safe parsing and improved speed; implementation
language and architecture alone establish neither a safety proof nor a speedup.
