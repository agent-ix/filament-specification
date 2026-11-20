---
id: TC-1605
title: "Hybrid enclosure and validated-witness certificates"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-652
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-193
    type: verifies
---

# TC-1605: Hybrid enclosure and validated-witness certificates

## Description

Verify the hybrid certificate forms, their exact check rule by rule with the
rule name and pointer of every rejection, the structural refusals, and the
certificate's effect on certification and refutation. Scope: FR-652-AC-1 to
FR-652-AC-5, FR-652-AC-7 to FR-652-AC-19, FR-193-AC-22, FR-193-AC-29 and FR-193-AC-30.

## Test Procedure

1. Check the `inductive-enclosure` certificates FR-652-AC-1 lists.
2. Check a `flowpipe` with a Picard violation and a well-formed one covering horizon 10.
3. Read the non-polynomial point trace of FR-652-AC-3 with its nine-step witness, with none, with the witness cut after its step at `[28, 32]`, and with the one-step witness whose `B` is `v [1, 41]`.
4. Check the mismatched, malformed and work-limited certificates FR-652-AC-4 lists.
5. Check the `Braking` flowpipe with its one jump step (carrying `from`) at the narrowed jump time `[0, 10]`, with a jump-step time omitting time 0, with time `[0, 20]`, and against the violated claim.
6. Check the `Rise` boxes `v [0, 41/2]`, `v [0, 39/2]` and `v [0, 20]`, and the `Twin` box (FR-652-AC-7).
7. Check the `Braking` and `Hold` inductive certificates of FR-652-AC-8.
8. Check the `Braking` flowpipes of FR-652-AC-9: the narrow first box, the wrong first time, and the two-initial-mode model with and without its initial `Stopped` dwell.
9. Check FR-652-AC-10's variants of FR-652-AC-5's certificate and its `Cond` and `Lift` cases.
10. Check the `Fork` flowpipes of FR-652-AC-11 against both claims.
11. Check the `Tick` flowpipe of FR-652-AC-12 with and without `from` on its third step.
12. Check the coverage cases of FR-652-AC-13 over `Braking`, `Fork` and `Drop`.
13. Check the claim cases of FR-652-AC-14 over `Hold`, `Braking`, and `Drop`, `Ramp` and `Decay` under `model-steps`.
14. Check the witness cases of FR-652-AC-15 over `Grow`, `GrowLow` and `Shift`, and the `GrowJump` cases of FR-652-AC-16.
15. Read each structurally mismatched certificate of FR-652-AC-17 on the analyze path and in a replay request.
16. Settle the items of FR-652-AC-18 and FR-193-AC-29.
17. Check the definedness cases of FR-652-AC-19 over `Recip`, `ForkDiv` and `GrowJump`, and read the `Ramp` and `Gate` behaviours of FR-193-AC-30.

## Expected Results

- Step 1: accepted and `certified`; rejected at rule (a), `uncertified` with `certificate-rejected`.
- Step 2: rejected naming the step and `uncertified`; accepted and `certified`.
- Step 3: `refuted` and recorded in `counterexamples`; `inconclusive` three times, each cut witness rejected `witness-violation` at its last step.
- Step 4: `invalid_result`/`invalid-value` at `/certificate`; `box-shape` at `/certificate/boxes/Moving` and at `/certificate/steps/3/apriori`; `work-limit`.
- Step 5: accepted and `certified`; rejected `jump` at `/certificate/steps/160`; accepted; rejected `claim` at `/certificate/steps/0`.
- Step 6: accepted; `a` at `/certificate/boxes/Up`; `b` at `/certificate/boxes/Up`; `b` at `/certificate/boxes/M`.
- Step 7: `c` at `/certificate/boxes/Stopped`; `d` at `/certificate/boxes/Moving` twice; `d` at `/certificate`; `Hold` accepted, then `d` at `/certificate/boxes/H`.
- Step 8: `initial` at `/certificate/steps/0` twice; accepted; `initial` at `/certificate/steps/0`.
- Step 9: accepted by contraction; `picard` at `/certificate/steps/3`; `successor` at `/certificate/steps/6`; `successor` at `/certificate/steps/1`; `Cond` rejected `picard` at `/certificate/steps/0`; `Lift` rejected `picard` at `/certificate/steps/0` and its box rejected `b` at `/certificate/boxes/M`.
- Step 10: accepted; `claim` at `/certificate/steps/10`; `jump` at `/certificate/steps/4`, `/certificate/steps/6`, `/certificate/steps/10` and `/certificate/steps/10`.
- Step 11: accepted; `successor` at `/certificate/steps/2`.
- Step 12: `coverage` at `/certificate/steps/0`, `/certificate/steps/159` and `/certificate/steps/4`; `Drop` accepted.
- Step 13: `Hold` accepted, then `claim` at `/certificate/steps/0` twice; `claim` at `/certificate/steps/0`; `Drop` accepted, then `claim` at `/certificate/steps/1`; `Ramp` rejected `claim` at `/certificate/steps/1`; `Decay` rejected `claim` at `/certificate/steps/0`.
- Step 14: `witness-start` at `/certificate/dwells/0/0`; `witness-picard` at `/certificate/dwells/0/3` and `/certificate/dwells/0/7`; accepted with the matching `end`, `witness-violation` at `/certificate/dwells/0/8` with the other; `Shift` accepted, `witness-unique` at `/certificate/dwells/0/0`; `GrowJump` accepted, `witness-guard` at `/certificate/dwells/0/1` twice.
- Step 15: each refusal at its stated code and pointer, `invalid_result` on the analyze path and `invalid_runtime_input` in the replay request; no internal fault.
- Step 16: certificate recorded unchecked; `refuted` by exact replay with the witness unchecked; `inconclusive`, `replay-parity` with the witness unchecked; `incomplete`, `limit-reached{limit: work_units}` with the witness rejected `work-limit`; `work-limit` at `/certificate/boxes/Moving`; FR-193-AC-29's `certified`, `insufficient-horizon` with the certificate unchecked, and `uncertified` with `certificate-rejected{rule: coverage, at: /certificate/steps/0}`.
- Step 17: `picard` at `/certificate/steps/0`; `jump` at `/certificate/steps/5`; `witness-guard` at `/certificate/dwells/0/1`; `Ramp` reads `x = 1` at position 1, `Gate` keeps `x = 0`; `Decay` holds `x = 1` at position 1.
