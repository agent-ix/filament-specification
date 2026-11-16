---
id: TC-1653
title: "A linked count is proved inside its result type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-145
    type: verifies
---
# TC-1653: A linked count is proved inside its result type

## Description

A linked `count<N>` whose `[0, size maximum]` is not inside `N` is refused
`unproved-range`; under direct kernel evaluation the final count is refused
`integer_out_of_domain`.

Scope: FR-145-AC-16.

## Test Procedure

1. Check `count<Int[0, 3]>(x in q: x > 0)`, then into `Int[0, 5]` and `Integer`, for `q` a sequence, bag and set bounded `[0, 5]`.
2. Check the `Int[0, 3]` count under `if size(q) <= 3` and `if size(q) <= 4`.
3. Check `count<Int[1, 3]>(x in q: true)` over `Sequence<Integer>[0, 3]`.
4. Evaluate `count<Int[0, 3]>(x in q: x)` directly in the kernel over five `true` occurrences.

## Expected Results

- Step 1: `Int[0, 3]` refused `undefined_expression`/`unproved-range`; the others admitted.
- Step 2: `<= 3` admitted; `<= 4` refused `unproved-range`.
- Step 3: refused `unproved-range` (the lower bound is `0`).
- Step 4: `integer_out_of_domain`/`outside-domain` at the `count`.
