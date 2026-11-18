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

Verify the hybrid certificate forms, their exact check and their effect on certification and refutation. Scope: FR-652-AC-1 to FR-652-AC-5 and FR-193-AC-22.

## Test Procedure

1. Check the `inductive-enclosure` certificates FR-652-AC-1 lists.
2. Check a `flowpipe` with a Picard violation and a well-formed one covering horizon 10.
3. Read the non-polynomial point trace of FR-652-AC-3 with an accepted witness, with none, and with a straddling last box.
4. Check the mismatched, malformed and work-limited certificates FR-652-AC-4 lists.
5. Check the `Braking` flowpipe with its one jump step at the narrowed jump time `[0, 10]`, with a jump-step time omitting time 0, with time `[0, 20]`, and against the violated claim.

## Expected Results

- Step 1: accepted and `certified`; rejected at rule (a), `uncertified` with `certificate-rejected`.
- Step 2: rejected naming the step and `uncertified`; accepted and `certified`.
- Step 3: `refuted` and recorded in `counterexamples`; `inconclusive` twice.
- Step 4: each refusal or rejection as stated.
- Step 5: accepted and `certified`; rejected with rule `jump` at the jump step; accepted; rejected with rule `claim`.
