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
4. Execute the bounded plan `verify Release using p { check tested claim
   NoInitial::CheckCells method model_based_test domain M::NoInitial bound
   100; }` over package `test/no-initial`. `NoInitial` declares only
   `cells: Population<Cell>`, `Cell` is record value type
   `ix://test/no-initial/Cell` with `value: Int[0,1]`, and `CheckCells` is a
   state clause whose predicate is `true`. Send:
   `{"plan":"Release","universe_snapshot":"universe.json","seed":7,"max_trace_length":8}` and no `initial`. The admitted snapshot is:

   ```json
   {"format":"filament.state.snapshot/v1","identity":{"authority":"agent-ix","identity":"ix://test/no-initial/universe"},"observation":"current","anchor":{"kind":"initialization","name":"NoInitial"},"model":{"identity":"test/no-initial"},"populations":[{"population":"ix://test/no-initial/cells","complete":true,"objects":[{"key":"c1","type":"ix://test/no-initial/Cell","fields":{"value":{"integer":"0"}}},{"key":"c2","type":"ix://test/no-initial/Cell","fields":{"value":{"integer":"1"}}}]}]}
   ```

   Inspect each generated model-based-test case's initial state and repeat
   the same request. Also send a numeric `universe_snapshot` and a missing
   path. Separately, use a refusal fixture whose model declares populations
   `cells` then `unbounded_cells`, both of `Cell`; send a snapshot whose
   `cells` remains complete as above, and the snapshot adds
   `{"population":"ix://test/no-initial/unbounded_cells","complete":false,"objects":[]}`; the plan fixture's model also declares `unbounded_cells: Population<Cell>` after `cells`.
   Finally send the successful snapshot to a selected plan with no
   state-subject `model_based_test` step.

## Expected Results

- Step 1: `failed`, the three dispositions, `failed_steps` `["safe"]`,
  `inputs` `{"initial": null, "seed": "7", "max_trace_length": "8", "max_value_bits": null}`, exit
  22 (FR-301's highest-severity code); then `document` present, `dispositions` `null`, exit 0.
- Step 2: the plan runs; `missing-member`, `missing-name` twice and
  `ambiguous-name`, each at `/plan`, exit 20.
- Step 3: `wrong-value-kind` at `/seed`, `invalid-value` at `/execute`,
  `unknown-member` at `/engine` and `/seed`, `missing-member` and
  `invalid-value` at `/document`; `missing-member` at `/max_trace_length`
  only.
- Step 4: the bounded plan admits and the complete `cells` row admits exact
  keys `{c1,c2}`; all generated
  initial states and reachability use the four-state product of those keys
  with `Cell.value` in `{0,1}`, with no other key or population. The snapshot's
  field values do not become initial states. `inputs.initial` is `null`, and
  the repeated request gives a byte-equal model-based-test record. The numeric
  member is `wrong-value-kind`, the missing path is `io-error`, a
  refusal fixture's complete `cells` row and `complete: false`
  `unbounded_cells` row leave `unbounded_cells` without a finite universe and yield `declined`
  with `requires_bound`/`requires-bound`, member `subject`, and the FR-120
  root list containing member type key `{package:"test/no-initial",path:["model","record_value_type","ix://test/no-initial/Cell"],ordinal:0}` before any case runs; a plan without a
  consuming step refuses `unknown-member` at `/universe_snapshot`.
