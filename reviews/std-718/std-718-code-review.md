---
id: SR-202611210022442313144557
title: "Code review — STD-718 randomized scheduler vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json, spec/objects/interfaces/FR-413-probabilistic-counterexample.md, tests/backend_provider_v1.rs"
review_set: subset
---

## Summary

Reviewed the STD-718 schema and vector changes against their existing schema qualification test. The new reader-refusal vectors are intentionally for the future contract-ir reader, but AC-12 currently presents schema and reader refusals as one runtime outcome and is not trace-bound to TC-431.

## Verdict

**CONDITIONAL** — AC-12 needs to distinguish schema admission refusal from replay-reader refusal and bind its schema vector case to existing evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-12 conflates schema rejection with replay-reader refusal and has no test binding | spec/objects/interfaces/FR-413-probabilistic-counterexample.md:240; proposals/backend-provider-v1/schema.json:874; tests/backend_provider_v1.rs:535,773 |

## Finding Details

### FND-001 — AC-12 conflates schema rejection with replay-reader refusal and has no test binding

The single-choice vector is in `adverse`, so the Draft 2020-12 schema rejects it at `choices.minItems` before a native reader runs. The other two invalid vectors are in `reader_refusals` and are schema-valid, where replay checks exact sum and distinct choices. AC-12 currently assigns `invalid_runtime_input`/`invalid-value` to all three cases, without stating which layer refuses the single-choice case. Also `quire matrix --scope . --format tsv` reports the new FR-413-AC-12 as `untagged`; the existing TC-431 positive/adverse vector test trace tags do not include it. Separate the schema refusal from reader refusals and provide the trace binding/evidence for the schema-level case.

## Scope

- FR-413-AC-12 — examined: “A randomized witness entry with two distinct choices at probabilities `1/3` and `2/3` is admitted; the corresponding entry with one choice, with probabilities `1/3` and `1/3`, or with the same choice twice at `1/2` each is refused `invalid_runtime_input`/`invalid-value` at the `choice` member.”
- FR-413 `WitnessScheduler` paragraph — context only: randomized-choice wrapper and replay-owned invariants.
- `proposals/backend-provider-v1/schema.json` `ProbabilisticCounterexample.scheduler.entries[].choice` — examined: randomized branch has `choices.minItems: 2`; descriptions assign exact sum and distinctness to replay.
- `proposals/backend-provider-v1/vectors.json` positive, adverse, and reader_refusals entries named `fr413-randomized-witness-scheduler*` — examined.
- `tests/backend_provider_v1.rs` `tc_431_positive_vectors_are_admitted_with_owned_members` and `tc_431_adverse_vectors_are_refused` — examined; their trace tags do not include FR-413-AC-12, and they do not iterate `reader_refusals`.


## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AC-12 now describes schema admission; AC-13 owns the single-choice schema refusal; AC-14/15 own schema-valid replay-reader refusals. TC-431 now traces AC-12 and AC-13, and the matrix reports both tagged. |
