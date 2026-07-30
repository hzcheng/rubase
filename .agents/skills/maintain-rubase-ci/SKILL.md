---
name: maintain-rubase-ci
description: Safely modify RuBase GitHub Actions, required checks, dependency automation, CodeQL, branch protection, or release workflow configuration. Use for changes under `.github/`, Action version updates, CI job changes, workflow permission changes, Dependabot configuration, security scanning, or GitHub repository rules that must remain compatible with protected `main`.
---

# Maintain RuBase CI

Preserve the protected-branch contract while evolving automation. Use GitHub
read APIs to discover live state; do not guess required check names or assume a
mutable Action tag is safe.

## Audit before editing

1. Inspect the local branch, worktree, `.github/workflows/`,
   `.github/dependabot.yml`, and relevant release documentation.
2. Read current required check contexts from GitHub branch protection.
3. Read Actions permissions, allowed Actions, SHA-pinning policy, merge
   settings, environments, and rulesets when the task affects them.
4. Inspect a real workflow run to confirm displayed check names.
5. Stop before external writes unless the user explicitly requested repository
   configuration changes.

## Update Actions safely

1. Resolve every Action version to its final commit with:

   ```bash
   .agents/skills/maintain-rubase-ci/scripts/resolve-action-sha.sh \
     actions/checkout v7
   ```

2. Pin the complete SHA and retain the human-readable major version:

   ```yaml
   uses: actions/checkout@<full-sha> # v7
   ```

3. Keep workflow permissions at the minimum needed by each job.
4. Keep third-party Actions disallowed unless the user explicitly approves a
   reviewed exception.
5. Add retention limits to transient artifacts.

## Preserve branch and release protection

- Do not rename a required check casually. When a rename is necessary, publish
  the workflow change through a PR, observe the new real check, then update
  branch protection without leaving an impossible requirement.
- Do not enable SHA enforcement until all workflows on the default branch use
  full SHAs.
- Keep `main` strict, linear, PR-only, admin-enforced, and protected from force
  push or deletion.
- Keep CodeQL required and keep dependency security updates enabled.
- Keep the `release` environment restricted to version tags and manual
  approval.
- Never weaken immutable releases or the version-tag update/deletion rules.

## Validate and publish

1. Run `git diff --check` and `cargo xtask ci`.
2. Verify every `uses:` entry contains a 40-character SHA.
3. Publish through a draft PR; never push directly to `main`.
4. Wait for all existing CI and new workflow checks.
5. Merge only after successful checks.
6. Apply dependent repository settings after the workflow is present on
   `main`, then read the settings back to verify them.
7. Report workflow checks, repository settings changed, and any manual release
   approval still required.
