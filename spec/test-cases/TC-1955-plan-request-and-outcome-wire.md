---
id: TC-1955
title: "Plan request admission and outcome document"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-805
    type: verifies
---

# TC-1955: Plan request admission and outcome document

## Description

Verify the outcome document members, exits and the request reader. Scope: FR-805-AC-1 to FR-805-AC-8.

## Test Procedure

1. Run FR-650-AC-1's request.
2. Run an infeasible and an incomplete request.
3. Run a decimal-objective request.
4. Send requests with an unknown member, a duplicate policy, a string `policy` and a zero limit.
5. Run a request whose certificate is rejected.
6. Run the counter at goal 21, the counter with `plan_search.max_states` 2 and a hybrid request with no hybrid provider.
7. Read `objective.value` of FR-650-AC-1, FR-810-AC-3 and FR-809-AC-2.
8. Send `limits.max_steps` and `accounting.steps` bare and under a host `synthesis` member, and hybrid requests with `dwell_grid` 1/2, `1` and `0/1`, and one carrying `initial`.

## Expected Results

- Step 1: the members as stated, exit 0.
- Step 2: `plan` null, closure certificate, exit 10; `cause`, exit 22.
- Step 3: the run value form, not a JSON float.
- Step 4: the four refusals at their pointers, exit 20.
- Step 5: `rejected` with `rule` and `entry`.
- Step 6: every member present; `null` objective value, plan, replay and bound; certificate and certification as FR-805 "Members by status" states.
- Step 7: `{"type": "integer", "value": "3"}`, the `rational` `60/1` with `unit` `min`, the `rational` `3/4`.
- Step 8: `unknown-member` at the bare and the `/request/synthesis`-prefixed pointers; the half grid admitted, then `wrong-value-kind` and `invalid-value` at `/dwell_grid`; `unknown-member` at `/initial`.
