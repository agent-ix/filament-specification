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

Verify enclosure-validated feasible plans, closure-certified infeasibility, incomplete and unsupported outcomes, unsound methods and the absence of optimality claims. Scope: FR-819-AC-1 to FR-819-AC-6.

## Test Procedure

1. Plan the `Fill`/`Drain` tank toward `Drain` with `dwell_grid` 1.
2. Plan toward `h >= 12`; tamper with the closure certificate.
3. Plan with `hybrid.max_solver_steps` 1; plan with no hybrid provider.
4. Plan with an `unclassified` provider; send a request without `dwell_grid`.
5. Plan step 1 under `always holds(h <= 9)`.
6. Replace the tank's point initial height with the interval `h in [0,1]`
   in mode `Fill`, retaining rate 1, `Fill` invariant `h <= 10`, transition
   guard `h >= 8`, and the goal of reaching `Drain`. Check the dwell-8
   candidate by the scheduled-model FR-193 item over the entire set.
   Then replace the rate with a nondeterministic parameter in `[1,2]`,
   fixed along each trajectory, and check that same candidate. Read the
   generated type nodes and the separate enclosure evidence in both cases.

## Expected Results

- Step 1: `feasible`, dwell 8, `elapsed` 8, `bound` null.
- Step 2: `infeasible`, `certified`; then `incomplete` with the rejected rule.
- Step 3: `incomplete` naming `hybrid.max_solver_steps`; `unsupported` naming `hybrid-dynamics`.
- Step 4: never `feasible`; refusal at `/dwell_grid`.
- Step 5: the plan is kept; no result is `optimal`.
- Step 6: generated trajectory-local `h` remains Rational, and hybrid
  `delay`/`elapsed` reference builtin Rational, with no invented unit node.
  For rate 1, a sound enclosure has heights `[8,9]` after the dwell and
  validates every initial point. With the nondeterministic rate, some
  trajectories would reach heights above 10 before that dwell ends;
  the candidate is not validated as feasible from a chosen representative
  point. The initial set and continuous enclosures remain FR-193 evidence,
  not scalar field values or a point-initial restriction.
