# RuBase Repository Instructions

## Scope

- Preserve unrelated user changes and keep each change focused.
- Treat `crates/rubase` as the public facade and `crates/rubase-cli` as a
  consumer of that facade.
- Add internal crates only after their ownership, dependency direction, and
  independent testing or build value are clear.

## Worktree isolation

- Keep the primary checkout on a clean, current `main`. Use it only for
  read-only inspection, synchronization, and worktree management.
- Before changing tracked files, create a dedicated branch and worktree at
  `.worktrees/<branch-slug>` from the current `origin/main`.
- Reuse an existing worktree only when it already belongs to the same task and
  its unrelated state is understood and preserved.
- Run edits, tests, commits, pushes, and pull-request preparation from the
  dedicated worktree.
- Never remove a dirty worktree. After its pull request is merged, remove the
  worktree, prune stale metadata, and delete the merged local branch.
- Keep `.worktrees/` ignored and never commit its contents.

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

## Skill evolution

- At the end of every feature, fix, refactor, ADR, CI, or release task, review
  whether the work exposed missing or stale reusable agent guidance.
- Report one outcome in the handoff: no skill change needed, an existing skill
  was improved, or a new skill is recommended.
- Update an existing skill when the lesson is repository-specific, reusable,
  and supported by the completed work. Avoid encoding one-off task details.
- Create a new skill only for a distinct recurring or high-risk workflow that
  does not fit an existing skill.
- Validate every changed skill with the repository's skill validation process
  and keep its `agents/openai.yaml` metadata aligned with `SKILL.md`.

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
