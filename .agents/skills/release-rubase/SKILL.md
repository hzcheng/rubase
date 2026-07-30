---
name: release-rubase
description: Prepare, validate, and publish a RuBase version through its protected GitHub release process. Use when the user requests a version bump, changelog finalization, release candidate, Git tag, GitHub Release, release artifact verification, or registry publication; do not use for ordinary development builds.
---

# Release RuBase

Treat a release as an explicitly authorized external publication. Follow
`docs/releasing.md`, protected `main`, the `release` environment, and immutable
version tags.

## Establish the release scope

1. Require an explicit target version and confirm whether the request covers a
   GitHub Release, crate registry publication, or both.
2. Confirm the version follows SemVer and the tag is `vX.Y.Z`.
3. Read `CHANGELOG.md`, the workspace version, package metadata, and changes
   since the previous version tag.
4. Stop rather than invent release notes, compatibility claims, credentials, or
   registry publication authority.

## Prepare through a PR

1. Start from a clean, current `main`.
2. Create the release branch in `.worktrees/<branch-slug>` from the current
   `origin/main`; keep the primary checkout on `main`.
3. Update the workspace version and every intentionally synchronized package.
4. Move relevant entries from `Unreleased` into a dated version section and
   retain an empty `Unreleased` section.
5. Update documentation required by public API or compatibility changes.
6. Run:

   ```bash
   cargo xtask release-check
   ```

7. Review the packaged file list and final diff.
8. Open a PR and wait for every required CI and CodeQL check before merging.

## Tag and publish

1. Refresh local `main` after the release PR merges.
2. Verify the release commit is the exact current `origin/main` commit.
3. Create the version tag once. Never move, replace, force-push, or delete it.
4. Push the tag to start `.github/workflows/release.yml`.
5. Wait for verification and platform builds.
6. Obtain the required `release` environment approval. Do not bypass it.
7. Verify the GitHub Release contains the expected platform binaries and
   `SHA256SUMS`.
8. Verify the published release is immutable.

## Registry publication

Treat `cargo publish` as a separate external write. Run a dry run first, verify
package ownership and credentials without exposing secrets, and publish only
when the user explicitly included registry publication in scope.

## Report

Report the version, PR and commit, tag, workflow result, artifact verification,
release URL, and whether any registry publication remains pending. Report the
skill-evolution outcome required by `AGENTS.md`, applying reusable release
workflow improvements when supported by the completed release. Remove the clean
release worktree and prune worktree metadata only after the release PR merges;
delete the merged local branch.
