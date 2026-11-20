---
id: SR-2026112008541001
title: "spec-review of completion spec batch 4, hybrid certificate rules (filament-specification PR 771, filament-spec-language PR 3165)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification PR 771 head 036c9e9835ab322a625fbce3833ae5d1faf972c2 (FR-652, FR-193, FR-331, schema.json, vectors.json, TC-1605, TC-431); agent-ix/filament-spec-language PR 3165 head 2ed2446f56feeaee26b7e3516f61c7d3eff2a188 (FR-751, FR-750, FR-450, FR-621, FR-1019, FR-1021, TC-2101, TC-1221, TC-3035, TC-2869)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-652
    type: references
  - target: ix://agent-ix/filament-specification/FR-193
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
---

## Summary

Ticket: QSL-2531 (with QSL-2532). One pass of quoin:spec-review over `git diff origin/main...HEAD` in both worktrees, reviewer-only, no builds. The weight is soundness: a rule that accepts a certificate for a false claim is the worst outcome.

Checked clean: the `initial`, `successor`, `jump` (including the narrowed jump time) and `coverage` rules, the dwell tree, and the per-instant Kleene reading of `claim` together enclose every behaviour by `horizon` with at most `jumps` jumps and establish `exhaustive`, given a sound `picard`. Rules (a), (c) and (d) have their inclusions the right way round (initial hull inside the box; post image inside the target box). The `validated-witness` rules build a real behaviour from the trace's start, with every instant and the jump instant taken from the witness and never from the trace. Settlement is consistent across FR-652, FR-750, FR-450, FR-621 and FR-968. Every member of `HybridCertificate`, `HybridCertificateStep` and `HybridJumpSource` is read by a structural refusal or examined by a rule. Each of the nine step-back scenarios (SR-2026112003154600000001 FND-001..009) is now rejected by a named rule or settled by exact replay: FR-652-AC-9, AC-11, AC-13, AC-5, AC-7, AC-14, AC-16, FSL FR-750-AC-16 and FR-450-AC-13. No AC id collides with origin/main or with open PRs (FSpec #769 merges cleanly with #771). The new vectors match the schema. `quire validate` exits 0 on every changed file. Nothing previously specified is dropped.

Three rules still accept wrong certificates: the boundary-face test of rule (b), the contracted Picard inclusion when a conditional is involved, and the `model-steps` reading of positions after a behaviour's last jump.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The boundary-face rule takes the bound `e` over `E` while the other variables may leave `E` with `x_i`. Invariant `x = y` makes both upper faces boundary faces, so (b) accepts `E = x, y in [0, 1]` for `x' = y' = 1`, and `always holds(h.x <= 1)` is certified although `x = 2` at time 2. A conditional `e` breaks the same rule. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:186-194, 286 |
| FND-002 | high | Contracted Picard is unsound when a contracting conjunct contains a conditional. With `x' = 1`, `y' = x`, invariant `x <= if y > 0 then 10 else 0` and initial `(0, 0)`, the one step with box `(0, 0)`, `B` `x [-1, 0], y [-1, 0]` and `h` 1 passes every flowpipe rule, and `always [0, 1] holds(h.x <= 0)` is certified although `x = t`. The "Why contraction is sound" first-exit argument does not prove inclusion just after the exit instant. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:196-215, 322 |
| FND-003 | high | Under `model-steps`, FR-652 reads a behaviour with fewer than `p` jumps at position `p` as its last jump's state, and cites FR-193 for it. FR-193 says no such thing. Its only repetition rule repeats the last state in time. One mode with `x' = 1`, `0 <= x <= 1`, `x = 0` initially and no transitions is then certified for `always [0, 1] holds(h.x < 1)`, although its repeated last state is `x = 1`. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:417-423 |
| FND-004 | medium | FR-621's list of records carrying a hybrid certificate leaves out an `incomplete` record whose replay the work limit stopped. Its list of unchecked cases leaves out a refused replay and a replay stopped by the work limit. FR-652 records all three unchecked. | spec/functional/FR-621-write-the-analyze-result-as-one-outcome-document-per-request.md:120 |
| FND-005 | medium | The witness's decisive reading is "evaluated as under Reading the claim … definitely false at the origin", but that section admits only `on origin`. How an `on each` claim (an activation condition, a capture) is refuted is not stated. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:385-387, 479-486 |
| FND-006 | medium | FR-193 does not say what a solution of a flow with `if` is. Under Carathéodory solutions, `x' = if x > 0 then 1 else 0` from `x = 0` has `x = t`, which leaves `B = [0, 0]` although the non-strict Picard condition and (b)'s face rate pass. The soundness arguments need classical solutions. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:270-273 |
| FND-007 | low | FR-652-AC-4's box rejection names neither `box-shape` nor its pointer. No AC rejects `box-shape` at a step pointer, and none rejects for definedness in `picard`, `jump` or `witness-guard`. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:599 |
| FND-008 | low | The "may end" test ignores enabled transitions. Every `Moving` step of Braking then holds `v [0, 20 - k/16]` at every later instant, so the true `always [5, 10] holds(h.v <= 15)` is never certified. This incompleteness is not stated. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:392-396 |
| FND-009 | low | `witness-picard` defines `l` for every step except the first step of dwell 0. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:439 |

