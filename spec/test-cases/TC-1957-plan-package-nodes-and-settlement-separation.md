---
id: TC-1957
title: "Plan package nodes and separation from verification settlement"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-807
    type: verifies
---

# TC-1957: Plan package nodes and separation from verification settlement

## Description

Verify the nodes the planner reads and its separation from FR-492. Scope: FR-807-AC-1 to FR-807-AC-4.

## Test Procedure

1. Plan from a package with and without its temporal clause node.
2. Send a planner settlement request carrying a plan outcome as a slot result.
3. Set each engine's work limit to 1 and run the other.
4. Check an invariant over a reported plan's execution.

## Expected Results

- Step 1: plans, then the policy refused.
- Step 2: `invalid_runtime_input` at `/slots/<i>/result`.
- Step 3: each unchanged.
- Step 4: the item settles on its own.
