# RuBase Repository Instructions

## Scope

- Work from the repository root unless a command explicitly requires another
  directory.
- Preserve unrelated user changes and keep each change focused.
- Treat `crates/rubase` as the public facade and `crates/rubase-cli` as a
  consumer of that facade.
- Add internal crates only after their ownership, dependency direction, and
  independent testing or build value are clear.

## Rust and dependency policy

- Use the Rust toolchain pinned by `rust-toolchain.toml`.
- Declare third-party dependency versions in `[workspace.dependencies]`.
- Explain why a new production dependency is necessary before adding it.
- Keep internal crates private unless publication is an explicit requirement.
- Keep `unsafe` code forbidden. Propose an ADR before changing this policy.
- Return structured errors for recoverable failures; do not use
  `unwrap`/`expect` as recoverable error handling.

## Architecture governance

- Read relevant files under `docs/architecture/` and `docs/adr/` before making
  structural changes.
- Record an ADR for new crate boundaries, production dependencies, public API
  commitments, persisted formats, concurrency models, I/O models, or changes to
  the `unsafe` policy.
- Do not silently change persisted bytes. Version the format and add
  compatibility fixtures before merging such a change.
- Avoid freezing speculative database architecture in code or documentation.

## Testing and validation

- Keep unit tests beside implementations.
- Put public crate behavior tests under the crate's `tests/` directory.
- Put cross-crate behavior in `tests/integration` and persisted-version
  compatibility in `tests/compatibility`.
- Add a regression test for every bug fix when practical.
- Run the narrowest relevant test while iterating, then run `cargo xtask ci`
  before handing off a code change.
- Run `cargo xtask release-check` for packaging, release, profile, or public
  package changes.
- Add or update benchmarks before claiming a performance improvement.

## Git and GitHub

- Never push directly to `main`; publish changes through a pull request.
- Do not rename required checks without coordinating the branch protection
  update:
  - `Validate workspace`
  - `Test on macos-latest`
  - `Test on windows-latest`
  - `CodeQL (Rust and Actions)`
- Pin every external GitHub Action to a full commit SHA and retain a version
  comment.
- Preserve least-privilege workflow permissions.
- Never move or delete a `v*` release tag.

## Code Review Rules

- Flag changes that weaken durability, compatibility, corruption detection, or
  resource-lifecycle guarantees without explicit justification and tests.
- Flag public API or persisted-format changes without documentation and
  compatibility coverage.
- Flag new dependencies, crate boundaries, or `unsafe` policy changes without
  the required ADR.
- Flag CI changes that bypass required checks, broaden token permissions, use a
  mutable Action reference, or weaken release protections.
- Leave formatting and mechanical lint enforcement to CI.
