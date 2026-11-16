---
id: TC-1700
title: "A record-sample counterexample names each position by its sample index"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: verifies
---

# TC-1700: A record-sample counterexample names each position by its sample index

## Description

Check the observed-arm `{sample}` step of a `TemporalCounterexample` over a
trace of record samples: each step names its position by its index in the
request trace's position list, equal samples at different positions get
different steps, the loop closes on the request's own loop positions, and the
reader refuses malformed or mixed steps before replay.

Scope: FR-364-AC-16.

## Test Procedure

1. Read and replay the counterexample `{"trace": "observed", "kind": "formula", "prefix": [{"sample": "0"}, {"sample": "1"}], "loop": [{"sample": "2"}], ...}` of `always holds(r.level < 2)` against the request whose lasso holds prefix samples `level` 0, 0 and loop sample `level` 2.
2. Read the same counterexample with one step `{"sample": "0", "document": "0"}`, with a `sample` of `0` (a JSON number), of `"01"` and of `"3"`, with the first prefix step replaced by `{"document": "0"}`, and with `loop` `[{"sample": "1"}]`.

## Expected Results

- Step 1: the counterexample is admitted and replay reproduces the violation at position 2.
- Step 2: each variant is refused before replay; the mixed-step variant as a mix of steps, the others as `malformed_wire` or `invalid_runtime_input`/`invalid-value` at the step.
