# TODO

## Delivery and compatibility

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-003 | deferred | Select the PHP 8.x production acceptance profile | Revisit after PHPRS-030 | PHP 8.x.x is the language target; select release acceptance versions within that family, integer/platform assumptions, representative applications, extensions and measurable release criteria before production claims. No compatibility-only shims. |

## Syntax and parsing

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-047 | open | Verify pipe expressions and precedence | PHPRS-044 | Permanent positive/negative fixtures cover chained pipes, callable operands, precedence and malformed boundaries with source-spanned trees. |
| PHPRS-054 | open | Verify clone and exit callable forms | PHPRS-044 | Cover named/unpacked/trailing/first-class argument forms and clone's parenthesis ambiguity; reject malformed argument lists without executing input. |
| PHPRS-055 | open | Verify direct new-expression dereferencing | PHPRS-044 | Cover calls, property/nullsafe access, offsets and dynamic/anonymous construction; reject bare-new forms excluded by the grammar. |
| PHPRS-056 | open | Verify attributed global constants | PHPRS-044 | Cover attribute groups, lists and namespace scopes on global constants; reject invalid attribute placement and incomplete declarations. |
| PHPRS-090 | in_progress | Remove the PHP-derived grammar dependency | PHPRS-091, PHPRS-092, PHPRS-093, PHPRS-094, PHPRS-095, PHPRS-096 | Current grammar/php-8.5.0 inputs, extracted grammar and generated tables conflict with the owner's independent-implementation rule. Replace them with an independently authored language contract, remove vendored implementation source/extraction tooling and retain one production parser path, with the full normal suite passing. |
| PHPRS-092 | open | Define independent lexical vocabulary | PHPRS-091 | Specify keywords, operators, punctuation and contextual tokens independently; verify spellings, uniqueness and spans against documentation and local tokenization observations. No PHP-generated token numbering. |
| PHPRS-093 | open | Author independent expression grammar | PHPRS-091, PHPRS-092 | Define literals, interpolation, collections, calls, access, assignments and operator precedence from documentation/behavior, with positive/negative and precedence fixtures. |
| PHPRS-094 | open | Author independent statement and control-flow grammar | PHPRS-091, PHPRS-092, PHPRS-093 | Define blocks, flow control, exception handling, declarations and alternate syntax independently; retain malformed/truncation fixtures. |
| PHPRS-095 | open | Author independent declaration and type grammar | PHPRS-091, PHPRS-092, PHPRS-093 | Define functions, captures, namespaces, classes, traits, interfaces, enums, properties/hooks, attributes and types independently; verify boundaries and rejection fixtures. |
| PHPRS-096 | open | Compile the independent grammar and cut over once | PHPRS-093, PHPRS-094, PHPRS-095 | Generate tables solely from the authored contract using the generic local table compiler; replace PHP-derived artifacts, validate deterministic generation, conservation, limits and per-function coverage without a fallback parser. |
| PHPRS-048 | open | Preserve property mode across comments | PHPRS-044; PHP 8.5 scanner property rules | Keep object/nullsafe property names contextual across comments and whitespace; test reserved words, malformed comments, interpolation and code transitions. |
| PHPRS-049 | open | Verify property-hook syntax and __PROPERTY__ | PHPRS-044, PHPRS-045 | Cover block/arrow/abstract and attributed hooks, references, parameter lists and promoted hooks; distinguish grammar acceptance from compile-time restrictions and retain malformed/truncation tests. |
| PHPRS-052 | open | Audit lexical behavior against independent contracts | PHPRS-092 | Verify token spellings, magic names, casts, strings, numbers and contextual modes against language documentation and observable behavior; retain bounded rejection and span tests. Do not inspect PHP implementation sources. |
| PHPRS-057 | open | Define independent modifier rejection policies | PHPRS-095 | Specify admitted modifier combinations and syntax-level rejection independently from language documentation and observable behavior; distinguish compile/runtime restrictions and retain invalid-input tests. |
| PHPRS-053 | blocked | Compare syntax with a native PHP 8.5 oracle | PHPRS-047, PHPRS-049, PHPRS-052, PHPRS-054, PHPRS-055, PHPRS-056, PHPRS-057; local PHP is 8.3.6 only | Run the corpus against PHP 8.5 without executing inputs and investigate disagreements. Missing prerequisite: an available PHP 8.5 executable; source-only download authorization does not authorize binary/package transfers. |
| PHPRS-010 | in_progress | Finish the PHP 8.5 syntax delivery gate | PHPRS-090, PHPRS-031, PHPRS-047, PHPRS-049, PHPRS-052, PHPRS-053, PHPRS-054, PHPRS-055, PHPRS-056, PHPRS-057 | Retain the single source-spanned PHP 8.5 parser, reproducible grammar, syntax corpus and CLI; close language acceptance and quality evidence before full syntax claims. Execution remains deferred. |

