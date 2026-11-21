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
   fixed along each trajectory. For both variants declare
   `type One = Int[1,1];` and use `One` as the synthesis's length domain,
   with `dwell_grid` the typed Rational `1/1`, empty policy and sufficient
   limits and deadline. Read the generated nodes and enclosure evidence.
   For the rate variant, exhaust the candidate grid with the registered
   `flowpipe-outward-rounded` candidate validator and no accepted FR-652
   schedule-space infeasibility certificate. No resource stop, cancellation
   or provider failure occurs. The invariant bounds any possible `Fill`
   dwell by 10 (even `h=0`, rate 1 cannot remain longer); there are no
   further transitions before reaching `Drain`, and no other schedule
   lengths are in `One`. This finite outer bound adds no planner setting.

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
  the slowest initial trajectory requires dwell at least 8 for the guard,
  whereas the fastest from `h=1` requires dwell at most `9/2` for the
  invariant. No one-step grid schedule validates. After the declared
  bounded grid is exhausted without an accepted schedule-space
  infeasibility certificate, the exact result is `incomplete` with cause
  `grid-exhausted{dwell_grid: {type: rational, numerator: "1", denominator: "1"}}`,
  plan `null`, certificate `null` and bound `null`, never `feasible` or
  `infeasible` in this provider fixture. The initial set and continuous
  enclosures remain FR-193 evidence, not scalar field values or a
  point-initial restriction.
