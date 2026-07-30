# ADR-0001: Use a facade-oriented Cargo workspace

- Status: accepted
- Date: 2026-07-30
- Owners: maintainers

## Context

RuBase needs room to grow without exposing its internal package structure as a
permanent public API. The repository also needs independently runnable
integration tests, compatibility tests, benchmarks, and automation.

## Decision

Use a Cargo workspace with `rubase` as the public facade and `rubase-cli` as the
binary package. Keep repository automation in `tools/xtask`. Add internal crates
only when a stable dependency boundary has emerged.

Cross-cutting test and benchmark harnesses are separate workspace packages. Fuzz
and packaging directories are kept outside the default build until configured.

## Consequences

Consumers have one stable library dependency while internal code can be
reorganized. Shared metadata, dependency versions, lints, and profiles remain
centralized. The facade adds a small amount of indirection.

## Alternatives considered

- A single package would be initially simpler but would mix library, binary, and
  repository-level test concerns.
- Many narrowly scoped packages would create premature boundaries and increase
  build and release overhead.
