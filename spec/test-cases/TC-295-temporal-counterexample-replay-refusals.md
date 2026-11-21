---
id: TC-295
title: "Temporal counterexample wire, replay and refusals"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: verifies
---

# TC-295: Temporal counterexample wire, replay and refusals

## Description

Replay of model-arm and observed-arm counterexamples: the wire members, the
replay refusals, the missing fairness premise and the parity outcome.

## Test Procedure

Start from the TC-293 step 2 counterexample `X` (loop of three `upd(a)` steps,
`c = b`, fairness (`weak`, `attemptUpdate`, `whole`)) and the TC-290 Counter
subject. Derive:

1. `X` unchanged.
2. `X` with the loop's last step removed, so the loop does not close.
3. `X` replayed against the clause with `fair weak each …`.
4. `X` with the second step's `post_state` replaced by the state key of `(2,0)`.
5. A Counter prefix with four `inc` steps, the fourth at value 3.
6. A Counter lasso whose loop is a terminal stutter step at value 2.
7. `X` with `trace_position` 0 against `always eventually holds(c.versionNumber
   = 0)`, which is true on the lasso.
8. `X` with the member `fairness` removed, with an extra member `engine`, and
   with `trace` `"sampled"`.
9. A safety refutation of TC-290's `always holds(x.value <= 2)`.
10. An observed-arm counterexample for `eventually[0,1] holds(c.value = 2)`
    under the event-position profile: `trace` `"observed"`, three steps whose
    documents hold `c.value` 0, 1 and 2,
    `trace_position` 0, empty `fairness`, every document held by the byte
    provision.
11. Step 10's counterexample against the clause under `fair weak inc`, with
    `fairness` (`weak`, `inc`, `whole`).
12. Step 10's counterexample with its third document missing from the
    provision.
13. Withdrawn.
14. Withdrawn.
15. Closure stutter. The TC-290 Counter subject, `value` 0 to 3, no `terminal`
    declaration, empty fairness, and the clause `eventually holds(c.value = 3)`
    under infinite-trace in an item that selects `stutter_closure` (FR-368):
    the counterexample `L` has the `prefix` of one `inc` step to `value` 1, the
    `loop` the one step `{"type":"closure-stutter"}` whose `post_state` key is
    the key of `value` 1, and `trace_position` 0.
16. `L` for the same clause in an item that does not select `stutter_closure`.
17. `L` with a prefix of three `inc` steps and the loop at `value` 3, a
    terminal state.
18. `L` with a `closure-stutter` step appended to its `prefix` (index 1), and `L`
    with the loop `[closure-stutter, inc]`, the closure stutter first.
19. `L` with the loop's `post_state` replaced by the key of `value` 2.
20. `L` with `kind` `deadlock`, `trace_position` `null`, `over_binding` `null`
    and empty `fairness`, so that only the closure rule fails; `L` with a `delay`
    member of 1 on the closure stutter step; and `L` for an item that does not
    select `stutter_closure`, with that `delay` member of 1.
21. `L` against the clause under `fair weak inc`, with `fairness` (`weak`,
    `inc`, `whole`), and under `fair strong inc` with (`strong`, `inc`,
    `whole`).
22. `L` against `eventually holds(c.value >= 0)` in an item that selects
    `stutter_closure`, true at position 0.
23. The step 6 lasso, a terminal stutter at `value` 3, for the clause of step 15
    in an item that selects `stutter_closure`, and step 6 unchanged.
24. Mixed loop, on the TC-293 ConfigVersion subject with `c` bound to `a`, under
    `filament.temporal.infinite-trace/v1`, empty fairness, in an item that
    selects `stutter_closure`, and the clause `always (holds(c.versionNumber =
    1) implies eventually[1,1] holds(c.versionNumber = 2)) and always eventually
    holds(c.versionNumber = 2)`. An engine's lasso over the stutter closure has
    the prefix `[closure-stutter]` at `(0,0)` and the loop `(0,0) -upd(a)-> (1,0)
    -closure-stutter-> (1,0) -upd(a)-> (2,0) -upd(a)-> (0,0)`; the clause is
    false at its position 2, where `versionNumber` is 1 and the next position
    is 1 again. The writer drops both closure stutters and writes the prefix
    empty and the loop of three `upd(a)` steps, `trace_position` 1: the
    engine's position 2 less the one dropped prefix stutter, which is the
    position of `versionNumber` 1 on the written lasso.
