# TODO

## Delivery and compatibility

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-001 | open | Establish the minimal Rust package and hello world fixture | None | One package with no third-party dependencies builds offline; `examples/hello.php` matches README.md. |
| PHPRS-002 | open | Define the first supported syntax and execution contract | None | Independent statement and error types define accepted bytes, whitespace, escape handling, input/output bounds, and clean rejection of unsupported syntax. |
| PHPRS-003 | deferred | Select the production compatibility profile | Revisit after PHPRS-030 | Select a pinned PHP version, integer/platform assumptions, representative applications, extensions, and measurable release criteria before expanding production claims. |

## Syntax and parsing

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-010 | open | Parse the hello world PHP subset | PHPRS-001, PHPRS-002 | Parse an opening tag, whitespace, one `echo`, a double-quoted byte literal with `\n`, and a semicolon into the independent statement type; reject malformed, truncated, oversized, and trailing unsupported input without panicking. |
| PHPRS-011 | deferred | Expand literals and statement syntax | Revisit after PHPRS-030 | Choose the next executable example; specify and test its required single quotes, escapes, comments, closing tags, interpolation, or multiple statements before adding them. |

## Execution and values

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-020 | open | Evaluate one echo statement | PHPRS-002 | Produce the exact contracted output bytes independently of parser and OS internals; enforce output bounds and test output failure handling at the host boundary. |
| PHPRS-021 | deferred | Add scalar values, variables, and expressions | Revisit after PHPRS-030 | Select a small program, define evaluation order and coercions against the chosen PHP profile, and pass permanent semantic and invalid-input tests. |
| PHPRS-022 | deferred | Add arrays, references, and managed resources | After PHPRS-021; activate only when a selected example requires them | Define ownership, copy-on-write and aliasing behavior, cycle handling, destructor order, rooting, cleanup, and identity reuse; validate bounded models and runtime regressions before use. |

## Command line and host boundaries

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-025 | open | Run a source file and report failures | PHPRS-010, PHPRS-020 | The planned command in README.md reads one file and writes exact stdout bytes; missing arguments, unreadable files, invalid programs, and output failures produce stderr diagnostics and nonzero exit status. |
| PHPRS-026 | deferred | Add PHP CLI options and configuration | Revisit after PHPRS-030 | Implement only the options required by the next selected programs, with explicit configuration precedence and platform-independent contracts. |

## Tests and delivery evidence

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-030 | open | Verify the hello world delivery | PHPRS-025, PHPRS-031 | Permanent automated tests run the file from README.md and assert exact stdout, empty stderr, and exit zero; rejection, file, and output-error tests pass offline. Mark hello world implemented only after these checks pass. |
| PHPRS-031 | open | Verify function quality and resource boundaries | Alongside PHPRS-010, PHPRS-020, PHPRS-025; required before declaring their quality checks complete | Measure at least 80 percent executable-line coverage per production function and SonarQube complexity at most 9 per new/changed function; test bounded invalid inputs and review ownership/cleanup. SonarQube tooling is not installed locally: record actual missing evidence rather than substituting estimates, and obtain transfer authorization before acquiring tools. |
| PHPRS-032 | deferred | Grow the PHP conformance corpus | Revisit after PHPRS-030 and PHPRS-003 | Add version-pinned differential fixtures and independent expected results for each new feature; preserve diagnostics, exit status, and side effects, and investigate oracle disagreements. |

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
