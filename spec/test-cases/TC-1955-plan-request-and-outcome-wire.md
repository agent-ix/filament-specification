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

Verify the outcome document members, exits and the request reader. Scope: FR-805-AC-1 to FR-805-AC-5.

## Test Procedure

1. Run FR-650-AC-1's request.
2. Run an infeasible and an incomplete request.
3. Run a decimal-objective request.
4. Send requests with an unknown member, a duplicate policy, a string `policy` and a zero limit.
5. Run a request whose certificate is rejected.

## Expected Results

- Step 1: the members as stated, exit 0.
- Step 2: `plan` null, closure certificate, exit 10; `cause`, exit 22.
- Step 3: the run value form, not a JSON float.
- Step 4: the four refusals at their pointers, exit 20.
- Step 5: `rejected` with `rule` and `entry`.
