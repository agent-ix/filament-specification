---
id: SR-2026112115155546054652125
title: "spec-review/failure-domain of closure-stutter writer limits and TC-295 cases 15-25"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/filament-specification; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md (Closure stutter, Named limit); spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md steps 15-25; QSL-2247; branch at 7537ca96, PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
  - target: ix://agent-ix/filament-specification/TC-295
    type: reviews
---

## Summary

Ticket: QSL-2247. I checked edge cases, identity confusion and the grounding of TC-295 cases 15-25 against the TC-290 Counter, whose value is 0 to 3, whose only operation is `inc` (guarded `value < 3`), and whose terminal state per FR-361 is `value = 3`.

Grounded and correct:
- Case 15: `eventually holds(c.value = 3)` is false on the lasso that repeats `value` 1, and empty fairness makes it fair, so it is `refuted`.
- Case 16: the alphabet refusal.
- Case 17: `value` 3 is terminal, refused at `/loop/0/transition`.
- Case 19: the post-state tamper.
- Case 21: `inc` is enabled at `value` 1, so both weak and strong are unfair.
- Case 22: a `true` gives parity.
- Case 23: the terminal-stutter control is unchanged.
- Case 25: the schema vectors, with the schema-admitted placements stated honestly.

## Verdict

**CONDITIONAL.** Four mediums: one writer-contract gap in the named limit, two unconstructible or ambiguous TC oracles, and one untested restriction. Fix them in the contract before FSL builds FR-128/FR-1030 cases from them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The named limit says how a mixed closure loop is written ("its model steps alone") but not what happens to closure stutters in the engine's prefix. The prefix may hold stutters, and FR-364 forbids `closure-stutter` in any prefix. The limit also does not say that `trace_position` must be re-indexed onto the projected lasso. Two writers would emit different counterexamples for one engine lasso, and one that keeps the engine's index evaluates at the wrong position. The limit's soundness claim is accurate. Fix: say the writer drops every closure stutter in prefix and loop, except the sole-loop form, and that `trace_position` indexes the written lasso. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:317-325 |
| FND-002 | medium | TC-295 case 24 names no subject, lasso or clause: "chosen so that its truth on the lasso differs from its truth with a stutter inserted". Its starting fixture cannot supply one: the Counter is monotone to its terminal `value` 3, so it has no loop of model steps other than the terminal stutter. The TC-293 ConfigVersion subject `X`, which has `upd` loops, is not named. As written, the case's oracle cannot be built, and the AC-25 mixed-loop sentence it backs is unverified. Fix: name the subject (for example `X`), the lasso, a concrete stutter-sensitive clause using `next`, and `trace_position`. | spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md:66-69,121 |
| FND-003 | medium | TC-295 case 20's first half, "`L` with `kind` `deadlock`", has no determined first refusal. `L` keeps `trace_position` 0 and a non-empty `loop`, while the wire table requires `trace_position` null for `deadlock` and FR-366 shapes it as a finite prefix. Expected 20 names no pointer ("because the counterexample is not a `formula` one"), while AC-25 says `/…/transition/type`. An implementation that refuses first on the deadlock shape, possibly `malformed_wire`, or at `/trace_position`, cannot be judged. Fix: give the pointer `/loop/0/transition/type`. Either set `trace_position` null and empty `fairness` so that only the closure rule fails, or state the precedence. | spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md:57,112-114; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:87,430-431 |
| FND-004 | medium | The subject-class restriction has no case. A closure-stutter step over a timed, task-derived or protocol subject is outside the alphabet even when the item selects `stutter_closure`, and this is the rule that keeps time divergence and task continuations unwaived. AC-25 covers only "an untimed, non-task model subject", and TC-295 uses only the Counter. A replayer that admits the step for a `time dense` subject passes every case. Hybrid (FR-193) subjects are also not named in "neither task-derived, timed nor protocol". Fix: add an AC-25 clause and a TC-295 case (for example a `time dense` variant of `L` with `stutter_closure`, refused at `/loop/0/transition/type`), and say whether hybrid counts as timed. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:297,312-315,585; spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md:45-75 |
| FND-005 | low | TC-295 case 18's second half adds "an `inc` step to the `loop` beside the closure stutter" without saying before or after. Expected 18 says "at the loop step's `/transition/type`" without the index. Placing `inc` after the stutter also breaks loop closure, which step 4 refuses later. The refusal index (`/loop/0` or `/loop/1`) is not fixed. Name the order and the pointer. | spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md:54-55,107-109 |

## Scope

