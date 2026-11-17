---
id: TC-1951
title: "Plan search bounds, meters and completeness"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-801
    type: verifies
---

# TC-1951: Plan search bounds, meters and completeness

## Description

Verify the planner limits, the evaluation meter, cancellation, node merging and the separation from the verification planner's limit. Scope: FR-801-AC-1 to FR-801-AC-6.

## Test Procedure

1. Run FR-650-AC-1's request with `plan_search.max_states` 2, then 1000.
2. Run it with a low `accounting.work_units`.
3. Cancel a run; run one past its deadline.
4. Plan over a model where `inc, dec` returns to the start.
5. Plan with a goal reading `Reach.steps[0]`.
6. Run with `planner.max_work=1` and with `plan_search.max_work=1`.

## Expected Results

- Step 1: `incomplete` naming `plan_search.max_states`, then `optimal`.
- Step 2: `incomplete` naming `accounting.work_units`.
- Step 3: `cancelled{source: requested}` and `cancelled{source: deadline}`, best plan as `feasible`, never `infeasible`.
- Step 4: `states` counts distinct nodes.
- Step 5: no merging; `optimal` with first step `double`.
- Step 6: unchanged, then `incomplete` naming `plan_search.max_work`.
