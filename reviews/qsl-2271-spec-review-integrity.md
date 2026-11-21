---
id: SR-202611210022442313144108547
title: "spec-review/integrity review of FR-831 behavior, ACs and the FR-322/FR-466 deltas"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md; FR-322-AC-514; FR-466-AC-20; proposals/checked-package-v2/schema.json"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. This checks FR-831 Behavior against AC-1 to AC-9, FR-322-AC-514, FR-466-AC-20, the FR-322 field table rows, the schema object and TC-2113, for completeness, consistency and atomicity.

The following are consistent across all of them:
- the closed five-member row;
- absent means unavailable; present means complete; `[]` only when every `m` is 0;
- the order: row charge, then the joins, then the key with equal before order, then the ordinal, then the view, then coverage;
- the cost of each step;
- the loci;
- the schema-invalid list against the actual `Identifier` pattern (`^[A-Za-z_][A-Za-z0-9_]*$`), which is the existing definition and not a narrower one;
- the Occurrence roles (six);
- the 9007199254740991 bounds.

The scan order, ascending structural node-id order, equals the row key order, so coverage can merge-walk groups against nodes without an extra index.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The key step orders by NodeRef "then `role`, `ordinal`, `source_ordinal`" but defines no order for `role` strings. Join 3 already forces `declaration`, so the order is unreachable, but the key is underspecified as written. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:151-157 |

## Verdict

Passes, with one low item.
