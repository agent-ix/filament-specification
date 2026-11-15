---
id: TC-1325
title: "An equality sum refusal names its bound, and the join is explicit"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-149-AC-29
    type: verifies
---

# TC-1325: An equality sum refusal names its bound, and the join is explicit

## Description

Verify that the FR-149 arithmetic-operand rule is kept, that its refusal names the failing interval, bound and supplying operand, and that `convert<Integer>` spells the comparison at the join.

## Test Procedure

Select `filament.value.complete/v1`. Check `function f using v(a: Int[0, 10], b: Int[0, 10], c: Int[0, 10]): Boolean pure { c = a + b }`, the same body under the guard `a <= 4 and b <= 5`, and the body `convert<Integer>(c) = a + b`, evaluating the last at `a = 8`, `b = 9`, `c = 10`.

## Expected Results

The first is `undefined_expression`/`unproved-range` at `a + b` and names `[0, 20]`, `Int[0, 10]` and `c`; the guarded body checks; the explicit join checks with no range obligation and evaluates to `false`.
