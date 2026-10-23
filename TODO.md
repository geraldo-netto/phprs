<!-- Copyright 2026 Geraldo Netto
SPDX-License-Identifier: Apache-2.0 -->

# TODO

Current scope (owner instruction 2026-10-23): license the public hello world base
under Apache-2.0; further development is intended private and proprietary.
Other runtime work and native PHP 8.5 comparison stay deferred; SonarQube
measurement is unavailable.

## Delivery and compatibility

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-003 | deferred | Select the PHP 8.x production acceptance profile | Revisit after PHPRS-030 | PHP 8.x.x is the language target; select release acceptance versions within that family, integer/platform assumptions, representative applications, extensions and measurable release criteria before production claims. No compatibility-only shims. |

## Syntax and parsing

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-115 | open | Admit readonly function identifiers | PHP 8 language contract; local token-parse observations | Local PHP token parsing admits `function readonly(){}`, `readonly(...)` and `readonly(1)->p`, but phprs rejects them. Native parsing rejects bare `readonly;`, class/constant names and `use function readonly;`. Preserve readonly modifiers and these placement boundaries; retain failing-before/passing-after tests. |
| PHPRS-053 | deferred | Compare syntax with a native PHP 8.5 oracle | Owner instruction 2026-10-23: skip native 8.5 validation and continue implementation | Native PHP 8.5 comparison is explicitly deferred; only PHP 8.3.6 is installed. Revisit only on owner resumption with an available executable or explicit transfer authorization. Do not claim native 8.5 equivalence. |
| PHPRS-010 | in_progress | Finish the PHP 8.5 syntax delivery gate | PHPRS-090, PHPRS-031, PHPRS-047, PHPRS-049, PHPRS-052, PHPRS-054, PHPRS-055, PHPRS-056, PHPRS-057, PHPRS-115 | Retain the single source-spanned PHP 8.5 parser, reproducible grammar, syntax corpus and CLI; close language acceptance and quality evidence before full syntax claims. The owner selected hello world independently of this unfinished gate; broader runtime work stays deferred. |

## Execution and values

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-021 | deferred | Add scalar values, variables, and expressions | Revisit after PHPRS-030 | Select a small program, define evaluation order and coercions against the chosen PHP profile, and pass permanent semantic and invalid-input tests. |
| PHPRS-022 | deferred | Add arrays, references, and managed resources | After PHPRS-021; activate only when a selected example requires them | Define ownership, copy-on-write and aliasing behavior, cycle handling, destructor order, rooting, cleanup, and identity reuse; validate bounded models and runtime regressions before use. |

## Command line and host boundaries

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-026 | deferred | Add PHP CLI options and configuration | Revisit after PHPRS-030 | Implement only the options required by the next selected programs, with explicit configuration precedence and platform-independent contracts. |

## Tests and delivery evidence

| id | status | feature | dependencies | completion condition |
|---|---|---|---|---|
| PHPRS-031 | in_progress | Finish function-quality and resource-boundary verification | PHPRS-035 | Retain fresh per-function coverage at least 80 percent (198 Rust function/closure groups and 20 Python tooling functions currently pass), bounded invalid-input tests and ownership/cleanup checks documented in README.md; obtain the missing complexity evidence before sign-off. |
| PHPRS-035 | blocked | Measure SonarQube complexity per function | SonarQube Rust/Python analysis tooling is not installed locally | Produce SonarQube complexity values at most 9 for every new/changed production, test and tooling function, including heredoc_close_preserves_active_identity_phprs_034 and the hello world executor/CLI/tests; record analyzer/tool versions and refactor any violations. Requires locally available supported analysis tooling or explicit owner authorization for necessary transfers; estimates and Clippy are not substitutes. |

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
| PHPRS-116 | open | Audit public-base provenance and third-party notices | Apache-2.0 selected; current independent-implementation provenance in LANGUAGE.md and grammar/README.md | Audit the current source, grammar, vocabulary, fixtures, tooling and generated artifacts for third-party material and required notices; document origins and redistribution permissions before packaging. Review removed historical material separately; the current license does not relicense earlier Git history. License selection alone is not a completed provenance audit. |
| PHPRS-081 | deferred | Package verified public-base releases | PHPRS-116; owner resumption of packaging | Apache-2.0 is selected for the public base. Produce reproducible packages only for validated targets after the provenance audit; further private/proprietary development requires a separate repository and licensing boundary. |
