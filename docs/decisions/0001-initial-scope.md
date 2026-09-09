# ADR-0001: Initial scope and core boundaries

Status: accepted for initial implementation · Date: 2026-09-09.

## Context

The project aims to make common frontend rewrites easier to perform and review. Existing tools already provide structural matching, code transformation, visual previews and semantic analysis. The initial product hypothesis concerns task completion cost, not an unoccupied feature category.

## Decisions

1. Target TSX component API changes first: explicit JSX attribute renaming and literal string value mapping.
2. Use one Oxc-based Rust analysis core with file-local binding analysis. Match the literal module source of direct named imports, including local aliases and shadowing.
3. Expose constrained operations through familiar conditions. Do not build a parser, complete TypeScript checker, generic matcher or arbitrary example synthesis in the initial milestone.
4. Preserve unrelated source bytes with immutable snapshot-bound edit plans. Keep uncertain or conflicting cases unchanged with explanations.
5. Use a local CLI and browser UI. Alpha exports patches; guarded direct writes follow as a separately validated path.
6. Keep source analysis independent of filesystem and UI so a later public WASM playground can reuse the core.
7. Keep public documentation honest about planned versus implemented behavior. User-study and growth evidence remain separate from engineering verification.

## Consequences

The first release covers fewer transformations than existing general codemod engines. This is acceptable only if it establishes a clearer workflow for its supported tasks. Arbitrary patterns would justify reconsidering ast-grep; cross-file resolution needs its own contract and fixtures.

External research can proceed alongside implementation. Lack of participant availability does not block foundational development, but no study result may be claimed without observations.

## Revisit when

- At least one real task cannot reasonably fit either initial operation.
- User trials show rule authoring still dominates the workflow.
- Maintaining constrained operations costs more than an established matcher plus adapters.
- Source resolution beyond direct import declarations is necessary for repeated real use cases.

See [architecture](../architecture.md), [competitive baseline](../research/competitive-baseline.md), and [roadmap](../../ROADMAP.md).
