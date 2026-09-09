# Project instructions

## Start each task

1. Inspect the current branch, HEAD, `git status --short`, and relevant diff. Preserve changes made by the user or other agents.
2. Read [docs/status.md](docs/status.md), [ROADMAP.md](ROADMAP.md), and the relevant entry in [docs/backlog.md](docs/backlog.md).
3. For implementation, read the affected parts of [docs/architecture.md](docs/architecture.md) and [docs/validation.md](docs/validation.md). Read [docs/project-plan.md](docs/project-plan.md) when product scope matters.
4. State the bounded task, intended files, and smallest sufficient validation. Continue routine authorized work without repeatedly asking for approval.

## Product contract

- The initial scope is TSX, direct named imports, file-local alias/shadowing analysis, explicit JSX attribute renaming, and literal string value mapping.
- Use one Oxc-based Rust analysis core for the initial constrained operations. Do not implement a new parser, generic pattern language, complete typechecker, or cross-file export graph without a justified design change.
- Initial `moduleSpecifier` matching means the import declaration's literal module source. Do not claim it proves the final resolved package or physical module.
- Use the machine status protocol in `docs/architecture.md`: `ready`, `not_matched`, `skipped`, `conflicted`, `invalid`. UI labels may translate these as 可改写、未匹配、跳过、冲突、无效. Preserve concrete reasons; unsupported or uncertain cases must not silently produce edits.
- Preserve unrelated source bytes. Keep all core edits in UTF-8 byte ranges; convert to editor positions only at the UI boundary.
- Analysis and preview use immutable file snapshots and versioned requests. Late results cannot replace current results.
- Alpha exports patches. Direct application belongs to M3 and requires content checks and a separate validated write path.
- Parsing or typechecking success is not proof of arbitrary behavior preservation. Migration rules describe user-selected changes, not guaranteed semantic equivalence.

## Work boundaries

- This repository begins with planning documents only. Inspect actual manifests and scripts before claiming a command exists or a component is implemented.
- Runtime dependency versions and toolchains must be selected, pinned and tested in RWB-001. Do not install unrelated global tools or change user-level configuration.
- Use synthetic or appropriately licensed public fixtures. Do not copy private repositories, local credentials, absolute personal paths, chat transcripts, or user-specific Codex configuration into the repository.
- Keep changes focused. Never use broad staging in a dirty tree, destructive reset/clean, force-push, or overwrite another agent's work.
- Use explicit file lists for commits. Follow the current user's authorization for commits, pushes, PRs and releases; a backlog item is not blanket authorization to publish future work.
- Do not contact people, post announcements, start recurring automations, or publish packages as an incidental part of implementation.

## Validation and reporting

- For documentation-only changes, inspect the diff, local references, and instruction consistency. Do not run project builds merely because validation layers are listed.
- For code changes, run the meaningful checks for affected behavior. Reuse valid results; stop when the required checks pass unless a new failure or uncovered risk requires more work.
- Keep these evidence levels distinct: design, implemented source, fixture tests, end-to-end local behavior, packaged binary, user study, and public growth observations.
- Never claim benchmark, adoption, Star growth, or usability results without recorded observations. Do not invent speed multipliers.
- At a milestone boundary, update `docs/status.md` with concrete evidence, unresolved questions, and the next task. Keep it concise; link detailed evidence instead of copying logs.
- Run worktree or independent-agent reviews only when authorized by the user or project workflow. Give each writer explicit file ownership, and serialize Git changes and external publication through one owner.

## Code review rules

Flag wrong-symbol edits, unsupported cases that silently change code, stale snapshots, overlapping edits, Unicode offset errors, excessive formatting churn, and any mismatch between preview and output. Confirm regression fixtures test these contracts instead of mirroring implementation details.

## Continuation

Use [docs/agent-workflow.md](docs/agent-workflow.md) for task handoff. New conversations should be able to continue from repository documents and GitHub issues without depending on previous chat history.
