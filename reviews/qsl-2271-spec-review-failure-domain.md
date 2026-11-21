---
id: SR-202611210022442313144108550
title: "spec-review/failure-domain review of FR-831 joins, identity and refusal order"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md (Source-unit context, Reader checks and refusal order, Typed read, Authority and invariance)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. This checks the identity, purity and edge cases of FR-831. Each of the following is closed explicitly:
- Identity confusion: the full control NodeRef plus the declaration occurrence is the source_map key. There is no lookup by ordinal alone, no fallback to the root or to another package, and no byte authority taken from a SourceRef.
- Foreign, dependency and absent controls, and nodes of the wrong form, operator, role or occurrence ordinal.
- A canonical index over an empty `related`.
- Duplicate and out-of-order keys.
- A gap in, or a nonzero start of, the ordinals.
- `[]` over a nonempty `related`, a group with no rows, and a dependency member that is missing or incomplete.
- Absent versus null versus `[]`: null is `malformed_wire`, and absent and present are distinct outcomes.
- Denial of a charge, and refusal of the read, are all-or-nothing, with no partial view.
- Repeated aliases or indexes, and two aliases of one record, stay deterministic under FR-731 source-first evaluation: semantics reads the canonical record, and the diagnostic names the alias of the first failing row.
- The existing Owns source replay is unchanged.
- The producer-only completeness limit (an omitted trailing duplicate, or the exact authored spelling) is stated as non-detectable, with no false reader claim.
- One declaration occurrence per control (FR-464) is grounded; the two-occurrence case is recorded as absent by standard instead of guessed.

Vector-level defects in the oracles of CO-R17 and of the coverage refusals are recorded in the code review (SR-202611210022442313144108544) and not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found (placeholder) | - |

## Verdict

Clean at the contract level.
