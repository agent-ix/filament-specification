---
id: TC-2061
title: "Check a conditional probability certificate"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-412
    type: verifies
---
# TC-2061: Check a conditional probability certificate

## Description

The checker accepts a `conditional` certificate only after rebuilding the
activating states and, under every scheduler, the reset process itself.

Scope: FR-412-AC-9, FR-412-AC-10.

## Test Procedure

1. Over FR-407-AC-18's model, offer the `every-scheduler` certificate with
   `post` values 1 and `1/2` at the two first activating states and `reset`
   value `1/2` at `s0`, for `>= rational(1, 2)` and `>= rational(3, 5)`.
2. Offer it again with `reset`'s `s0` value `1/4`, and again with `post`'s
   `1/2` replaced by `3/5`.
3. Under the workload choosing `a` and `b` with `1/2` each, offer the
   `workload` certificate with numerator `3/8` and denominator `5/8` for
   `>= rational(3, 5)`, then with denominator 1.
4. Offer a `vacuous` certificate over the variant where `g` is unreachable,
   then over the model itself.
5. Offer a `conditional` certificate with no `conditional` member, and a
   `workload` one with no `denominator`.
6. Serialize and read two product states that differ only in `instance`,
   and two that differ only in `activation`.

## Expected Results

- Step 1: accepted at `1/2`; `threshold-missed` at `s0` for `3/5`.
- Step 2: `inequality-fails` at `s0`; `inequality-fails` at the activating
  state reached by `b`.
- Step 3: accepted; then `inequality-fails` in the denominator.
- Step 4: accepted; then `inequality-fails` at the initial state.
- Step 5: `malformed` at `conditional`; `malformed` at `denominator`.
- Step 6: each pair keeps two distinct keys.
