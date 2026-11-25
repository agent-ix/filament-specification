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
Scope: FR-824-AC-1 to FR-824-AC-3 and FR-824-AC-8 to FR-824-AC-9.

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
4. Execute a `Release` plan with one state-clause `model_based_test` step
   over `M::NoInitial`, declaring `cells: Population<Cell>` and
   `Cell.value: Int[0,1]`, with
   `{"plan":"Release","universe_snapshot":"universe.json","seed":7,"max_trace_length":8}` and no `initial`. The admitted snapshot is:

   ```json
   {"format":"filament.state.snapshot/v1","identity":{"authority":"agent-ix","identity":"ix://test/no-initial/universe"},"observation":"current","anchor":{"kind":"initialization","name":"NoInitial"},"model":{"identity":"test/no-initial"},"populations":[{"population":"ix://test/no-initial/cells","complete":true,"objects":[{"key":"c1","type":"ix://test/no-initial/Cell","fields":{"value":{"integer":"0"}}},{"key":"c2","type":"ix://test/no-initial/Cell","fields":{"value":{"integer":"1"}}}]},{"population":"ix://test/no-initial/unbounded_cells","complete":false,"objects":[]}]}
   ```

   Inspect each generated model-based-test case's initial state and repeat
   the same request. Also send a numeric `universe_snapshot`, a missing
   path, the same snapshot with `cells.complete` false, and the snapshot to a
   selected plan with no state-subject `model_based_test` step.

## Expected Results

- Step 1: `failed`, the three dispositions, `failed_steps` `["safe"]`, exit
  22 (FR-301's highest-severity code); then `document` present, `dispositions` `null`, exit 0.
- Step 2: the plan runs; `missing-member`, `missing-name` twice and
  `ambiguous-name`, each at `/plan`, exit 20.
- Step 3: `wrong-value-kind` at `/seed`, `invalid-value` at `/execute`,
  `unknown-member` at `/engine` and `/seed`, `missing-member` and
  `invalid-value` at `/document`; `missing-member` at `/max_trace_length`
  only.
- Step 4: the complete `cells` row admits exact keys `{c1,c2}`; all generated
  initial states and reachability use the four-state product of those keys
  with `Cell.value` in `{0,1}`, with no other key or population. The snapshot's
  field values do not become initial states. `inputs.initial` is `null`, and
  the repeated request gives a byte-equal model-based-test record. The numeric
  member is `wrong-value-kind`, the missing path is `io-error`, a
  `complete: false` cells row leaves the population without a finite universe
  and yields `declined` with `requires_bound`/`requires-bound`, member
  `subject`, and the FR-120 root list before any case runs; a plan without a
  consuming step refuses `unknown-member` at `/universe_snapshot`.
