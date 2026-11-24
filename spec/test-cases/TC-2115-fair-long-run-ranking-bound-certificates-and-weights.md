---
id: TC-2115
title: "Fair long-run certificates carry a progress ranking, bound certificates and weights"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-412
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-411
    type: verifies
---
# TC-2115: Fair long-run certificates carry a progress ranking, bound certificates and weights

## Description

Verify the progress ranking of a long-run certificate under a fairness set, the
bound certificates of an unattained fair optimum, the one-entry-per-component
rule and the weighted `Every` forms. Scope: FR-412-AC-29 to FR-412-AC-33 and
FR-411-AC-18.

## Test Procedure

1. Check the fair `maximum` certificate with the ranking of FR-412-AC-29, then
   the cyclic-ranking, removed-ranking, nonzero-component-rank, no-fairness
   and `workload` variants, and the `minimum` variant.
2. Check the `lower` and `upper` bound certificates of FR-412-AC-30 and their
   defects.
3. Check the component-coverage cases of FR-412-AC-31.
4. Settle the weighted `Every` requests of FR-412-AC-32, and check the mixed-class component of FR-412-AC-33.

## Expected Results

- Step 1: accepted; `rank-broken`, `rank-missing` and `malformed` at the stated
  member or state.
- Step 2: accepted; `threshold-missed`, `malformed` and `inequality-fails` as
  stated.
- Step 3: `malformed` at `{"member": "components"}` for each coverage defect.
- Step 4: `proved` or `refuted` with certificates; none `unsupported`.
