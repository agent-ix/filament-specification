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

Verify that a reader admits the selected model-union and model-tuple witness rows
of `model-union-witness-vectors.json` and refuses each mutation with the stated
code, cause and locus. Scope: FR-830-AC-1 to FR-830-AC-15 (vectors v5). No reader or harness
implements FR-830 yet, so the vectors are inputs and expected results and this
case is not executed.

## Test Procedure

1. For each `positive` case, take the fixture pair its `fixtures` names from the
   vectors' `fixtures` member (A: `domain-package-acme-shapes.json` with
   `fixtures/positive-model-tuple-union.json`; B:
   `domain-package-acme-shapes-witness.json` with
   `fixtures/witness-model-union-rows.json`; C: the same document with
   `fixtures/witness-model-union-source-rows.json`, which adds the source union
   Route and is read with a valid `source_unions` row beside the model rows), apply the case's own `document_patch`
   to the FCD document, read the package with the strict read, and join the
   sidecar `{model_tuples, model_unions}` and the case's `witnesses.declarations`
   under the case's `scope` with an ample `NativeJoinBudget`. The sidecar and the
   declarations are the whole witness input; no `SelectedTupleDeclarationRow` or
   other witness is supplied from memory.
2. For each `schema_refusals` and `reader_refusals` case, take the input of its
   `base` case exactly as step 1 builds it, including the base's fixtures,
   `document_patch`, `scope`, `witnesses`, `model_tuples` and `model_unions`;
   replace the scope only with the case's `scope_override`, apply `witnesses_patch`
   to the witnesses and `patch` to the sidecar object `{model_tuples,
   model_unions}`, join, and compare the typed refusal's `code`, the `locus.pointer` and, where `expected.node` is given, the structural node with `expected`. The file has no `source_tuples` member in the existing codec, so a `file_members` entry is only the control of MUW-TS-09 and no source-tuple witness is constructible. A `file_members` entry names members of the
   identity file that stay present and that no check may read.
3. For each `budget_cases` case, build the input of its `base` (or `patch`ed base)
   as step 2 does, then run its probes on one `NativeJoinBudget`: measure the
   baseline work and storage delta and, from the charge log, the cumulative amount
   at the last charge of the common pass, of the `model_tuples` rows and of the
   closure edges, with ample limits, then assert each probe's admission or
   exhaustion, first unpaid charge kind and channel and the absence of output. A
   `then` case runs its second input on the same budget, and a `compare` case
   (MUW-B-08) measures its base and the named case on separate budgets and checks
   the stated relation of their charge logs.

## Expected Results

- Step 1: every positive case is admitted and its rows of both channels are returned byte for byte.
- Step 2: every refusal returns its code and locus, and no case returns a partial carrier.
- Step 3: every probe returns its stated admission or exhaustion with the first unpaid charge, the two channels charge the one budget, and consumption accumulates on the shared budget.
- Every positive input was read once by the existing strict read of filament-contract-ir `origin/main` with no carrier logic: all are admitted once the known `Mix.Held` object-payload gap (an existing reader gap) is removed, and the Held-bearing inputs are refused there at the model node. No vector is executed by an FR-830 carrier, since none exists.
