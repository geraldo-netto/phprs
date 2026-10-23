# Project instructions

## Delivery scope

- Seek the smallest independently verifiable delivery. The current stage is
  full PHP syntax support; execution and all other runtime work come later.
- The long-term goal is a production replacement for PHP. Do not introduce
  infrastructure or language features solely for a later milestone.
- Keep proposed, implemented, and verified behavior clearly distinguished.
- Keep TODO.md organized as one feature table per domain. Every feature needs a
  stable ID, disposition, explicit dependencies, and a verifiable completion
  condition. Remove completed entries; preserve deliberate deferrals.
- Reassess the next smallest verifiable example after each delivery. Deferred
  features do not automatically become active work.

## Implementation quality

- Separate independent language contracts from parser, execution, and OS adapters.
  Keep one authoritative implementation and owner per behavior.
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