## Execution and values

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-020 | deferred | Evaluate one echo statement | After syntax delivery; execution is explicitly deferred | Produce the exact contracted output bytes independently of parser and OS internals; enforce output bounds and test output failure handling at the host boundary. |
| PHPRS-021 | deferred | Add scalar values, variables, and expressions | Revisit after PHPRS-030 | Select a small program, define evaluation order and coercions against the chosen PHP profile, and pass permanent semantic and invalid-input tests. |
| PHPRS-022 | deferred | Add arrays, references, and managed resources | After PHPRS-021; activate only when a selected example requires them | Define ownership, copy-on-write and aliasing behavior, cycle handling, destructor order, rooting, cleanup, and identity reuse; validate bounded models and runtime regressions before use. |

## Command line and host boundaries

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-025 | deferred | Run a source file and report failures | After syntax delivery and PHPRS-020 | The planned execution command in README.md reads one file and writes exact stdout bytes; missing arguments, unreadable files, invalid programs, and output failures produce stderr diagnostics and nonzero exit status. |
| PHPRS-026 | deferred | Add PHP CLI options and configuration | Revisit after PHPRS-030 | Implement only the options required by the next selected programs, with explicit configuration precedence and platform-independent contracts. |

## Tests and delivery evidence

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-030 | deferred | Verify the hello world delivery | After syntax delivery and PHPRS-025 | Permanent automated tests run the file from README.md and assert exact stdout, empty stderr, and exit zero; rejection, file, and output-error tests pass offline. Mark hello world implemented only after these checks pass. |
| PHPRS-031 | in_progress | Finish function-quality and resource-boundary verification | PHPRS-035 | Retain fresh per-function coverage at least 80 percent (143 Rust function/closure groups and 21 Python tooling functions currently pass), bounded invalid-input tests and ownership/cleanup checks documented in README.md; obtain the missing complexity evidence before sign-off. |
| PHPRS-035 | blocked | Measure SonarQube complexity per function | SonarQube Rust/Python analysis tooling is not installed locally | Produce SonarQube complexity values at most 9 for every new/changed production, test and tooling function, including heredoc_close_preserves_active_identity_phprs_034; record analyzer/tool versions and refactor any violations. Requires locally available supported analysis tooling or explicit owner authorization for necessary transfers; estimates and Clippy are not substitutes. |

## Control flow and callable code

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-040 | deferred | Add branches, loops, functions, and exceptions | After PHPRS-021; activate one example at a time | Each selected feature has independent evaluation, scope, call, and unwind contracts with permanent semantic and cleanup tests. |
| PHPRS-041 | deferred | Add closures, generators, fibers, and dynamic loading | After PHPRS-040; activate when a selected application needs a feature | Validate captured or suspended lifetimes, cancellation, include/eval/autoload behavior, and resource settlement for each implemented feature. |

## Object model and introspection

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-050 | deferred | Add classes and object behavior | After PHPRS-022 and PHPRS-040; activate for a selected example | Define and test visibility, properties, methods, inheritance, types, interfaces, traits, magic methods, destruction, and related PHP behavior incrementally. |
| PHPRS-051 | deferred | Add namespaces, attributes, enums, and reflection | After the corresponding callable/object foundations; activate for selected programs | Implement the required resolution and metadata contracts and pass version-pinned application and conformance tests. |

## Standard library and extensions

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-060 | deferred | Add built-in functions and library types | Revisit after PHPRS-030; prioritize actual application requirements | Add required string, array, numeric, JSON, date/time, filesystem, and SPL behavior in small tested increments with independent error and coercion contracts. |
| PHPRS-061 | deferred | Add external facilities and extension support | After PHPRS-003 and relevant value/resource contracts | Decide extension source/API/ABI expectations explicitly; implement required regex, database, crypto, networking, and other facilities through owned boundary adapters with native acceptance tests. Arbitrary Zend extensions require separate compatibility evidence. |

## Hosting and production operations

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-070 | deferred | Add web request handling | After selected application language/library prerequisites; outside hello world | Implement request/response contracts, superglobals, headers, body handling, sessions, and cleanup with isolated repeat-request tests. |
| PHPRS-071 | deferred | Add production workers and native platform support | After PHPRS-070 and a selected deployment profile | Validate the chosen FastCGI/worker lifecycle, configuration, timeouts, cancellation, graceful shutdown, logging, capabilities, request isolation, and bounded resource use on each claimed platform. |
| PHPRS-072 | deferred | Validate a production release | After PHPRS-003 and all selected application/platform/extension prerequisites | Run representative applications, compatibility suites, native fault tests, security review, and sustained load tests against explicit release thresholds; retain reproducible evidence and mark unsupported capabilities visibly. |

## Performance and distribution

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-080 | deferred | Optimize measured bottlenecks | After correct representative workloads exist | Publish reproducible latency, throughput, startup, and memory baselines; introduce bytecode, caches, or JIT only when measurements justify them, retaining one authoritative execution path. |
| PHPRS-081 | deferred | Choose licensing and package releases | Owner license decision required before adding a license; packaging follows a verified runnable delivery | Select a project license and audit any reused PHP material/dependencies; produce reproducible packages only for validated targets. Public repository visibility does not supply a license. |
