---
id: TC-2013
title: "Simulation reports invariant violations as findings"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-181
    type: verifies
---

# TC-2013: Simulation reports invariant violations as findings

## Description

Verify that a simulation result reports each invariant violation on a
reported trace as a finding and settles no claim. Scope: FR-181-AC-33.

## Test Procedure

1. Simulate, by enumeration and by sampling with a fixed seed, a subject
   whose declared invariant is false at a state its traces reach.
2. Repeat the sampled request with the same seed.
3. Simulate the same subject with the invariant removed, and a subject whose
   invariant holds at every reached state.

## Expected Results

- Step 1: one finding per violating state on a reported trace, naming the
  trace, the position, the invariant and the object; no counterexample or
  verdict.
- Step 2: equal findings.
- Step 3: the outcome, traces and coverage of step 1 without the findings;
  no finding for the holding invariant.
