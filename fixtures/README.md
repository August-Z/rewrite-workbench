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

Successful analysis has `status: "skipped"` and `reasonCode: "binding_spike_only"`. `named_import` is an observation, not a `ready` rewrite candidate. `other_binding` deliberately combines local declarations and imports outside the spike's index. RWB-002 adds a separate classifier below; these spike observations remain unchanged and are not promoted into candidates.

From the repository root, run `cargo test --workspace --locked`. Detailed environment and execution evidence is in [RWB-001 validation](../docs/evidence/rwb-001.md).

## RWB-002 binding classification

Each `bindings/<case>/` contains original synthetic `input.tsx` bytes and an independently authored `expected.json` with a selector and the complete expected per-tag classification/evidence (except prose messages). Exact tag, local declaration and import declaration spans are checked. The test runner requires readable messages but keeps their wording outside the golden contract. `.gitattributes` also protects these inputs from line-ending conversion.

| Case | Expected evidence / regression risk |
| --- | --- |
| `direct-sources` | Direct/aliased and quoted named imports; imported name vs local name; distinct module, barrel and subpath literals; parsed escape equivalence; local export does not erase an import binding. |
| `shadowing` | Destructured parameter, block binding before declaration, nested capture, var hoisting, catch parameter, named function expression, loop binding, class and scope exit. |
| `unsupported-imports` | Default including named default syntax, namespace, TS import-equals, source/defer phases and nested JSX members never confirm a component. |
| `indirection` | Wrapper, assignment, destructuring, require and later assignment never borrow the original import's identity. |
| `type-only` | Declaration/specifier/default/namespace type-only imports and local value declarations shadowing type imports; ordinary value alias still confirms. |
| `unresolved-and-reexports` | Named/star/namespace re-exports create no local value binding; DOM/custom tags ignore same-name imports; member/namespaced/this names remain unsupported. |
| `unicode-crlf-bom` | Chinese/emoji/combining character, non-ASCII alias, BOM, CRLF and no final newline; exact label and declaration byte positions. |
| `merged-bindings` | Import and local/type declarations can share one Oxc SymbolId without syntax errors; reject merged symbols in either declaration order, preserving all declaration spans. |
| `no-jsx` | Re-export declarations, comments and JSX-looking strings produce zero usage results. |

Additional core tests vary the selector to reject alias-as-export, path normalization, case differences and selected default exports. Invalid-input tests place a valid matching JSX before malformed/recoverable JSX, invalid regex or duplicate lexical declarations and require whole-file rejection.

`bindingConfirmed` refers only to the selected declaration in this input. All confirmed cases are `skipped / operation_not_evaluated`; all negative fixtures assert false plus their specific exclusion/skip reason. No fixture claims operation correctness or generates edits. CLI tests independently check selector forwarding, invalid input, argument/encoding rejection, and unchanged Unicode/BOM/CRLF input bytes.

Run `cargo test --workspace --locked`. See [RWB-002 validation](../docs/evidence/rwb-002.md) for results and remaining boundaries.
