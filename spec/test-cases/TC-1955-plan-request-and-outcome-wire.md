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

Verify the outcome document members, native receiver/outcome identity, exits and the request reader. Scope: FR-805-AC-1 to FR-805-AC-10.

## Test Procedure

1. Run FR-650-AC-1's request.
2. Run an infeasible and an incomplete request.
3. Run a decimal-objective request.
4. Send requests with an unknown member, a duplicate policy, a string `policy` and a zero limit.
5. Run a request whose certificate is rejected.
6. Run the counter at goal 21, the counter with `plan_search.max_states` 2 and a hybrid request with no hybrid provider.
7. Read `objective.value` of FR-650-AC-1, FR-810-AC-3 and FR-809-AC-2.
8. Send `limits.max_steps` and `accounting.steps` bare and under a host `synthesis` member, and hybrid requests with `dwell_grid` 1/2, `1` and `0/1`, and one carrying `initial`.
9. Use TC-1950 steps 6 and 7 to read a multiple-outcome receiver-bearing
   step, a singleton receiver-bearing step and a singleton receiverless
   step. Resolve the receiver's exact reference triple against the supplied
   initial world. Also use TC-2037 step 8's mixed receiver types.
10. At the existing replay and certificate verification entry surfaces,
    supply the three-outcome step with its ordinal omitted, then with
    ordinals `-1`, `3`, `0.5` and `"0"`, and with another in-range ordinal
    while keeping its original post-state. Supply the singleton with an
    ordinal of `0`. Separately omit a required receiver, add a receiver
    to the receiverless step, or substitute the wrong operation's receiver
    type. These are edits of supplied verification data, not new plan
    request members or a new outcome-reader API.

## Expected Results

- Step 1: the members as stated, exit 0.
- Step 2: `plan` null, closure certificate, exit 10; `cause`, exit 22.
- Step 3: the run value form, not a JSON float.
- Step 4: the four refusals at their pointers, exit 20.
- Step 5: `rejected` with `rule` and `entry`.
- Step 6: every member present; `null` objective value, plan, replay and bound; certificate and certification as FR-805 "Members by status" states.
- Step 7: `{"type": "integer", "value": "3"}`, the `rational` `60/1` with `unit` `min`, the `rational` `3/4`.
- Step 8: `unknown-member` at the bare and the `/request/synthesis`-prefixed pointers; the half grid admitted, then `wrong-value-kind` and `invalid-value` at `/dwell_grid`; `unknown-member` at `/initial`.
- Step 9: the receiver-bearing steps write the FR-181 typed reference
  exactly, including universe, object type and object identity. Only the
  three-outcome step has `outcome_ordinal` (2 when it selects `S2`);
  singleton ordinals and receiverless receivers are absent, never null.
- Step 10: each verification attempt rejects the missing, unexpected,
  wrongly typed, out-of-range or inconsistent identity member, located at
  the actual supplied offending member when present. A valid in-range
  ordinal selecting another post-state is not accepted as the recorded
  step. Existing entry-surface refusal and certificate-failure contracts
  apply; the test asserts no new wire carrier or refusal code.
