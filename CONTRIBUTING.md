# Contributing

The repository is currently in its planning/bootstrap phase. There is no released application or established runtime build command yet.

## Start with a bounded task

Read [AGENTS.md](AGENTS.md), [current status](docs/status.md), and the relevant [backlog item](docs/backlog.md). Keep changes focused on the current milestone. Open an issue with a concrete code example before proposing a broad new rewrite category.

## Propose a rewrite

Include a minimal input, intended output, cases that must remain unchanged, and cases that cannot safely be decided. Use synthetic or appropriately licensed public code. Do not include credentials, private repository content or personal information.

## Validate changes

For documentation changes, check the diff, local links and consistency. For implementation changes, use the actual commands established in the repository and the affected contracts in [validation](docs/validation.md). Avoid unrelated builds and repetitive checks once the required checks pass.

Add a regression fixture for wrong-symbol edits, unsupported syntax or snapshot conflicts. Do not claim correctness from an attractive preview alone.

## Pull requests

Explain the user-visible change, supported scope, validation and remaining limitations. Preserve unrelated changes and avoid mass formatting. Update status or architecture when the effective contract changes.

Contributions are made under this repository's [MIT license](LICENSE). Preserve notices for third-party code and dependencies.