## Recommendations

- FND-001: FR-652:187-189, replace "with `e` not reading `x_i` and" with "with `e` free of conditionals and reading no continuous variable other than those whose rate in the mode is the constant 0, and". AC-1 (`v >= 0`) and AC-7 (`v <= 41/2`) are unchanged. Add to AC-7: "over `Twin`, `hybrid Twin using p { initial { mode M when { x = 0 and y = 0 }; } mode M invariant { x = y } flow { x' = 1; y' = 1; } }`, the box `M: x [0, 1], y [0, 1]` against `always holds(h.x <= 1)` is rejected `b` at `/certificate/boxes/M`."
- FND-002: replace FR-652:203-213 ("The Picard and advance inclusions … stays enclosed.") with: "Let `t*` be the last instant up to which a behaviour stays in `B`. Up to `t*` its state lies in the tube and, satisfying the invariant, in `Contract(Inv, tube)`, so in `B`. Just after `t*` a variable can leave `B` only at a face where `B`'s bound equals the tube's, which needs that variable's rate to reach 0 there. When every rate and every contracting conjunct's `e` is free of conditionals (after expansion), both are Lipschitz near `B`, so the distance beyond `B` grows by at most a constant times its own integral and stays 0 (Grönwall). A conditional can make a rate or a bound jump at `B`'s face, so the reader SHALL check the Picard and advance inclusions uncontracted for a step of a mode whose rates, or whose invariant's contracting conjuncts, contain a conditional." Add the `x <= if y > 0 then 10 else 0` model to FR-652-AC-10, rejected `picard` at `/certificate/steps/0`. If the lead prefers the author's fallback (Picard uncontracted, with only `box` and the advance contracted), AC-5, AC-10 and FR-1019 "Dwells" must widen `B` to the uncontracted tube.
- FND-003: add to FR-193 "Behaviours and positions", after the `model-steps` bullet: "A behaviour that takes `k` jumps has positions 0 to `k`; at every position after `k` it reads the state of position `k`." Then change FR-652:419-420's citation to that sentence. If FR-193 rules the other way (the last state in time), FR-652 SHALL also read, at each position after a dwell's depth, the `B` of every step from which a behaviour may end.
- FND-004: FR-621:120, replace "and an `incomplete` record whose witness check the work limit stopped" with "and an `incomplete` record whose replay or witness check the work limit stopped". Replace "(FR-750 "Dispatch": a proof settled `insufficient-horizon`, or the witness of a trace exact replay decides)" with "(FR-750 "Dispatch": a proof settled `insufficient-horizon`, or the witness of a trace exact replay decides, whose replay is refused or whose replay the work limit stops)".
- FND-005: append to FR-652 "Readings and the decisive box": "For an `on each` claim, a reading is decisive when, over the readings up to it, some activation instant has its activation condition definitely true and definitely defined, and its obligation, with each capture read over the witness's state at that instant, definitely false."
- FND-006: FR-193:272, replace "follows a solution of `m`'s flow equations" with "follows a classical solution of `m`'s flow equations, a continuously differentiable function whose derivative equals each right side at every instant".
- FND-007: FR-652-AC-4, replace "a box missing variable `a` is rejected at its pointer" with "a box missing variable `a` is rejected `box-shape` at `/certificate/boxes/Moving`, and FR-652-AC-5's certificate with `steps/3` `apriori` `v [1, 0]` is rejected `box-shape` at `/certificate/steps/3/apriori`".
- FND-008: append to FR-652:396: "This test ignores whether a transition is enabled at the invariant's boundary, so it can leave a true claim uncertified, never a false one certified."
- FND-009: FR-652:439, replace "`l` the previous step's `time.upper` in the dwell, or for the first step of dwell `d > 0` the jump instant of dwell `d - 1`" with "`l` 0 for the first step of dwell 0, the jump instant of dwell `d - 1` for the first step of dwell `d > 0`, and otherwise the previous step's `time.upper` in the dwell".

## Verdict

filament-specification #771: blocking (FND-001, FND-002, FND-003). filament-spec-language #3165: no high in its own text, so mergeable, but land it with #771's fix round. FR-1019 "Dwells" changes if FND-002 takes the uncontracted fallback, and FND-004 (medium) is in FR-621.

Open point 3 is an acceptable incompleteness. No AC requires certification of Braking with `horizon` 20, `jumps` 1 and `exhaustive` `["jumps"]`. FR-193's example and TC-373 HY-20 are verdict-mapping cases with no certificate. FR-1019's `model-steps` bound reports no `jumps` exhaustiveness for such a dwell. The cause is step granularity: the `Moving` dwell empties only on a step whose covered instants pass 20. With `horizon` 21 the certificate passes.

