---
id: TC-1042
title: "A step-for-step refinement transfers previous and interval operators"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-380
    type: verifies
---
# TC-1042: A step-for-step refinement transfers previous and interval operators

## Description

Verifies that a proved claim using previous (`once[1,1]`) or an interval
operator transfers through a refinement exactly when the refinement is
step-for-step (FR-380 "Step-for-step refinements"), and that a previous
clause refuted by terminal stutter does not become a proved premise.

## Test Procedure

1. Declare a refinement of the abstract counter whose step map sends every
   concrete operation to one abstract operation, with no `stutter` or `any`
   arm, over an untimed concrete subject; establish proved premise results
   for the refinement and the abstract claims `always (holds(c.value = 1)
   implies once[1,1] holds(c.value = 0))` and `always (holds(c.value = 1)
   implies eventually[0,2] holds(c.value = 2))`, satisfying FR-380's subject
   match, fairness implication and premise-form requirements.
2. Request a transfer of each claim to the concrete subject.
3. Request the same transfers through the compare-and-set refinement, which
   maps a concrete step to `stutter`.
4. Classify a refinement with an `any` arm and one over a `time dense`
   concrete subject.
5. Check the abstract clause `always (holds(c.value = 2) implies
   once[1,1] holds(c.value = 1))` with terminal stutter included. Use its
   refuted abstract-claim premise to request transfer through step 1's
   proved step-for-step refinement.

## Expected Results

1. The refinement premise and both positive abstract-claim premises are
   proved. An exhaustive proof premise requires its core-accepted
   certificate; an unchecked exhaustive result is not a proved premise.
2. Each transfer settles `proved`, basis `decisive-witness`, and no exploration
   of the concrete subject runs (FR-380-AC-9).
3. Each settles `inconclusive`, cause `transfer-not-preserving`, naming
   `once[1,1]` and `eventually[0,2]` (FR-380-AC-9).
4. Neither is step-for-step (FR-380-AC-9).
5. The abstract clause is `refuted`: on the counter behaviour with values
   `0, 1, 2, 2, ...`, the antecedent holds at the repeated terminal state 2
   but `once[1,1] holds(c.value = 1)` is false because the preceding state
   also has value 2. The transfer settles `inconclusive`, basis `unsettled`,
   cause `premise-unproved`, naming the abstract-claim premise (slot 1)
   and its `refuted` result; the transfer itself is never `refuted`
   (FR-380-AC-9, FR-380-AC-5).
