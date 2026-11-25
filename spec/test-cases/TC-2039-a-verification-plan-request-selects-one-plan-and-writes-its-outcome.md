---
id: TC-2039
title: "A verification plan request selects one plan and writes its outcome"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-824
    type: verifies
---

# TC-2039: A verification plan request selects one plan and writes its outcome

## Description

Verify the verification plan request's members, its plan selection, its
per-step inputs and the `filament.verification-plan-outcome/v1` document.
Scope: FR-824-AC-1 to FR-824-AC-3 and FR-824-AC-8.

## Test Procedure

1. Execute `Release` with `seed` 7 and `max_trace_length` 8 where `safe` is refuted; then request it
   with `emit` `true` and `execute` `false`.
2. Send `{"max_trace_length": 8}` over the one plan `Release` and over two plans; name `Nope`, the claim `Never`, and
   a bare `Release` declared as `A::Release` and `B::Release`.
3. Send `seed` `"7"`, both flags `false`, an `engine` member, `seed` on a
   plan of `model_check` steps only, and, over a selected package with
   `max_trace_length` 8, no `document` and a plan document of another
   package; then `Sized` (FR-824-AC-7) with neither `max_trace_length` nor
   `max_value_bits`.
4. Execute FR-192-AC-8's `Release` plan over its finite-domain `M::Order`
   state subject with seed 7 and `max_trace_length` 8, omitting `initial`.
   Inspect each generated model-based-test case's initial state and repeat
   the same request.

## Expected Results

- Step 1: `failed`, the three dispositions, `failed_steps` `["safe"]`, exit
  22 (FR-301's highest-severity code); then `document` present, `dispositions` `null`, exit 0.
- Step 2: the plan runs; `missing-member`, `missing-name` twice and
  `ambiguous-name`, each at `/plan`, exit 20.
- Step 3: `wrong-value-kind` at `/seed`, `invalid-value` at `/execute`,
  `unknown-member` at `/engine` and `/seed`, `missing-member` and
  `invalid-value` at `/document`; `missing-member` at `/max_trace_length`
  only.
- Step 4: every generated initial state belongs to the complete initial-state
  domain admitted by `M::Order`, drawn from the subject's finite object
  universe and FR-190 state-field domains, with no key outside that universe;
  `inputs.initial` is `null`, and the repeated request gives a byte-equal
  model-based-test record.
