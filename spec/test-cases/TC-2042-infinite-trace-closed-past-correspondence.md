---
id: TC-2042
title: "Infinite-trace closed-past requests map to the infinite-trace TL target"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-095
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-092
    type: verifies
---

# TC-2042: Infinite-trace closed-past requests map to the infinite-trace TL target

## Description

Verify the infinite-trace closed-past row of the correspondence table: its target, the agreement of independent native and TL evaluators over lassos, the pre-origin reading, and the requests that stay unsupported. Scope: FR-095-AC-12.

## Test Procedure

1. Under `filament.temporal.infinite-trace/v1`, map `once[1,1] not p`, `historically[0,2] p`, `p since[0,2] q` and `p triggered[1,2] q`.
2. Over every lasso of a prefix of 0 to 2 positions and a loop of 1 to 3, at most 3 positions, over every total valuation of `p` and `q`, evaluate each step-1 request natively and with the TL infinite-trace evaluator at every activation position the unrolled lasso distinguishes.
3. Over the lasso where `p` holds at every position, evaluate `once[1,1] not p` and `historically[0,2] p` at position 0 in both evaluators and in one that clips the closed past window at the origin.
4. Map `once p`, `once[1,*] p`, `eventually[0,2] p` and `p` under the same profile.

## Expected Results

- Step 1: each maps to `mltl.infinite-trace/v1` with `tl-syntax.formula-unbounded/v1` and `tl-syntax.lasso-trace/v1`.
- Step 2: equal verdicts at every position of every lasso; the population cardinality is recorded.
- Step 3: `true` and `false` in both evaluators; the clipping evaluator gives `false` and `true`.
- Step 4: the typed unsupported mapping of FR-095-AC-9, naming the infinite-trace dimension, with no substitute formula.
