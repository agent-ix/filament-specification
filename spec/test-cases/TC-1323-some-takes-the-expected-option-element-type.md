---
id: TC-1323
title: "Some takes the expected option element type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-143-AC-21
    type: verifies
---

# TC-1323: Some takes the expected option element type

## Description

Verify that `Some(e)` is a general option constructor whose payload is checked against the expected element type, so a literal payload adopts a bounded element type.

## Test Procedure

Select `filament.value.complete/v1`. Check `Some(3)` and `Some(30)` as a function result, an argument and an `=` operand against `Option<Int[0, 9]>`; `Some(b)` for `b` of `Int[0, 9]` and of `Int[0, 3]`; `Some(3)` at an `Integer` result; `let x = Some(3) in x`; and `Some(1)` in a state clause outside any refinement. Read the emitted package.

## Expected Results

`Some(3)` is admitted at each `Option<Int[0, 9]>` position, `Some(30)` is `ill_typed`/`type-mismatch` at `30`, `Some(b)` is admitted for `Int[0, 9]` and refused at `b` for `Int[0, 3]`, `Some(3)` at an `Integer` result is refused at `Some(3)`, the `let` has type `Option<Integer>`, the state-clause `Some(1)` is the constructor, and each admitted `Some(e)` is one `value`/`option_value` node.