- FR-364 Closure stutter paragraph (examined; FND-004)
- FR-364 Named limit paragraph (examined; FND-001)
- TC-295 cases 15, 16, 17, 19, 21, 22, 23, 25 (examined, clean)
- TC-295 case 18 (examined; FND-005)
- TC-295 case 20 (examined; FND-003)
- TC-295 case 24 (examined; FND-002)
- TC-290 Counter subject (context_only)
- TC-293 ConfigVersion subject (context_only)
- FR-361 Terminal states; FR-366 deadlock shape (context_only)

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The writer drops every closure stutter in the prefix and the loop except the sole-loop form, and `trace_position` is re-indexed past each dropped preceding stutter (AC-29). |
| FND-002 | fixed | Concrete and checked by hand. X has `c` = `a`, and the engine lasso has a prefix stutter at (0,0) and the loop upd, stutter, upd, upd back to (0,0). The clause is false at engine position 2, where v=1 is followed by v=1. The written lasso is three upd(a) steps and `trace_position` 1 (2 minus the one dropped prefix stutter) is v=1. The clause is true there (v=1, then v=2, and always eventually 2), so the result is `replay-parity`. |
| FND-003 | fixed | `trace_position` null, `over_binding` null and empty `fairness` leave only the closure rule failing. The expected pointer is `/loop/0/transition/type`, and step 2's 'before any step is re-executed' fixes it ahead of the deadlock evaluation (step 7). |
| FND-004 | still-open | The hybrid wording is fixed (hybrid counts as timed, FR-364:298-299; AC-27 names timed, task-derived, hybrid and protocol), but TC-295 step 25 still cannot be built, for two reasons. (1) The timed half uses fixture B of FR-364-AC-23, whose `M::Call` has three always-enabled operations, so B has no quiescent product-terminal state. The case's 'at its quiescent state' and the claim that 'FR-364-AC-19's terminal-stutter loop of delay 1 at the same state still replays' have no such state. B's only item is a `schedulable` EN-6 item, while `stutter_closure` is allowed only on a `formula` item (schema line 474), and no clause, prefix or entry key is given. (2) The protocol half names 'TC-375's protocol subject', but TC-375 holds eleven vectors (PK-01 to PK-11) with no single subject, and no clause, item or lasso is named. The author report calls this an FSL consumer obligation, but it is this PR's own AC-27 oracle. Fix: use a `time dense` variant of the TC-290 Counter (inc guarded `value < 3`, quiescent at `value` 3) with L's prefix, clause and `stutter_closure`, refused at `/loop/0/transition/type`, and the AC-19 delay-1 `terminal-stutter` control at `value` 3. For the protocol half, name one vector (for example PK-01 `Fill`), a formula clause, and the lasso: empty prefix, loop [closure-stutter] with `post_state` its initial state's key, which is enough because the refusal comes before re-execution. |
| FND-005 | fixed | Prefix index 1, and the loop `[closure-stutter, inc]` with the stutter first. Expected 18: `/prefix/1/transition/type` and `/loop/0/transition/type`, both before re-execution. |

Round 2, against ba5ad863daf7b6e3a30c84c267cf8f1ca83e6145 (fix delta c70d0889..ba5ad863, one commit; base 9f5b30a5 unchanged). Only the delta was verified; static checks only (`quire validate --strict` on FR-364, TC-295 and TC-431, exit 0). No Cargo or runtime run. The 15 findings fixed in round 1 were re-checked against the delta, and none regressed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | Both halves are now buildable and are this PR's own AC-27 oracle, not consumer work. (a) TimedCounter is the TC-290 Counter under `time … dense unit ms` (FR-415:59). A model with no clock is admitted (FR-415:186). There is no invariant, no `urgent` and no task set, so `value` 3 is quiescent and terminal (FR-161 Idle tail). L at `value` 1 is non-terminal and outside the timed alphabet, refused at `/loop/0/transition/type`. (b) The TC-376 `Fill` with `TwoPre: self.v <= 1` (PK-01), `k` bound to `c`, and the TC-376 clause `eventually holds(k.v = 3)`. Its initial state s0 (root at fork, enabling `fork(Both)`) is non-terminal, and its key is recorded by PK-01. With an empty prefix and the loop [closure-stutter], the alphabet check comes before re-execution, so the lasso needs nothing else. Hybrid counts as timed. One precision gap in the timed control's oracle is recorded as FND-006 (low). |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | TC-295 step 25(a)'s `terminal-stutter` control gives no `trace_position`, and expected 25 says only that it "replays as FR-364-AC-19's quiescent rule says". With the step's own clause `eventually holds(c.value = 3)`, the control lasso (three `inc` steps to `value` 3, then a delay-1 stutter) makes the clause true. A correct replayer therefore passes every step check and settles `inconclusive`, `replay-parity`, not `reproduced-with-evaluated-witness`, and "replays" can be read as a refutation. The rule that grounds a delay-1 stutter on a plain, task-free timed subject is FR-161 "Idle tail"/"Timed lasso" (quiescent state, one stutter with positive delay). AC-19 belongs to the task-automaton product rule of FR-364 step 4, which is the author's own caveat. This is a contained mechanical fix to one expected-result sentence: give `trace_position` 0, state that the control passes the closure, alphabet and delay checks and settles `inconclusive`, `replay-parity` because the clause holds at `value` 3, and cite FR-161 alongside or instead of AC-19. Optionally, name the `filament.temporal.complete/v1` profile layer that FR-415 requires. | TC-295:80-92,145-150; FR-161:131-151; FR-364-AC-19 |
