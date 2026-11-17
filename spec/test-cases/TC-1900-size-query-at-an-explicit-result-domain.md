---
id: TC-1900
title: "Size query at an explicit result domain"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-783
    type: verifies
---

# TC-1900: Size query at an explicit result domain

## Description

Verify that `size<N>(c)` is typed `N`, that the length interval of `c` must lie
inside `N`'s domain, that a non-integer `N` and a non-collection operand are
refused, and that the typed and untyped forms return the same length. Scope:
FR-783-AC-1, FR-783-AC-2, FR-783-AC-3, FR-783-AC-4.

## Test Procedure

1. Check `size<Int[0, 5]>(q)`, `size<Integer>(q)`, `size<Int[0, 3]>(q)`, the
   guarded `size<Int[0, 3]>(q)` and `size<Int[1, 5]>(q)` for each collection
   kind of FR-783-AC-1.
2. Check the refused type arguments and operand of FR-783-AC-2.
3. Evaluate `size<Int[0, 5]>(q)` and `size(q)` over the values of FR-783-AC-3,
   and check `size<Count>(q)` under `type Count = Int[0, 10];`.
4. Evaluate `size<Int[0, 1]>(q)` directly in the kernel over three occurrences.

## Expected Results

- Step 1: the two wide domains check; `Int[0, 3]` refuses
  `undefined_expression`/`unproved-range` with required `[0, 3]` and proved
  `[0, 5]` unless guarded; `Int[1, 5]` refuses over a minimum of `0` and checks
  over a minimum of `1`.
- Step 2: `ill_typed`/`type-mismatch` at the type argument, at `3`, and an
  unresolved type name at `Undeclared`.
- Step 3: `3` and `3`; `2` and `2`; type `Count`.
- Step 4: `integer_out_of_domain`/`outside-domain` at the `size` call.
