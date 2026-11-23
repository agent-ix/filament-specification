---
id: SR-202611210022442313144558
title: "Base spec review — STD-718 FR-413 scheduler shape"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-413-probabilistic-counterexample.md, proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json"
review_set: subset
---

## Summary

Reviewed the changed FR-413 statement, its acceptance criterion, schema branch and published vectors against the base requirements checklist. The wire form is concrete, but AC-12 merges two refusal layers and has no computed test binding.

## Verdict

**CONDITIONAL** — clarify the refusal layer in AC-12 and connect its schema case to a test binder.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-12 conflates schema rejection with replay-reader refusal and is untagged in the computed matrix | spec/objects/interfaces/FR-413-probabilistic-counterexample.md:240 |

## Finding Details

### FND-001 — AC-12 conflates schema rejection with replay-reader refusal and is untagged in the computed matrix

The one-choice case is rejected by the schema's `minItems: 2` constraint, while the bad-sum and duplicate-choice cases are schema-valid and belong to replay's semantic reader checks. The current criterion attributes the same `invalid_runtime_input`/`invalid-value` result to all three. The computed matrix marks FR-413-AC-12 `untagged`; the published vector data is not itself a trace binder. Distinguish the schema-level refusal from reader-level refusals and bind the schema-level case to its qualifying test.

## Scope

- FR-413-AC-12 — examined: the single positive case and three refusal cases.
- FR-413 randomized `WitnessScheduler` paragraph — examined: wrapper spelling, minimum cardinality, exact rational range, sum, distinctness and replay ownership.
- Schema randomized `choice` branch — examined: object shape, `minItems`, rational type, and open/closed member boundaries.
- The positive vector, single-choice adverse vector, and the two `reader_refusals` vectors — examined for alignment to the FR and schema.
- Computed trace matrix — examined for the new criterion; status is `untagged`.


## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The revised ACs separate schema rejection from replay-reader refusal. TC-431 now binds the positive and single-choice schema cases; the matrix reports AC-12/13 tagged, while AC-14/15 are explicitly Inspection criteria. |
