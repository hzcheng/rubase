---
name: develop-rubase
description: Implement, fix, or refactor code in the RuBase repository while preserving its architecture, compatibility, testing, and pull-request requirements. Use for ordinary RuBase feature work, bug fixes, internal refactors, public API changes, dependency changes, or test additions; use a more specific RuBase skill when the task is an ADR, CI maintenance, or a release.
---

# Develop RuBase

Follow the repository instructions in `AGENTS.md`. Keep the change scoped to the
user's request and do not invent database architecture that has not been
accepted in an ADR.

## Isolate the change

1. Inspect the primary checkout and existing worktrees before editing.
2. If the current directory is not already the dedicated worktree for this
   task, run the following from the primary checkout root to create one from
   the current `origin/main`:

   ```bash
   git fetch origin
   git worktree add .worktrees/<branch-slug> \
     -b <branch-name> origin/main
   ```

3. Perform all tracked-file changes, validation, commits, pushes, and
   pull-request preparation inside that worktree.
4. Keep the primary checkout on `main`; do not make the task change there.

## Orient the change

1. Inspect `git status`, `git worktree list`, the affected crate, nearby tests,
   and relevant files in `docs/architecture/` and `docs/adr/`.
2. Identify the observable behavior and the smallest layer that owns it.
3. Classify the change before editing:
   - Internal implementation: preserve public behavior and add focused tests.
   - Public API: document it and add integration or doc tests.
   - Persisted bytes: require a format version, compatibility fixtures, and an
     ADR before implementation.
   - Concurrency or resource lifecycle: state invariants and add deterministic
     failure-path tests.
   - Performance-sensitive behavior: establish a benchmark before optimizing.
   - New production dependency or crate boundary: use `$record-rubase-adr`.
4. Stop and ask for direction when the needed decision would materially expand
   scope or contradict an accepted ADR.

## Implement

1. Preserve existing user changes.
2. Keep dependencies directed toward `crates/rubase`; do not expose internal
   packages as the default consumer API.
3. Put shared dependency versions in the root workspace manifest.
4. Keep recoverable failures explicit and structured.
5. Keep `unsafe` forbidden.
6. Add the smallest tests that fail without the change and pass with it.
7. Update public docs, architecture notes, or compatibility fixtures when the
   change affects them.

## Validate

1. Run the narrowest relevant test during iteration.
2. Run `cargo xtask ci` before handoff.
3. Run `cargo xtask release-check` when changing public package metadata,
   profiles, packaging, or release behavior.
4. Run or update a relevant benchmark when making a performance claim.
5. Inspect the final diff for accidental API, dependency, format, or generated
   file changes.

## Hand off

Report the behavior changed, important design choices, tests run, and remaining
risks. Include the required skill-evolution outcome from `AGENTS.md`. Update an
existing skill when the completed work produced a clear reusable lesson; only
recommend a new skill for a distinct recurring or high-risk workflow. Commit,
push, or open a PR only when the user asks for publication. After a PR merges,
remove its clean worktree, prune worktree metadata, and delete the merged local
branch.
