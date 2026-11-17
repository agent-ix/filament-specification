---
id: TC-1958
title: "Plans over tick and dense timed models"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-808
    type: verifies
---

# TC-1958: Plans over tick and dense timed models

## Description

Verify tick elapsed time, dense delays, unattained infima, time-locks and exact delay output. Scope: FR-808-AC-1 to FR-808-AC-5.

## Test Procedure

1. Plan three ticks of 10 min with two other steps.
2. Plan `go` guarded `x >= 2` minimizing `elapsed`.
3. Plan with the guard `x > 2`.
4. Add a time invariant `x <= 1`; a final state forbidding the stutter.
5. Repeat step 3.

## Expected Results

- Step 1: `elapsed` 30 min, delays 0.
- Step 2: `optimal`, delay 2, closure accepted.
- Step 3: `feasible`, bound 2 not attained, delay 5/2.
- Step 4: `infeasible`; not admissible.
- Step 5: byte-identical, delay `5/2`.
