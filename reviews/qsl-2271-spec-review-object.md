---
id: SR-202611210022442313144108548
title: "spec-review/object-review of the ProtocolCorrelationOccurrence row and its cross-references"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md; proposals/checked-package-v2/schema.json $defs.ProtocolCorrelationOccurrence; spec/objects/interfaces/FR-322-checked-package-artifact.md field tables"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. This is an object review of the new row object and its cross-references:
- The schema `ProtocolCorrelationOccurrence` is closed, with five required members.
- `control` reuses `NodeRef`, `occurrence` reuses `Occurrence` and `alias` reuses `Identifier`, so no new narrower lexical rule is introduced.
- The two integer bounds match the FR text.
- FR-322 gains matching field-table rows for the optional member, next to `claim_names`.
- The README's row spelling matches.
- The source context is the existing `source_map` entry under its existing key `(node_id, role, ordinal)`. No SourceOwner or new source identity is invented, and every region is borrowed.
- The member is outside the identity sidecar and every preimage.
- The relationships to FR-322, FR-466, FR-464 and FR-822 resolve. The fixed names `protocol_correlation_occurrences` and `canonical_index` are used throughout.

The missing IR FR-358 owner is recorded once, in the base review (SR-202611210022442313144108545 FND-001), and is not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found (placeholder) | - |

## Verdict

Clean.
