# Synthetic fixtures

All inputs in this directory were written for Rewrite Workbench and are covered by the repository's MIT license. `@example/ui`, `@example/other`, and `./barrel` are synthetic import literals, not dependencies to install or resolve. No target-project scripts are executed.

## RWB-001 binding spike

Each `spike/<case>/` contains:

- `input.tsx`: exact UTF-8 source bytes. Do not normalize its BOM, line endings, or final newline; `.gitattributes` disables Git text conversion for these inputs.
- `expected.json`: manually specified file status, reason, and opening-tag binding observations in source order. Every span is a half-open UTF-8 byte range into `input.tsx`, including declaration spans. Raw Oxc IDs are not persisted.

The fixture runner compares the entire expected structure. Error fixtures additionally require a nonempty diagnostic and no partial bindings; diagnostic prose is not part of the golden contract. There is no recipe or rewritten output yet because this spike performs no rewrite. RWB-003/004 will add those artifacts for edit and operation fixtures.

| Case | Expected evidence / regression risk |
| --- | --- |
| `direct` | Paired JSX element binds to its named import; only the opening tag is observed. |
| `alias-shadowing` | Outer and later `B` bind to the import; the parameter-shadowed `B` binds to a different declaration. |
| `block-shadowing` | A nested function captures the block-local `B`; leaving the block restores the import binding. |
| `literal-sources` | Three same-export-name imports retain their distinct literal sources; no barrel resolution is inferred. |
| `unicode-crlf-bom` | Chinese text, emoji, a non-ASCII identifier, BOM, CRLF, and no final newline retain exact byte spans. |
| `unresolved-and-intrinsic` | DOM, unresolved component, and member-expression tags cannot be mistaken for named-import evidence. |
| `parse-error` | Malformed TSX invalidates the input. |
| `recoverable-parse-error` | A recovered mismatched closing tag still invalidates the input. |
| `invalid-regex` | Regex syntax errors are reported with regex parsing explicitly enabled. |
| `semantic-error` | Duplicate lexical declarations are rejected by Semantic syntax checks. |

Successful analysis has `status: "skipped"` and `reasonCode: "binding_spike_only"`. `named_import` is an observation, not a `ready` rewrite candidate. `other_binding` deliberately combines local declarations and imports outside the spike's index. Complete candidate reasons and unsupported-import classification belong to RWB-002.

From the repository root, run `cargo test --workspace --locked`. Detailed environment and execution evidence is in [RWB-001 validation](../docs/evidence/rwb-001.md).