25. Subject classes, each in an item that selects `stutter_closure` with the
    clause `eventually holds(c.value = 3)` of step 15, and the loop
    `[closure-stutter]`. (a) `TimedCounter` is the TC-290 Counter subject
    declared `time dense unit ms` (FR-415), with no clock, no time invariant
    and no task set, quiescent at `value` 3; `L`'s `closure-stutter` loop with
    its `post_state` the key of `value` 1; and, as the control, a loop of the
    one `terminal-stutter` step with delay 1 after a prefix of three `inc` steps,
    at `value` 3, `trace_position` 0, where no source transition is enabled and
    the state is not urgent (FR-161 "Idle tail" and "Timed lasso"); `L`'s
    `prefix` is the one `inc` step. (b) The `Fill` protocol of TC-376 with
    `TwoPre: self.v <= 1` as PK-01 builds it, `k` bound to `c`, with the clause
    `eventually holds(k.v = 3)` under infinite-trace: `initial_state` 0, an empty
    `prefix`, the loop `[closure-stutter]` whose `post_state` key is the initial
    protocol state's key as PK-01 records it, `trace_position` 0.
26. Schema vectors. The backend-provider vector `temporal-outcome-closure-stutter`
    and its three adverse vectors, which add an `operation` member to the step,
    spell its type `closure_stutter` and remove its `post_state`.

## Expected Results

1. Reproduces: `reproduced-with-evaluated-witness`, item `refuted`
   (FR-364-AC-3).
2. Refuses `invalid_runtime_input`/`invalid-value` (FR-364-AC-4).
3. Refuses as an unfair lasso naming (`weak`, `attemptUpdate`, `each`) with
   the starved receiver `b`; no `refuted` (FR-364-AC-5).
4. Withdrawn: the recorded post-state key is not checked against a successor.
5. Refuses `invalid_runtime_input`/`invalid-value`: `inc` is not enabled at
   value 3 (FR-364-AC-4).
6. Refuses `invalid_runtime_input`/`invalid-value`: value 2 is not terminal
   (FR-364-AC-4).
7. `inconclusive`, cause `replay-parity` (FR-364-AC-6).
8. All three refuse before replay (FR-364-AC-2).
9. A finite prefix with an empty `loop`, `kind: formula`, which reproduces
   (FR-364-AC-1).
10. `reproduced-with-evaluated-witness`: positions 0 and 1 hold `c.value` 0
    and 1, so the formula is false at position 0 (FR-364-AC-9).
11. `unsupported`, cause `missing-fairness-premise`, naming
    (`weak`, `inc`, `whole`), with no formula evaluated (FR-364-AC-9).
12. Refuses `invalid_runtime_input`/`invalid-value` (FR-364-AC-9).
13. Withdrawn.
14. Withdrawn.
15. `reproduced-with-evaluated-witness`, item `refuted`: no fairness constraint
    is enabled at the loop state and the clause is false on the trace that
    repeats `value` 1 (FR-364-AC-25).
16. Refuses `invalid_runtime_input`/`invalid-value` at `/loop/0/transition/type`
    (FR-364-AC-27).
17. Refuses `invalid_runtime_input`/`invalid-value` at `/loop/0/transition`: the
    state is terminal (FR-364-AC-27).
18. Both refuse `invalid_runtime_input`/`invalid-value`, before any step is
    re-executed: the first at `/prefix/1/transition/type`, the second at
    `/loop/0/transition/type` (FR-364-AC-27).
19. Refuses `invalid_runtime_input`/`invalid-value` at `/loop/0/post_state`
    (FR-364-AC-27).
20. Each refuses `invalid_runtime_input`/`invalid-value`: the first at
    `/loop/0/transition/type` (kind), the second at `/loop/0/delay`, the third at
    `/loop/0/transition/type`, the alphabet check running before the delay check
    (FR-364-AC-27).
21. Each refuses as an unfair lasso naming the constraint, `inc` being enabled at
    `value` 1; no `refuted` (FR-364-AC-28, FR-364-AC-5).
22. `inconclusive`, cause `replay-parity` (FR-364-AC-25).
23. The terminal stutter at `value` 3 replays as step 6's rules say, unchanged
    by the item's `stutter_closure`, and step 6 still refuses at `value` 2
    (FR-364-AC-4, FR-364-AC-25).
24. The written lasso replays to `inconclusive`, cause `replay-parity`, never
    `refuted`: the clause is true on three `upd(a)` steps, where `versionNumber`
    1 is followed by 2 (FR-364-AC-29).
25. In (a) the `closure-stutter` loop refuses `invalid_runtime_input`/`invalid-value`
    at `/loop/0/transition/type` (FR-364-AC-27). The `terminal-stutter` control
    passes the closure, alphabet and delay checks and settles `inconclusive`,
    cause `replay-parity`, because `eventually holds(c.value = 3)` is true at
    `value` 3; it is not a refutation (FR-161 "Idle tail" and "Timed lasso",
    FR-364-AC-25). In (b) the `closure-stutter` loop refuses
    `invalid_runtime_input`/`invalid-value` at `/loop/0/transition/type`
    (FR-364-AC-27).
26. The positive vector is admitted and the three adverse vectors are refused at
    the patched step (FR-364-AC-26). The schema is checked for nothing else
    here; the refusals of steps 16, 18, 20 and 25 are replay's.
