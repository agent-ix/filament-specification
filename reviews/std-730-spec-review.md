---
id: SR-2026112515500002
title: "Spec review: PR 846 claim-node literals in FR-322, FR-429, FR-438, TC-431"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; git diff 7de9a209...500439ee: spec/objects/interfaces/FR-322-checked-package-artifact.md (FR-322-AC-276), spec/functional/protocol/FR-429-report-protocol-deadlocks.md (FR-429-AC-17), spec/functional/memory/FR-438-report-data-races.md (FR-438-AC-9), spec/test-cases/TC-431-backend-provider-wire-vectors.md (BP-51)"
review_set: subset
---

## Summary

Ticket: STD-730. Each of the four edits replaces the quoted protocol-clause claim node `{package: "example", path: ["Example", "Handshake"], ordinal: 0}` with `{..., path: ["Example", "Handshake", {"node": 44}], ...}`. The new literal is the document-held form that FR-322 requires: index segment `{"node": i}` and never an operand slot (FR-322-AC-278, "Operand index segments"). It equals the claim in `derived-items-request` items 5 and 6, and node 44 of the package that vector is read against (positive-all-families) is the owner model `acme/orders` `model`/`object_type` Order, which the clause's own operand slot resolves to. Each AC already names `derived-items-request`, so the index has a stated frame. No other sentence in any of the four ACs changes, and no other spec file quotes the old two-segment id. The quire log (`STD-730-quire-validate.log`) exits 0. Its two EARS warnings are at FR-322 line 122, which this PR does not touch.

Sub-analyses: only the base consistency check applies, because the edit changes one literal and no requirement statement, structure, object or relationship.

## Verdict

**PASS**. The ACs agree with the vectors, with FR-322's identity rules and with each other: FR-429-AC-17 and FR-438-AC-9 still name the same claim node.

## Assurance Context

No applicable AssuranceProfile was found under `spec/`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

- FR-322-AC-276 (examined, clean)
- FR-429-AC-17 (examined, clean)
- FR-438-AC-9 (examined, clean)
- TC-431 BP-51 (examined, clean)
- FR-322 "Protocol clause owner identity" and "Operand index segments", FR-322-AC-278 (context only)
