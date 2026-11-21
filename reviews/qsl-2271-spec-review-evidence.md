---
id: SR-202611210022442313144108552
title: "spec-review/evidence review of FR-831 verification methods"
type: SpecReview
analysis: evidence
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md Acceptance Criteria; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. This checks the declared verification of each FR-831 AC (all `Test (TC-2113)`) against what TC-2113 can actually witness. The schema decisions (AC-6), the six-form packages (AC-3, half) and vector consistency are executable now. Reader outcomes (AC-1, AC-2, AC-4, AC-5, AC-7) are honestly disclosed as unexecuted until a reader exists. Test is the right method for them once that reader lands.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-831-AC-8 states a producer property ("adding duplicate authored clauses leaves every node, canonical triple, node id ... unchanged"), but TC-2113 builds both invariance packages from one package with two member edits, so equality holds by construction and no producer is exercised. AC-8 needs producer (FSL lowering) evidence, or should be re-scoped to the member's position outside preimages. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md FR-831-AC-8; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md Step 5 |
| FND-002 | medium | FR-831-AC-1 obliges a consumer to refuse with `unavailable_correlation_provenance` and never proceed with empty premises, but TC-2113 has no consumer step or vector for it. The verification names no consumer-side test, so that clause has no planned evidence. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md FR-831-AC-1; spec/test-cases/TC-2113-protocol-correlation-occurrences-join-the-control-occurrences.md Steps 3-5 |
| FND-003 | low | FR-831-AC-9's first clause ("no `SourceOwner` coherence applies to a row") is a non-behaviour with no observable outcome. Only its borrowed-entry, dependency and second-occurrence clauses are testable (and covered by CO-P10, CO-R08 and CO-R21). | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md FR-831-AC-9 |

## Verdict

Changes requested on the two medium items.

## Dispositions

Round 1, head ce96a4235251d09d6b4c58d1e13e9c2a4d9ed679.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-8 is now explicitly the producer invariance over real source variants (once, repeated, reversed), with planned TC-2113 step 5 run by the FSL producer. The by-construction static pair is split out as AC-11 and labelled as not the producer step. |
| FND-002 | fixed | The consumer clause is now AC-12, with planned TC-2113 step 6 and concrete `consumer.cases` CO-K01 to CO-K04 (refuse on absent over nonempty; proceed over empty related; clauses from a complete table; present []). The FSL guard is retained, and AC-1 no longer carries the consumer clause. |
| FND-003 | fixed | AC-9 now states only observable outcomes: the whole borrowed entry under the full key, the dependency's own member and entry, and a second declaration occurrence refusing at `occurrence`. The no-SourceOwner prose stays in the context section. |
