---
id: TC-2097
title: "Integer widening at declared parameter and payload positions"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-149-AC-30
    type: verifies
---

# TC-2097: Integer widening at declared parameter and payload positions

## Description

Verify that a bounded integer argument at a declared function parameter or union member payload of a containing integer type is admitted as an explicit exact `filament.op.numeric.convert` in the checked package, that a narrowing argument at those positions keeps its range obligation, and that `Some` payloads stay exact.

## Test Procedure

Select `filament.value.complete/v1`. Declare `g(a: Integer)`, `h(a: Int[0, 2000])`, `t(a: Int[0, 9])` (each `Boolean`, body `true`) and `union Reading { Raw(Integer), Wide(Int[0, 2000]), Tiny(Int[0, 9]) }`. In a function with parameters `x: Int[0, 1000]`, `n: Integer`, `w: Int[0, 20]` and `b: Int[0, 3]`, check:

1. `g(x)`, `h(x)`, `Reading::Raw(x)`, `Reading::Wide(x)`, and read each argument or payload term in the checked package.
2. `g(n)`, `g(7)`, `Reading::Raw(n)`.
3. `t(w)`, `t(n)`, `Reading::Tiny(w)`, `Reading::Tiny(n)`, and `if w <= 9 then t(w) else false`.
4. `Some(b)` at an `Option<Int[0, 9]>` parameter.
5. `g(x div 2)`, `let y: Integer = x in g(y)`, the body of `function r using v(x: Int[0, 1000]): Integer pure { x }`, and `x = n`.

## Expected Results

1. Each is admitted; each argument or payload term is one `filament.op.numeric.convert` application whose type node is the declared type node, with no obligation.
2. No conversion node.
3. Each of the four is `undefined_expression`/`unproved-range` at the argument; the guarded call is admitted.
4. `ill_typed`/`type-mismatch` at `b`.
5. `g(x div 2)` records no conversion; the `let` form and `r` are admitted with no conversion node anywhere; `x = n` is `ill_typed`/`type-mismatch`.
