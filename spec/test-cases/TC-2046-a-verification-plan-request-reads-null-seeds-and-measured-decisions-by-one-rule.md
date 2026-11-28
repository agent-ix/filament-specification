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
FR-824-AC-6 and FR-824-AC-7.

## Test Procedure

1. In a `native-run/1` request, send `seed` `"7"`, `-1`, `7.5` and
   `18446744073709551616`.
2. Send `{"plan": "Release", "seed": null, "emit": null, "initial": null, "max_value_bits": null, "max_trace_length": 8}`,
   then `"verification_plan": null`.
3. Run FR-824-AC-5's plan with `s` measured accepted, rejected and
   undecided, and read `s`'s disposition.
4. Send `{"plan": "Release", "emit": true, "execute": false}`.
5. Send `{"plan": "Release"}`; `max_trace_length` 0, `-1`, `8.5` and `"8"`; `max_value_bits` 8 on `Release`; `max_trace_length` 8 on a plan of `model_check` steps only.
6. Over the plan `Sized` (one `model_based_test` step `s` over a model with a field `n: Integer`), send `max_trace_length` 8 without `max_value_bits`, with `max_value_bits` 0 and 65, and with `max_value_bits` 8.
7. Run a plan whose `model_based_test` step discards every case, with a second step that `depends` on it.

## Expected Results

- Step 1: `wrong-value-kind`, then `invalid-value` three times, each at
  `/request/verification_plan/seed`, exit 20.
- Step 2: `Release` runs with `inputs` `{"initial": null, "seed": "0", "max_trace_length": 8, "max_value_bits": null}`; then
  `wrong-value-kind` at `/request/verification_plan`, exit 20.
- Step 3: the disposition's members are exactly `status`, `method`,
  `evidence`, `blocked_by` and `result`; `result.verdict.decision.type` is
  `accepted`, `rejected` and `undecided` in turn.
- Step 4: `status` `null`, `dispositions` `null`, exit 0.
- Step 5: `missing-member` at `/request/verification_plan/max_trace_length` naming `tested`; `invalid-value` three times and `wrong-value-kind` there; `unknown-member` at `/request/verification_plan/max_value_bits` and at `/request/verification_plan/max_trace_length`; each exit 20, no step run.
- Step 6: `missing-member` naming `s`, then `invalid-value` twice, at `/request/verification_plan/max_value_bits`, exit 20; then `s` runs and `inputs.max_value_bits` is 8.
- Step 7: the step `inconclusive` with `result.cause` `{"type": "no-evaluated-case"}`, its dependent `blocked` by it, exit 22.
