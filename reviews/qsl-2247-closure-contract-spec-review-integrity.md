---
id: SR-2026112115155546054652123
title: "spec-review/integrity of the FR-364 closure-stutter change"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md; spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md; QSL-2247; branch at 7537ca96, PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
  - target: ix://agent-ix/filament-specification/TC-295
    type: reviews
---

## Summary

Ticket: QSL-2247. I checked completeness and consistency between the FR-364 prose, the ordered replay steps, AC-25 and TC-295.

Clean:
- `quire validate --strict` passes on FR-364 and TC-295 (quire 0.52.0, engine 0.62.2).
- AC-25 is a new id. AC-24 was the previous maximum, so there is no collision.
- The TC-295 expected results 15-19 and 21-23 cite AC-25 (and AC-4/AC-5 where appropriate) with pointers that match replay step 2.
- The FR-368 dependency is stated in Dependencies.

## Verdict

**CONDITIONAL.** One medium: the delay rule is stated only in prose, with no outcome code, and TC-295 tests it without AC backing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The delay refusal is incompletely specified. The Closure stutter paragraph says "a `delay` member on it refuses at `/loop/0/delay`" but gives no outcome code. Replay step 2's list of closure-stutter refusals (FR-364:427-433), which does give codes, omits it. FR-364-AC-25 does not mention delay. Yet TC-295 step 20 and expected 20 test it and cite AC-25. The schema admits `delay` on every `ModelStep`, so the refusal is replay's, but TC-295 step 25's list of schema-admitted, replay-refused placements omits it. Nothing fixes where the check sits relative to the alphabet, prefix and terminal checks. Fix: add "a `delay` member refuses `invalid_runtime_input`/`invalid-value` at `/loop/0/delay`" to step 2's list and to AC-25, and add delay to TC-295 step 25's list. This uses the code of the existing AC-19 terminal-stutter delay refusal, so the step grammar stays consistent. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:312,427-433,585; spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md:57,70,112 |
| FND-002 | low | Body links without edges. FR-364's frontmatter `relationships` has no edge to FR-368, which the new text depends on and which Dependencies now lists, nor to FR-380, which the text cites. FR-380 is also written "FSpec FR-380", a self-repository prefix with no link, unlike every other FR reference in the file. Add `depends_on` FR-368 and `references` FR-380, and link FR-380 as `[FR-380](../analysis/FR-380-transfer-claims-through-refinement.md)`. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:1-28,311,594 |

## Scope

- FR-364 Closure stutter paragraph delay sentence (examined; FND-001)
- FR-364 Replay step 2 refusal list (examined; FND-001)
- FR-364-AC-19 (context_only, for the existing delay refusal code)
- FR-364-AC-25 (examined; FND-001)
- FR-364 frontmatter and Dependencies (examined; FND-002)
- TC-295 procedure and expected results 15-25 (examined; see also SR-2026112115155546054652125)

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Step 2 now runs the closure-stutter checks "before any step is re-executed, at its own pointer", but it does not order them among themselves. One step can fail two of them: a `delay` on a `closure-stutter` in the `prefix`, a `delay` on one whose item lacks `stutter_closure`, or a closure step beside another loop step in a non-`formula` counterexample. Two implementations would then report different first pointers. A `delay` on a prefix step also has no `/loop/0/delay` to refuse at. TC-295 tests each rule alone, so nothing fails. Fix: state the order (alphabet, then placement, then kind, then `delay`), and write the delay pointer as the step's own `/delay`. | FR-364:441-449,608 |

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Delay: `invalid_runtime_input`/`invalid-value` at `/loop/0/delay`, checked before any step is re-executed. It is now in step 2's list, in AC-27, and in TC-295 step 20 (second half) and step 26. |
| FND-002 | fixed | `depends_on` FR-368 and `references` FR-380 added. FR-380 is now a relative link (FR-364:316). |

Round 2, against ba5ad863daf7b6e3a30c84c267cf8f1ca83e6145 (fix delta c70d0889..ba5ad863, one commit; base 9f5b30a5 unchanged). Only the delta was verified; static checks only (`quire validate --strict` on FR-364, TC-295 and TC-431, exit 0). No Cargo or runtime run. The 15 findings fixed in round 1 were re-checked against the delta, and none regressed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | Deterministic order: closure steps in wire order (prefix, then loop), first failing step refused. Per step: (a) alphabet, (b) placement, (c) kind, (d) the step's own `/delay` (`/prefix/<i>/delay` or `/loop/<j>/delay`, no hard-coded `/loop/0`); then, after re-execution, terminal at `/loop/0/transition` and entry key at `/loop/0/post_state`, reached only for a sole-loop step. AC-27 matches. TC-295 step 20's third case (no flag plus delay, giving `/loop/0/transition/type`) fixes alphabet before delay. The Closure stutter paragraph points to the step's own `/delay`. |
