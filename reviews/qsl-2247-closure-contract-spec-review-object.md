---
id: SR-2026112115155546054652124
title: "spec-review/object-review of the TemporalCounterexample model-arm step after closure-stutter"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md (wire form, FR-364-AC-2, model-arm step); spec/functional/temporal/FR-362-apply-fairness-constraints.md; spec/objects/interfaces/FR-400-hyper-counterexample.md; QSL-2247; branch at 7537ca96, PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
---

## Summary

Ticket: QSL-2247. I checked the `TemporalCounterexample` model-arm step object and every place that enumerates its transition forms.

Clean:
- The FR-364 model-arm step paragraph now lists three forms.
- FR-400 `HyperStep` is a separate object with its own `{"type":"step"}` shape, not `$defs.ModelStep`, and hyper replay never uses the closure. It is unaffected.
- FR-361 owns the terminal stutter, and its text is unchanged and still correct.
- FR-368 and FR-380 define the closure stutter as "belonging to no fairness constraint". FR-364's step 4 rule restates that consistently.

## Verdict

**CHANGES REQUESTED.** FR-364-AC-2 still enumerates the model-arm step forms without `closure-stutter`, and it requires an unlisted form to refuse before replay.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-364-AC-2 says "each model-arm step carries its FR-181 transition identity or the terminal-stutter identity and its post-state key; … A missing, extra or mistyped member … refuses before replay". `{"type":"closure-stutter"}` is neither an FR-181 transition identity nor the terminal-stutter identity. Under AC-2 a closure-stutter step is therefore a mistyped member that refuses `malformed_wire` before replay. That contradicts the new wire paragraph, the schema alternative, and AC-25's positive (`reproduced-with-evaluated-witness`). One implementation cannot satisfy both ACs. Fix: add "or the closure-stutter identity" to AC-2. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:565,101-103,585 |
| FND-002 | low | FR-362 "Enabled and taken" is the owner of what a step takes, and FR-364 Dependencies says FR-362 owns the lasso fairness rules. FR-362 names the terminal stutter step ("takes no constraint") but not the closure-stutter step. FR-364 step 4's rule is a correct derivation, but the owning requirement does not state it. Add one line to FR-362 in the FSpec follow-up, or record in FR-364 that the rule is derived from FR-368/FR-380. | spec/functional/temporal/FR-362-apply-fairness-constraints.md:87-101; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:506-508 |

## Scope

- FR-364 wire form table and model-arm step paragraph (examined, clean)
- FR-364-AC-2 (examined; FND-001)
- FR-362 Enabled and taken (examined; FND-002)
- FR-361 Terminal states (context_only)
- FR-400 HyperStep (examined, clean)
- FR-379 RefinementFailure prefix reuse of ModelStep (context_only; SR-2026112115155546054652120 FND-001)

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-2 now lists 'the FR-181 transition identity, the terminal-stutter identity or the closure-stutter identity'. It no longer contradicts AC-25 or AC-26. |
| FND-002 | fixed | FR-362 'Enabled and taken' now says the closure stutter step takes no constraint and satisfies none, and that its state's enabledness is the model state's own. FR-364 step 4 cites FR-362. |
