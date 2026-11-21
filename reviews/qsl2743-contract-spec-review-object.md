---
id: SR-2026112106460400000015
title: "spec-review/object-review of FR-830 row objects and cross-references"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-830-carry-selected-model-union-witness-rows.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; QSL-2743; PR 789"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: references
---

## Summary

Ticket: QSL-2743. PR agent-ix/filament-specification#789 at fb65d9db. This object review (method spec-review/object-review; the schema's `analysis` enum has no object value, so it is `base`) covers the domain objects FR-830 introduces or reuses and their cross-references.

Each object was checked as defined and closed:
- The model-union row `{structural_node, selected_document, type_handle, settled_key, cases}`.
- The case `{rank, position, name, original_case, member, payload}`.
- The three payload forms.
- The model-tuple row, which is exactly the existing `SelectedTupleDeclarationRow` fields `{structural_node, selected_document, original_identity, node_key}` (producer/research evidence: IR exact_projection).
- The node reference (CheckedNodeId), the original node key, the original value type and the declaration key.

The two sort keys are stated (rows by structural_node; cases by UTF-8 name). Each identity field names its source (owner identity, owner node, `VariantRecord.key`, settled declaration). Cross-references to FR-322, FR-440, FR-441 and FR-610 resolve. FR-322 points back to FR-830, and the cross-repository owners (filament-contract-ir FR-352/FR-358, the FSL identity file) are named without restating them. No object duplicates an existing one, and no enum or case node is invented.

## Verdict

**PASS.** No object-level defect. The type_handle/settled_key distinction is defined correctly; the vectors that should isolate it are missing (SR-2026112106460400000013).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

- FR-830 model-union row, case, payload objects (examined, clean)
- FR-830 model-tuple row (examined, clean)
- FR-830 encodings: node reference, original node key, original value type, declaration key (examined, clean)
- FR-830 Dependencies cross-references (examined, clean)
- FR-322 model union paragraph (context_only)
