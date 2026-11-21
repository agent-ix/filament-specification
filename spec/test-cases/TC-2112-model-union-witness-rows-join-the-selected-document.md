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
code, cause and locus. Scope: FR-830-AC-1 to FR-830-AC-13. No reader or harness
implements FR-830 yet, so the vectors are inputs and expected results and this
case is not executed.

## Test Procedure

1. For each `positive` case, build the input from `fixtures/positive-model-tuple-union.json`
   and the `domain_package` of `model-tuple-union-vectors.json`: apply the case's
   `extensions` in order (each extension's `document_patch` to the projection and
   `package_patch` to the package), then the case's own `document_patch` and
   `package_patch`; form the witnesses by concatenating the extensions' `witnesses`
   in order and then the case's own, member by member; join its `model_unions` under
   the case's `scope` with an ample `NativeJoinBudget`.
2. For each `schema_refusals` and `reader_refusals` case, take the input of its
   `base` case exactly as step 1 builds it, including the base's `scope`,
   extensions, patches and witnesses; replace the scope only with the case's
   `scope_override`, apply `witnesses_patch` to the witnesses, apply `patch` to the
   `model_unions`, join, and compare the typed refusal's `code` and the
   `locus.pointer` with `expected`.
3. For each `budget_cases` case, build the input of its `base` (or `patch`ed base)
   as step 2 does, then run its probes on one `NativeJoinBudget`: measure the
   baseline work and storage delta with ample limits, then assert each probe's
   admission or exhaustion, first unpaid charge kind and the absence of output. A
   `then` case runs its second input on the same budget.

## Expected Results

- Step 1: every positive case is admitted and its rows are returned byte for byte.
- Step 2: every refusal returns its code and locus, and no case returns a partial carrier.
- Step 3: every probe returns its stated admission or exhaustion with the first unpaid charge, and consumption accumulates on the shared budget.
- No vector is executed by any harness yet; the extension and Holder patches are unread inputs.
