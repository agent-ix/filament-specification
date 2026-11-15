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

Verifies that a claim using previous (`once[1,1]`) or an interval operator
transfers through a refinement exactly when the refinement is step-for-step
(FR-380 "Step-for-step refinements").

## Test Procedure

1. Declare a refinement of the abstract counter whose step map sends every
   concrete operation to one abstract operation, with no `stutter` or `any`
   arm, over an untimed concrete subject; prove the refinement and the
   abstract claims `always (holds(c.value = 2) implies once[1,1]
   holds(c.value = 1))` and `always (holds(c.value = 1) implies
   eventually[0,2] holds(c.value = 2))`.
2. Request a transfer of each claim to the concrete subject.
3. Request the same transfers through the compare-and-set refinement, which
   maps a concrete step to `stutter`.
4. Classify a refinement with an `any` arm and one over a `time dense`
   concrete subject.

## Expected Results

1. Both premises are proved.
2. Each transfer settles `proved`, basis `decisive-witness`, and no exploration
   of the concrete subject runs (FR-380-AC-9).
3. Each settles `inconclusive`, cause `transfer-not-preserving`, naming
   `once[1,1]` and `eventually[0,2]` (FR-380-AC-9).
4. Neither is step-for-step (FR-380-AC-9).
