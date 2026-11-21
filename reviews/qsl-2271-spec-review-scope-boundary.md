---
id: SR-202611210022442313144108551
title: "spec-review/scope-boundary review of FR-831 producer, reader and consumer allocation"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md Boundaries; FR-322-AC-514; FR-466-AC-20"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. Responsibilities are allocated cleanly:
- The FSL producer owns capturing every authored clause before deduplication, with its exact Identifier spelling and actual canonical position.
- FSpec owns the wire member, the schema and the reader consistency checks.
- The reader owns the typed read and the charges.
- Consumers own the named `unavailable_correlation_provenance` guard; strict admission still admits an absent member.
- FSL FR-731 owns dynamic source-order replay, which is explicitly not executed by the static vectors.
- IR owns the budget.

The member is outside identity preimages, `identityRef`, `source_map`, `protocol_locations` and old byte invariants (FR-322-AC-473 scoped). The contract claims no implementation, pin or runtime integration. The FR-466 schema enablement is in scope by parent decision. The FR-358 ownership attribution is recorded in the base review only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found (placeholder) | - |

## Verdict

Clean.
