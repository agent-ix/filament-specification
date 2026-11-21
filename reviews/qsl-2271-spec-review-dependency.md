---
id: SR-202611210022442313144108549
title: "spec-review/dependency review of FR-831 and TC-2113 edges"
type: SpecReview
analysis: dependency
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md relationships; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md relationships; FR-831 Dependencies"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. New edges:
- FR-831 specifies FR-322 and FR-466, and references FR-464 and FR-822.
- TC-2113 verifies FR-831, FR-322 and FR-466.

All targets exist and the edge types fit: FR-831 refines carrier and layout owners and only reads FR-464 order and FR-822 `recommend`.

The enablement work is bundled in the same PR by parent decision, with no split needed: the FR-466 schema fix of the receive/attempt/effect/event `related` term and the `recommend` layout and operator spelling. The FR text records it as enablement ("before this contract the schema admitted a nonempty `related` only on `send`").

The cross-repo prose dependencies are explicitly not restated: FSL FR-731 and FSL lowering as the producer. The external sequencing is not a spec edge and is not a defect: the QSL-2743 reader lands before this contract merges, and the producer pin follows the reviewed contract and reader. The missing IR FR-358 Dependencies entry is recorded in the base review only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found (placeholder) | - |

## Verdict

Clean.
