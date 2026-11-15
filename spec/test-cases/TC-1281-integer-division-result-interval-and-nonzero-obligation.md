---
id: TC-1281
title: "Integer division result interval and nonzero obligation"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-147
    type: verifies
---

# TC-1281: Integer division result interval and nonzero obligation

## Description

Check the result type and the nonzero obligation of `div`, `rem` and `mod`.

Scope: FR-147-AC-12 and FR-147-AC-13.

## Test Procedure

1. Check `a mod b` over `a: Int[0, 100]`, `b: Int[1, 12]` into `Int[0, 11]`
   and into `Int[0, 10]`; then with `a` unbounded into `Int[0, 11]`.
2. Under each law, read the type of `a rem b` over `a: Int[-100, 100]`,
   `b: Int[1, 12]` and of `a div b` over `a: Int[0, 100]`, `b: Int[1, 12]`.
3. Under each law, check `a div b`, `a rem b` and `a mod b` with
   `b: Int[0, 12]` unguarded, then under `b != 0`, then with `b: Int[1, 12]`.

## Expected Results

- Step 1: admitted, refused `unproved-range`, admitted.
- Step 2: `rem` `Int[0, 11]` under Euclidean and floor and `Int[-11, 11]`
  under truncating; `div` `Int[0, 100]`.
- Step 3: `undefined_expression`/`unproved-nonzero` at each operator before
  any range obligation; the guarded and nonzero-range forms discharge it.