## New findings (disposition pass 1)

Reviewed fix commits: filament-specification 036c9e98..15ed2258 and filament-spec-language 2ed2446f..7a0f3b3a.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | high | Uncontracted Picard, and rule (b)'s face-rate test, are unsound for a rate with a conditional even under classical solutions. With `x' = 1`, `y' = if y > 0 then x else 0` and `x = y = 0` initially, `y = t²/2` is a classical solution, since `y'` is continuous and equals the right side at every instant. Yet one flowpipe step with box `(0, 0)` and `B` `x [0, 1], y [0, 0]` passes `tube ⊆ B`, and the inductive box `x [-1, 1], y [-1, 0]` passes (b) (the `y` face rate is 0). Both certify `always … holds(h.y <= 0)`. | spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:237-239, 314, 350 |
| FND-011 | high | Under `model-steps`, a behaviour that dwells forever after its last jump has no rule in FR-193, and FR-652's position reading misses its states. Over `Decay` (`x' = -x`, `0 <= x <= 1`, `x = 1` initially, no transition), the closing two-step certificate certifies `always [1, 1] holds(h.x < 1)`: position 1 reads only the may-end `B` `x [0, 3/4]`. Under the terminal-stutter reading recommended below, position 1 holds `x = 1`. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:289-294; spec/functional/analysis/FR-652-check-hybrid-enclosure-and-witness-certificates.md:457-471 |

Recommendations:

- FND-010: at FR-652:237-239, replace "For a mode that is not contraction-admitted the reader SHALL therefore check the uncontracted inclusion `tube ⊆ B`, the classical a-priori condition, and still contract `box` and the advance." with "For a mode that is not contraction-admitted the reader SHALL therefore check the uncontracted inclusion `tube ⊆ B`, and still contract `box` and the advance. Both the uncontracted inclusion and rule (b)'s face rates also need rates that are Lipschitz near the box. So a step of a mode whose expanded rates contain a conditional fails `picard`, and such a mode's box fails `b`, unless every conditional's condition is *strictly decided* over that box: each comparison `a ~ b` it reads has `a - b` with an interval that excludes 0. The same branch is then taken near the box. Otherwise a classical solution can take the other branch just outside the box: with `x' = 1` and `y' = if y > 0 then x else 0` from `(0, 0)`, `y = t²/2` leaves `B` `x [0, 1], y [0, 0]` although `tube ⊆ B`." Add to FR-652-AC-10: "over `Lift`, `hybrid Lift using p { initial { mode M when { x = 0 and y = 0 }; } mode M invariant { x >= -1 and x <= 1 and y >= -1 } flow { x' = 1; y' = if y > 0 then x else 0; } }`, the step `{M, time [0, 1], box x [0, 0], y [0, 0], B x [0, 1], y [0, 0], step 1}` under `horizon` 1, `jumps` 0, `exhaustive` `["time"]` against `always [0, 1] holds(h.y <= 0)` is rejected `picard` at `/certificate/steps/0`, and the box `M: x [-1, 1], y [-1, 0]` against `always holds(h.y <= 0)` is rejected `b` at `/certificate/boxes/M`, `y > 0` not being strictly decided."
- FND-011: append to FR-193:294: "A behaviour that takes `k` jumps and then dwells forever, diverging in time, SHALL have at every position after `k` its state at position `k`, the state its last jump reaches (its start state when `k` is 0). A discrete behaviour that takes no further step stutters in its last state for the same reason (FSL ADR-018 terminal stutter): positions observe only jumps, so its last observation repeats." At FR-652:457-471, replace "so the reading at position `p` is the meet of the readings of the `box` of every jump step of depth `p` and of the `B` of every may-end step of every dwell of depth less than `p`; it is never read from the box at a behaviour's last jump alone, because the behaviour moves on from there before it ends." with "and, for the behaviours that dwell forever after `j < p` jumps, their state at position `j`. So the reading at position `p` is the meet of the readings of the `box` of every jump step of depth `p`, of the `B` of every may-end step of every dwell of depth less than `p`, and of the first `box` of every dwell of depth less than `p` that closes." Add to FR-652-AC-14 and FR-193-AC-30: "over `Decay`, `hybrid Decay using p { initial { mode D when { x = 1 }; } mode D invariant { x >= 0 and x <= 1 } flow { x' = -x; } }`, the steps `{D, time [0, 1/2], box x [1, 1], B x [1/2, 1], step 1/2}` and `{D, time [1/2, 1], box x [0, 3/4], B x [0, 3/4], step 1/2}` under `clock "model-steps"` with `horizon` 1, `jumps` 1 and `exhaustive` `["jumps"]` are rejected `claim` at `/certificate/steps/0` against `always [1, 1] holds(h.x < 1)`, since position 1 holds the start state `x = 1`."

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
| FND-008 | fixed | - |
| FND-009 | fixed | - |
