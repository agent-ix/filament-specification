---
id: TC-1966
title: "Plans over hybrid models validated by enclosures"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-819
    type: verifies
---

# TC-1966: Plans over hybrid models validated by enclosures

## Description

Verify enclosure-validated feasible plans, closure-certified infeasibility, incomplete and unsupported outcomes, unsound methods and the absence of optimality claims. Scope: FR-819-AC-1 to FR-819-AC-5.

## Test Procedure

1. Plan the `Fill`/`Drain` tank toward `Drain` with `dwell_grid` 1.
2. Plan toward `h >= 12`; tamper with the closure certificate.
3. Plan with `hybrid.max_solver_steps` 1; plan with no hybrid provider.
4. Plan with an `unclassified` provider; send a request without `dwell_grid`.
5. Plan step 1 under `always holds(h <= 9)`.

## Expected Results

- Step 1: `feasible`, dwell 8, `elapsed` 8, `bound` null.
- Step 2: `infeasible`, `certified`; then `incomplete` with the rejected rule.
- Step 3: `incomplete` naming `hybrid.max_solver_steps`; `unsupported` naming `hybrid-dynamics`.
- Step 4: never `feasible`; refusal at `/dwell_grid`.
- Step 5: the plan is kept; no result is `optimal`.
