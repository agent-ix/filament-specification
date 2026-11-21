---
id: TC-2112
title: "Model-union witness rows join the selected document"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: verifies
---

# TC-2112: Model-union witness rows join the selected document

## Description

Verify that a reader admits the selected model-union witness rows of
`model-union-witness-vectors.json` and refuses each mutation with the stated
code, cause and locus. Scope: FR-830-AC-1 to FR-830-AC-12. No reader or harness
implements FR-830 yet, so the vectors are inputs and expected results and this
case is not executed.

## Test Procedure

1. For each `positive` case, build the `model_unions` sidecar of the case over
   `fixtures/positive-model-tuple-union.json` (with the case's `document_patch`
   applied to the `domain_package` of `model-tuple-union-vectors.json` and its `package_patch` to the package), and join it
   under the case's `scope` with an ample budget.
2. For each `schema_refusals` and `reader_refusals` case, apply
   its `patch` to the rows of its `base` case, join them, and compare the result
   with `expected_code`, `expected_cause` and `locus`.
3. For each `budget_cases` case, run its probes on one `NativeJoinBudget`: measure the baseline work and storage delta, then assert each probe's admission or first unpaid charge.

## Expected Results

- Step 1: every positive case is admitted and its rows are returned byte for byte.
- Step 2: every refusal returns its code, cause and locus, every budget probe returns its stated first unpaid charge, and no case returns a partial carrier.
