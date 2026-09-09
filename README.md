# Rewrite Workbench

**Turn a small code change into a repeatable, reviewable project-wide rewrite.**

[中文项目计划](docs/project-plan.md) · [Roadmap](ROADMAP.md) · [Architecture](docs/architecture.md) · [Current status](docs/status.md) · [Agent entry point](AGENTS.md)

> **Status: project planning and repository bootstrap.** There is no installable CLI, working rewrite engine, or released application yet. The capabilities below are planned; follow the roadmap and issues for implementation evidence.

Rewrite Workbench is a planned local tool for frontend developers who need to update repeated code without learning an AST query language. Select an example, specify the intended change and scope, inspect matches and exclusions, and export a reviewable patch.

The first target is **TypeScript React component API changes**:

- Rename an explicit JSX attribute on a component imported from a specified module.
- Map a specific string attribute value to a new value.
- Account for local import aliases and shadowed names.
- Explain excluded or unresolved candidates instead of silently changing them.
- Preserve unrelated source text and save versioned rewrite recipes with examples.

## Design direction

The planned implementation uses a Rust core with Oxc for parsing and file-local symbol analysis, plus a React/TypeScript/Monaco interface served by a local CLI. The initial alpha exports patches. Guarded direct application, distributed binaries, and a browser playground follow in later milestones.

Scanning multiple files does **not** imply full cross-file semantic resolution. Initial module selection matches the literal source of an import declaration. Type inference, re-export tracing, framework-wide migrations, and arbitrary transformation synthesis are outside the first release scope.

## Why another tool?

[ast-grep](https://ast-grep.github.io/), [Codemod](https://docs.codemod.com/), and [WebStorm structural search and replace](https://www.jetbrains.com/help/webstorm/structural-search-and-replace.html) already offer substantial matching, rewriting, and preview capabilities. This project tests a narrower product hypothesis: **can ordinary frontend developers complete a correct rewrite with less setup and rule-authoring effort?**

That hypothesis has not yet been validated. See the [competitive baseline](docs/research/competitive-baseline.md) for known overlap and the planned comparison.

## Start here

| Need | Document |
| --- | --- |
| Product scope, users, delivery and decisions | [Project plan](docs/project-plan.md) |
| Milestones, dependencies and exit criteria | [Roadmap](ROADMAP.md) |
| Small implementation tasks | [Backlog](docs/backlog.md) |
| Modules, data contracts and correctness boundaries | [Architecture](docs/architecture.md) |
| Fixtures and validation strategy | [Validation](docs/validation.md) |
| Continue in Codex or another coding agent | [Agent workflow](docs/agent-workflow.md) |
| Actual progress and next task | [Status](docs/status.md) |
| Contribution process | [Contributing](CONTRIBUTING.md) |

Do not use installation commands copied from future roadmap items: packages have not been published. Initial implementation task **RWB-001** will add and verify the actual development commands.

## License

[MIT](LICENSE). Third-party dependencies retain their own licenses and notices.
