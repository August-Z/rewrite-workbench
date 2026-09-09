# Rewrite Workbench

**Turn a small code change into a repeatable, reviewable project-wide rewrite.**

[中文项目计划](docs/project-plan.md) · [Roadmap](ROADMAP.md) · [Architecture](docs/architecture.md) · [Current status](docs/status.md) · [Agent entry point](AGENTS.md)

> **Status: M1 correctness foundation, RWB-001 implemented and locally validated.** A Rust core and development CLI can parse TSX and inspect file-local bindings. Rewrite operations, patches, UI, and installable releases remain planned. See the [validation record](docs/evidence/rwb-001.md) for the tested scope.

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
| GitHub issues and milestones | [Tracking links](docs/github-tracking.md) |
| Modules, data contracts and correctness boundaries | [Architecture](docs/architecture.md) |
| Fixtures and validation strategy | [Validation](docs/validation.md) |
| Continue in Codex or another coding agent | [Agent workflow](docs/agent-workflow.md) |
| Actual progress and next task | [Status](docs/status.md) |
| Contribution process | [Contributing](CONTRIBUTING.md) |

## Run the development spike

Prerequisites: [rustup](https://rust-lang.org/tools/install/) on PATH and the platform's native linker/build tools. The current validation environment is macOS arm64 with Xcode command-line tools. Other platforms and distribution packages have not been validated.

From the repository root:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy --no-self-update
cargo fetch --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run --locked -p rewrite-cli -- inspect fixtures/spike/alias-shadowing/input.tsx
```

[rust-toolchain.toml](rust-toolchain.toml) selects Rust 1.98.1 for this repository without changing your default toolchain. [Cargo.toml](Cargo.toml) pins Oxc 0.149.0; [Cargo.lock](Cargo.lock) locks the dependency graph. There are no frontend dependencies yet. If rustup is installed but absent from PATH, add its `bin` directory to this shell's PATH before running these commands; no shell-profile edits are required.

The `inspect` command reads exactly one `.tsx` file and prints JSON from the shared core. For the alias fixture, the first and last `B` refer to the named import; the middle `B` refers to a function parameter. Valid inputs report `skipped / binding_spike_only` because no rewrite operation has been evaluated. Observations include literal import sources and UTF-8 byte spans; they do not resolve physical modules or expose persistent Oxc IDs.

To inspect an intentional syntax error:

```sh
cargo run --locked -p rewrite-cli -- inspect fixtures/spike/parse-error/input.tsx
```

This prints `invalid / parse_error` with diagnostics and exits **1**. Completed inspections exit **0**; argument, file-read, encoding, or output errors exit **2**. No target files are written. See [fixture conventions](fixtures/README.md) and [execution evidence](docs/evidence/rwb-001.md). Packages have not been published; these are source-development commands.

## License

[MIT](LICENSE). Third-party dependencies retain their own licenses and notices.
