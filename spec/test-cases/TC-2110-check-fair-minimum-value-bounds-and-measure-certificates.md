---
id: TC-2110
title: "Check fair-minimum, value-bounds and measure certificates"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-412
    type: verifies
---
# TC-2110: Check fair-minimum, value-bounds and measure certificates

## Description

The checker accepts an unbounded reachability certificate under a fairness
set only after recomputing the fair end components itself, accepts the
`lower` and `upper` certificates of a value-bounds proof by their own
conditions, and checks a finite-horizon measure certificate against the
payoff it computes from each decided state's key.

Scope: FR-412-AC-13, FR-412-AC-14, FR-412-AC-15.

## Test Procedure

1. Over `W` (`wait` loops at `s`; `go` reaches `g` or `f` with `1/2`
   each) under `fair strong go`, minimum, offer the `exact` certificate
   `s` `1/2`, `g` 1, `f` 0, ranks 1, 0, 0 for `>= 1/2`; then with `s` at
   0 and at `1/4`; then the `1/2` certificate with its fairness set removed.
2. On the same model offer `lower` `s` `2/5` without ranking for `>= 2/5`,
   `lower` `s` `3/5`, `upper` `s` `3/5` with policy `go` and rank 1, and
   `upper` `s` `1/4` with policy `wait`.
3. Add `drift` from `s` to `u`, whose only action loops at `u`. Offer
   values `s` 0, `u` 0 with ranks 1, 0; then `u` at `1/2`.
4. Over the workload chain `s` to `s`, `g`, `f` with `1/3` each, offer
   `lower` `7/16` (rank 1) for `>= 2/5`, `upper` `9/16` for `<= 3/5`,
   `lower` `5/8`, `upper` `7/16`, `lower` `7/16` for `>= 1/2`, and `lower`
   `7/16` without `ranking`.
5. Over `W` without fairness, maximum, offer `upper` `3/5` for `<= 3/5`,
   `lower` 1 with policy `wait`, and `lower` `7/16` with policy `go`.
6. Over `Retry`, offer the `ratio` certificate for `quantile 0.5 of (steps
   from holds(true) until holds(delivered) within 4) <= 2` under its
   workload; over the `fast`/`slow` variant under every scheduler, the
   level-0.4 shifted `exact` certificate, the level-0.5 one, and the
   level-0.4 one with its initial value replaced by `9/40`.
7. Over `Retry` under its workload, check the `mean of fraction
   holds(delivered) over [0, 2]` certificate.

## Expected Results

- Step 1: accepted; `rank-broken` at `s` twice; `rank-broken` at `s`.
- Step 2: accepted; `inequality-fails` at `s`; accepted as the upper side;
  `rank-broken` at `s`.
- Step 3: accepted; `inequality-fails` at `u`.
- Step 4: accepted; accepted; `inequality-fails` at `s` twice;
  `threshold-missed` at `s`; `malformed` at `ranking`.
- Step 5: accepted; `rank-broken` at `s`; accepted as the lower side.
- Step 6: accepted (numerator `3/4`, denominator 1); accepted with side
  `AtLeast` (`3/80`); `threshold-missed` at the initial state (`−1/16`);
  `inequality-fails` at the initial state.
- Step 7: accepted with initial value `5/12`.
