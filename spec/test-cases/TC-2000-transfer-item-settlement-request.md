---
id: TC-2000
title: "Transfer item settlement request and transfer record"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-820
    type: verifies
---

# TC-2000: Transfer item settlement request and transfer record

## Description

Admit, refuse and settle FR-820 transfer requests. Scope: FR-820-AC-1 to
FR-820-AC-5.

## Test Procedure

1. Build a transfer request for FR-380-AC-9's `once[1,1]` clause over a
   step-for-step refinement of the abstract counter, both slots filled with
   proved premise results; settle it, then settle it again with both entries
   read from bytes as `certified`.
2. Settle the same request over the compare-and-set refinement, over a
   refinement with an `any` arm, and over a `time dense` concrete subject.
3. Add a third slot; swap the slots; drop a slot's `result`; replace the
   refinement result with one over another concrete subject; replace the
   abstract-claim result with one over another package, initial state and
   universe; point `item.clause` at a bounded-profile clause.
4. Add an `F_P` constraint no `F_A` constraint implies; settle a liveness
   clause whose abstract-claim result lacks `stutter_closure`; seed the
   abstract-claim result `unsupported`, `incomplete` and `refuted` for a
   fragment clause, one per run.
5. Settle with `max_work` zero, and read the `transfer` record of every
   result above.

## Expected Results

1. `proved`, `decisive-witness`, `step_for_step.holds` `true`,
   `position-counting` `once[1,1]`, fairness implied, `safety`, both premise
   identities; the bytes-read run is `uncertified`.
2. `inconclusive`, `transfer-not-preserving{reason: fragment, operator:
   once[1,1]}`, with the first `stutter` row, the `any` row and
   `{"time": "dense"}` respectively.
3. Each refuses `invalid_runtime_input`/`invalid-value` at the member, with
   no planner result.
4. `transfer-not-preserving{reason: fairness}` naming the constraint;
   `transfer-not-preserving{reason: stutter-closure}`; `premise-unproved`
   naming slot 1 with `unsupported`, `incomplete` and `refuted`; none
   `refuted`.
5. `incomplete`, `limit-reached` naming `max_work`; every record carries all
   four facts.
