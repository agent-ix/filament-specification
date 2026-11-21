---
id: SR-202611210022442313144108546
title: "spec-review/ears-conformance review of the FR-831 requirement statement"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md (Description, Typed read)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. This applies the EARS conformance check to the new FR-831 statement and to its normative SHALL sentences in Behavior and Typed read. The Description is an event-driven optional clause ("When a producer hands off ... it MAY publish") followed by two reader obligations. Each obligation has a clear trigger and response, and the negative ("SHALL NOT treat the absence as an empty provenance") is explicit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-831 Description is one compound statement with three actors: a producer MAY, a receiving reader SHALL, a non-receiving reader SHALL. Typed read adds a fourth, a consumer SHALL refuse. That makes the producer option, the reader contract and the consumer guard hard to trace atomically. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:20-27, spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:218-222 |

## Verdict

Passes, with one low item. The statements are testable and unambiguous in trigger and response.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | After the actor split, `quire validate` (rc=0) still warns on the FR-831 Description: `[ears:non-singular]` at line 32 (the receiving-reader bullet holds two SHALLs) and `[quality:mixed-modal]` at line 29 (the producer bullet mixes MAY and SHALL in one statement). Each actor's obligations are still not one requirement per SHALL. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:27-40 |

## Dispositions

Round 1, head ce96a4235251d09d6b4c58d1e13e9c2a4d9ed679.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Description now states four separately segmented actor obligations (producer, receiving reader, non-receiving reader, consumer) in one FR, with no scope weakened. The producer's capture duty is now a normative SHALL. |
| FND-002 | still-open | New this round. Split each bullet into single-SHALL statements (for example, the producer MAY publish; when it publishes, it SHALL record ...), or accept the lint warning with a stated reason. |

## Dispositions (round 2)

Round 2. Head 7e8fcaf566cd05bb24068b0e26403816a8034086 (fix 7e8fcaf5 on top of a merge of origin/main 9add8a0f), local == remote.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The FR-831 Description is now nine statements, each with one actor, one condition and one modal. They keep the optional wire availability (MAY), the producer's complete pre-deduplication capture with exact Identifier alias and canonical position (SHALL), reader validation and typed read, reader absence reporting and no empty-absence treatment, and the consumer refusal with no empty or guessed premises. `quire validate --scope .` on FR-831 and TC-2113 gives rc=0 and 0 ears/quality warnings. Nothing was weakened. |
