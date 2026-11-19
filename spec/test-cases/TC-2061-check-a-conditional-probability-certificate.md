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
activating states and the reset process itself, and checks the reset process
over proper schedulers only.

Scope: FR-412-AC-9, FR-412-AC-10, FR-412-AC-11, FR-412-AC-12.

## Test Procedure

1. Over FR-407-AC-18's model, offer the `optimum` certificate with `post`
   values 1 and `1/2` at the two first activating states and `reset` value
   `1/2` at `s0` with policy `b`, for `>= rational(1, 2)` and
   `>= rational(3, 5)`.
2. Offer it again with `reset`'s `s0` value `1/4`, and again with `post`'s
   `1/2` replaced by `3/5`.
3. Under the workload choosing `a` and `b` with `1/2` each, offer the
   `ratio` certificate with numerator `3/8` and denominator `5/8` for
   `>= rational(3, 5)`, then with denominator 1.
4. Offer a `vacuous` certificate over the variant where `g` is unreachable,
   then over the model itself.
5. Offer a `conditional` certificate with no `conditional` member, a `ratio`
   one with no `denominator`, a `ratio` one under the objective `minimum`,
   and one with mode `"workload"`.
6. Add the action `stay` at `s0`. Offer `reset` values `1/2` at `s0` and at
   the state after `a` from which `g` is unreachable, policy `b`, ranks 1
   and 2; then values 0 with policy `stay`, with the reset choice, and with
   policy `b`.
7. On the step 6 model, offer a maximum `reset` for `<= rational(9, 10)`
   with `y(s0) = 1/2`, policy `b` and no row for `u`, the state after `a`
   from which `g` is unreachable; then with `u` at `1/2`; then a minimum
   `reset` whose `s0` choice is randomized over `a` and `b`.
8. Serialize and read two product states that differ only in `instance`,
   and two that differ only in `activation`.

## Expected Results

- Step 1: accepted at `1/2`; `threshold-missed` at `s0` for `3/5`.
- Step 2: `inequality-fails` at `{s0, part: reset}`; `inequality-fails`
  with `part` `post` at the activating state reached by `b`.
- Step 3: accepted; then `inequality-fails` with `part` `denominator`.
- Step 4: accepted; then `inequality-fails` at the initial state.
- Step 5: `malformed` at `conditional`, `denominator`, `mode` and `mode`.
- Step 6: accepted; `rank-broken` at `{s0, part: reset}` twice;
  `inequality-fails` at `{s0, part: reset}`.
- Step 7: `malformed` at `values`; `inequality-fails` at
  `{s0, part: reset}`; `malformed` at `policy`.
- Step 8: each pair keeps two distinct keys.
