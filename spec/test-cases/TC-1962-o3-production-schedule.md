---
id: TC-1962
title: "Reference case O3: production schedule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-810
    type: verifies
---

# TC-1962: Reference case O3: production schedule

## Description

Run reference case O3's published example through the planner and check every state of its plan with the native evaluator. Scope: FR-810-AC-3.

## Test Procedure

1. Run case O3's plan request as FR-810 describes it.
2. Run the variant FR-810-AC-3 names, where it names one.
3. Evaluate every state of each returned plan against the case's rules.

## Expected Results

- Step 1: the result, plan and exact objective FR-810-AC-3 states.
- Step 2: the variant's stated result.
- Step 3: every rule holds at every state.
