---
id: TC-1009
title: "Language forms recorded in the standard grammar"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-502
    type: verifies
---

# TC-1009: Language forms recorded in the standard grammar

## Description

Verify the population bound, quantified clock bound, delivery count interval, qualified protocol requirement, clock reference and Text constructor productions and their refusals (FR-502). Scope: FR-502-AC-1 to FR-502-AC-6.

## Test Procedure

1. Parse the population bounds of FR-502-AC-1.
2. Parse the time invariants of FR-502-AC-2.
3. Parse and check the channels of FR-502-AC-3.
4. Check the protocol requirements of FR-502-AC-4.
5. Check the clock references of FR-502-AC-5 in a timed model.
6. Check the Text constructor functions of FR-502-AC-6.

## Expected Results

Each step gives exactly the productions, values and located refusals its acceptance criterion states.
