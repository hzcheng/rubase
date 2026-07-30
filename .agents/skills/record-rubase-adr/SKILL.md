---
name: record-rubase-adr
description: Create or update RuBase architecture decision records for durable engineering choices. Use when selecting or changing crate boundaries, production dependencies, public API commitments, persisted formats, compatibility policy, concurrency or transaction models, sync versus async I/O, recovery strategy, or the repository's unsafe-code policy.
---

# Record a RuBase ADR

Use `docs/adr/0000-template.md` and keep decisions separate from incidental
implementation details.

## Decide whether an ADR is needed

Create an ADR when a choice is costly to reverse, constrains multiple future
changes, or changes a repository-wide invariant. Do not create one for a local
refactor, routine test addition, typo, or an implementation detail already
covered by an accepted ADR.

## Prepare the record

1. Keep the primary checkout on `main`; create or enter a task-specific
   `.worktrees/<branch-slug>` worktree before changing the ADR.
2. Read `docs/architecture/` and all ADRs touching the same decision.
3. Identify the next four-digit ADR number from existing filenames.
4. Copy `docs/adr/0000-template.md` to
   `docs/adr/NNNN-short-decision-name.md`.
5. Use status `proposed` until the user or maintainers accept the decision.
6. State:
   - Context and forces, including compatibility and operational constraints.
   - The decision and its explicit boundaries.
   - Positive and negative consequences.
   - Serious alternatives and why they were rejected.
6. Link or supersede older ADRs instead of silently contradicting them.

## Keep the decision useful

- Write the invariant or policy another contributor must preserve.
- Avoid code listings unless they clarify an interface or format.
- Avoid predicting modules or functionality unrelated to the decision.
- Identify migration or compatibility work when the decision changes existing
  public or persisted behavior.
- Keep implementation in the same PR only when the decision is accepted and the
  user requested implementation.

## Validate

Check filename numbering, template completeness, links to affected documents,
and consistency with accepted ADRs. Summarize the decision and unresolved
questions in the handoff. Report the skill-evolution outcome required by
`AGENTS.md`; update reusable guidance exposed by the decision workflow. After
merge, remove the clean task worktree, prune worktree metadata, and delete the
merged local branch.
