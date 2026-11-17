---
id: TC-1950
title: "Plan steps, arguments and goal over a state model"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-800
    type: verifies
---

# TC-1950: Plan steps, arguments and goal over a state model

## Description

Verify the plan value, reference and created-object arguments, the empty plan, undefined goals and unbounded parameters. Scope: FR-800-AC-1 to FR-800-AC-5.

## Test Procedure

1. Plan two `drain` steps over the `Server` population `n1`, `n2` and read `Reach.steps`.
2. Add `spawn` creating `n3` and plan with it.
3. Plan with a goal true in the initial state.
4. Plan with a goal undefined when `divisor` is 0.
5. Check a model with an unbounded `Integer` parameter.

## Expected Results

- Step 1: two step records with indexes, operation, arguments and states equal to the runtime's.
- Step 2: `n3` is a candidate only after `spawn`.
- Step 3: the empty plan, length 0.
- Step 4: the undefined plan does not reach the goal.
- Step 5: refused at the parameter as FR-806 states.
