---
id: TC-1648
title: "Read a bare decimal literal as an exact decimal value"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-662
    type: verifies
---

# TC-1648: Read a bare decimal literal as an exact decimal value

## Description

Portable controls for the decimal primary. Scope: FR-662-AC-1 to FR-662-AC-4.

## Test Procedure

1. Parse function bodies `0.9`, `5.0`, `0.90` and `0.5 M::ms` and read the denoted values.
2. Check `0.9` at a `Decimal[-1000, 1000; 0, 2; exact]` result, in `l < 0.9` with `l` of that type, in `0.9 < decimal(1, 0)`, in `n < 0.9` with `n: Integer`, and in `let x = 0.9 in x > decimal(1, 1)`; repeat with `decimal(9, 1)`.
3. Check `expected accumulate R until holds(B) <= 0.5` over an integer reward and the same claim with `decimal(5, 1)` and `rational(1, 2)`.
4. Parse `1e5` and `1.5E-3` as primaries.

## Expected Results

- Step 1: `decimal(9, 1)`, `decimal(50, 1)`, `decimal(90, 2)` and one quantity literal.
- Step 2: checks; checks; `ill_typed`/`ambiguous-literal` at `0.9`; `ill_typed`/`type-mismatch`; `ill_typed`/`ambiguous-literal`; each equal to the constructor's outcome.
- Step 3: three equal thresholds `1/2`.
- Step 4: `invalid_syntax`/`invalid-token` at the whole spelling.
