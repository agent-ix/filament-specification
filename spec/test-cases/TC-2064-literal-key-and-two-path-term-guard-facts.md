---
id: TC-2064
title: "Literal-key membership and two-path term guard facts"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-146
    type: verifies
---

# TC-2064: Literal-key membership and two-path term guard facts

## Description

Verify FR-146-AC-20 and FR-146-AC-21: a `contains(m, k)` guard with a literal
key proves presence for the read of the same literal key only, and a
comparison of a two-path `+` or `-` term against an integer literal bounds
that term, while three-path terms, `!=` and combinations of bounds prove
nothing.

## Test Procedure

Select the `filament.value.complete/v1` definitions through
[`value-complete.md`](../../proposals/filament-v1/definitions/value-complete.md)
and link each function body of FR-146-AC-20 and FR-146-AC-21 as its own
complete-V1 unit through the real local Rust boundary.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| K01 | `m: Map<Int[0, 9], Integer>[0, 3]`, body `if contains(m, 1) then m[1] else -1` | admitted |
| K02 | same `m`, body `if not contains(m, 1) then -1 else m[1]` | admitted |
| K03 | same `m`, `k: Int[0, 9]`, body `if contains(m, k) then m[k] else -1` | admitted |
| K04 | bodies `if contains(m, 1) then m[2] else -1`, `if contains(m, 1) then m[k] else -1`, `if contains(m, k) then m[1] else -1`, `if contains(m, 1) then -1 else m[1]`, `m[1]` | each `refused { code: undefined_expression, cause: unproved-presence }` at the index |
| S01 | `narrow(x: Int[0, 9])`, `x`, `y: Int[0, 9]`, bodies `if x + y <= 9 then narrow(x + y) else 0`, `if 9 >= y + x then narrow(x + y) else 0`, `if x + y < 10 then narrow((x) + (y)) else 0`, `if x + y > 9 then 0 else narrow(x + y)`, `if x - y >= 0 then narrow(x - y) else 0`, `if x + 1 <= 9 then narrow(x + 1) else 0` | each admitted |
| S02 | bodies `if x + y <= 10 then narrow(x + y) else 0`, `if x + y <= 9 then narrow(x + y + 1) else 0`, `if x + y != 10 then narrow(x + y) else 0`, `narrow(x + y)` | each `refused { code: undefined_expression, cause: unproved-range }` at the argument |
| S03 | `z: Int[0, 9]`, bodies `if x + y + z <= 9 then narrow(x + y + z) else 0`, `if x + y <= 9 and y >= 2 then narrow(x + 2) else 0` | each `refused { code: undefined_expression, cause: unproved-range }` at the argument |

## Expected Results

Every vector has the stated disposition, and each refusal is located at the
index or call argument named in its row.
