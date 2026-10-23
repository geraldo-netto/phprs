<!-- Copyright 2026 Geraldo Netto
SPDX-License-Identifier: Apache-2.0 -->

# Project instructions

## Delivery scope

- Seek the smallest independently verifiable delivery. Owner instruction
  (2026-10-23): deliver hello world execution now, without finishing the syntax
  gate first. All other runtime work remains deferred.
- The long-term goal is a production replacement for PHP. Do not introduce
  infrastructure or language features solely for a later milestone.
- Owner rule (2026-10-23): implement phprs independently from language
  specifications and observable behavior. Do not copy or translate PHP engine
  code, extract its grammar, or derive parser tables from PHP implementation
  sources. Previously derived artifacts must be replaced, not extended or kept
  as a fallback. This is an independent implementation requirement; prior source
  exposure means strict clean-room separation must not be claimed.
- Safety and performance are acceptance goals, not assumptions. Preserve bounded
  resources, explicit ownership and permanent fault/invalid-input regressions;
  support any speed or memory advantage with reproducible measurements over
  stated workloads and equivalent operations.
- Owner rule (2026-10-23, persistent for phprs): target PHP 8.x.x. Do not retain
  compatibility-only shims, deprecated aliases, legacy APIs/configuration/formats,
  migrations, wrappers or fallback paths. Update current callers, contracts,
  documentation and permanent tests together instead of preserving obsolete
  interfaces. Syntax still supported by the PHP 8 target is a language requirement,
  not a compatibility shim; do not remove it solely because it predates PHP 8.
- Keep the PHP 8.x target distinct from verified version coverage: the current
  single independently authored grammar targets PHP 8.5; the local comparison oracle is PHP 8.3.6
  and validates only shared syntax. Do not maintain parallel minor-version
  implementations or claim native PHP 8.5 validation without its oracle.
- Keep proposed, implemented, and verified behavior clearly distinguished.
- Keep TODO.md organized as one feature table per domain. Every feature needs a
  stable ID, disposition, explicit dependencies, and a verifiable completion
  condition. Remove completed entries; preserve deliberate deferrals.
- Reassess the next smallest verifiable example after each delivery. Deferred
  features do not automatically become active work.

## Licensing and future development

- Owner decision (2026-10-23): license the current public hello world base under
  Apache-2.0, using the standard unmodified license text and accurate notices.
- Attribute relevant authored files to Geraldo Netto with a copyright header and
  an Apache-2.0 SPDX identifier. Preserve interpreter shebangs and exact-byte
  fixtures; use metadata for formats without comments. Generated source headers
  must be emitted by the generator. Leave standard license text unmodified.
- Further development is intended for a separate private, proprietary edition.
  Do not publish subsequent implementation work under Apache-2.0 automatically;
  establish its repository and licensing boundary before starting that work.
- Preserve recipients' Apache-2.0 rights to the published base. The public-base
  license does not relicense removed material in earlier Git history.

## Implementation quality

- Separate independent language contracts from parser, execution, and OS adapters.
  Keep one authoritative implementation and owner per behavior.
- Owner rule (2026-10-23, persistent for phprs): define public interfaces,
  requests, responses, errors and schemas independently of concrete parser,
  execution, storage, UI or OS-backend internals. Implementations depend on these
  contracts through explicit adapters; consumers receive focused interfaces and
  concrete implementations are wired at the application boundary.
- Apply all five SOLID principles where relevant: cohesive single responsibilities;
  extension through explicit boundaries; substitutable implementations that honor
  the same behavioral, lifetime and resource contracts; narrow consumer interfaces;
  and dependencies directed toward independent abstractions. Test contracts
  independently. Add patterns only to solve concrete design problems, without
  speculative layers, duplicated state or compatibility wrappers.
- Each new or changed function has one responsibility, minimal variable scope,
  and SonarQube cyclomatic complexity at most 9. Track unavailable measurement.
- Require at least 80 percent executable-line coverage for each maintained
  production function, plus permanent boundary and invalid-input tests.
- Confirmed behavioral fixes require a permanent regression that fails before the
  fix and passes afterward, linked to its TODO ID.
- Check resource ownership, borrow lifetimes, cleanup, and safe reuse in changed
  code. Stateless code still needs explicit lifetime and input/output bounds.
- Record findings and missing validation in TODO.md. Documentation follows
  verified behavior; accepted requirements and independent tests remain the
  correctness standard.
- Prefer local tools and offline checks. Do not download dependencies or push
  changes without explicit authorization for that transfer.
