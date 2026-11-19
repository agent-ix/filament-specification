---
id: TC-2046
title: "A verification plan request reads null, seeds and measured decisions by one rule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-824
    type: verifies
---

# TC-2046: A verification plan request reads null, seeds and measured decisions by one rule

## Description

Verify that a `native-run/1` request locates verification plan refusals at
`/request/verification_plan`, reads a `null` member as absent, refuses a
seed outside its domain as `invalid-value` and one of another JSON kind as
`wrong-value-kind`, carries a measured step's decision only in its result,
and writes `status` `null` for a run that executes nothing. Scope:
FR-824-AC-6.

## Test Procedure

1. In a `native-run/1` request, send `seed` `"7"`, `-1`, `7.5` and
   `18446744073709551616`.
2. Send `{"plan": "Release", "seed": null, "emit": null, "initial": null}`,
   then `"verification_plan": null`.
3. Run FR-824-AC-5's plan with `s` measured accepted, rejected and
   undecided, and read `s`'s disposition.
4. Send `{"plan": "Release", "emit": true, "execute": false}`.

## Expected Results

- Step 1: `wrong-value-kind`, then `invalid-value` three times, each at
  `/request/verification_plan/seed`, exit 20.
- Step 2: `Release` runs with `inputs` `{"initial": null, "seed": 0}`; then
  `wrong-value-kind` at `/request/verification_plan`, exit 20.
- Step 3: the disposition's members are exactly `status`, `method`,
  `evidence`, `blocked_by` and `result`; `result.verdict.decision.type` is
  `accepted`, `rejected` and `undecided` in turn.
- Step 4: `status` `null`, `dispositions` `null`, exit 0.
