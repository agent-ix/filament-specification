---
id: FR-819
title: "Plan over hybrid models with enclosure-validated plans"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-800
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-193
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-652
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-194
    type: references
---

# FR-819: Plan over hybrid models with enclosure-validated plans

## Description

When a `synthesis` declaration's `grammar` names a `hybrid` declaration,
the planner SHALL search schedules of dwell times and discrete transitions,
SHALL report a plan only after a sound FR-193 enclosure of its whole
execution shows that every rule and the goal hold, SHALL report `infeasible`
only when an enclosure closure proves that no admissible plan exists within
the bound, and SHALL otherwise report `incomplete` naming the limit that
stopped it. A hybrid plan is never `optimal`.

## Inputs

The checked package; the `synthesis` declaration over a hybrid declaration;
the initial set (FR-193 `initial`); the request's `dwell_grid`, policy
clauses and limits; the negotiated hybrid provider (FR-193 "Routing a
hybrid item").

## Outputs

A `feasible`, `infeasible`, `incomplete` or `unsupported` plan outcome
(FR-805) with the enclosure evidence behind it.

## Behavior

### The plan over a hybrid model

- FR-194's `grammar` resolution SHALL also admit a `hybrid` declaration,
  which selects hybrid plan synthesis; `domain D` is an integer range type of
  admitted plan lengths, as for a state model.
- A step SHALL be a dwell `d`, an exact non-negative rational number of the
  model's time unit during which the continuous state follows the current
  mode's flow under its invariant, followed by one discrete transition of
  that mode, taken where its guard holds and applying its reset. A plan of
  length `n` has `n` transitions; `elapsed` is the exact sum of its dwells;
  a step record carries `delay` (the dwell) and `operation` (the
  transition's target mode).
- The plan is open-loop: it fixes the transitions and dwells, and its
  execution is every trajectory those admit from the initial set, under
  every value of a nondeterministic parameter the model declares.

### Feasible plans

- The planner SHALL propose candidate plans whose dwells are multiples of
  the request's `dwell_grid`, a positive exact rational required for a
  hybrid plan, each dwell bounded by the time horizon the mode invariants
  and the declaration give, enumerated in FR-804's plan order.
- Each candidate SHALL be validated as an FR-193 bounded-reachability item
  over its fixed schedule, on the negotiated hybrid provider: the candidate
  is *validated* only when a method the FR-193 method table makes sound for
  disjointness (`flowpipe-outward-rounded`, or `inductive-enclosure`)
  returns an enclosure of the whole execution in which every mode
  invariant, every policy clause and the guard of each scheduled transition
  hold on the whole enclosure, and the goal holds on the whole final
  enclosure. A point estimate, a simulation or an `unclassified` method
  validates nothing.
- The first validated candidate SHALL be reported `feasible`, with the
  provider's enclosure and its certification: `certified` when the provider
  returned an FR-652 certificate the core accepted, otherwise the provider's
  FR-331 certification. `bound` is `null`; no optimality is claimed, because
  the dwell grid does not cover the continuum of schedules.

### Infeasible plans

- The planner SHALL report `infeasible` only when it holds an enclosure
  closure: an outward-rounded enclosure of every trajectory from the initial
  set under every transition and every dwell (not only grid dwells), up to
  `max D` transitions and within the horizon, closed under each mode's flow
  and each reset, whose final-state part is disjoint from the goal or in
  which every path breaks a rule. The closure SHALL be an FR-652 certificate
  the core checks; `infeasible` carries it with certification `certified`.
- A closure the core rejects SHALL demote the result to `incomplete`,
  recording the rejected rule and entry.

### Incomplete and unsupported

- When neither a validated candidate nor an accepted closure exists, the
  result SHALL be `incomplete`, with cause `limit-reached` naming the limit
  that stopped the search: a `plan_search` limit (FR-801), the provider's
  `hybrid.max_solver_steps`, or `hybrid.max_solver_time` as `timed-out`
  (FR-193 "Solver budgets"); a candidate set exhausted on the grid with no
  closure is `incomplete` with cause `grid-exhausted{dwell_grid}`, naming the
  request member whose refinement may find a plan; `dwell_grid` is a search
  setting, not an FR-461 limit.
- With no hybrid candidate the request SHALL settle `unsupported`,
  `unsupported_projection`/`unsupported-requested-capability`, naming
  `temporal-satisfaction` and `hybrid-dynamics`, as FR-193 states.
- A hybrid plan request with no `dwell_grid` refuses
  `invalid_runtime_input`/`missing-member` at `/dwell_grid`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-819-AC-1 | Over a tank with mode `Fill` (`h' = 1`, invariant `h <= 10`, transition to `Drain` when `h >= 8`) and `Drain` (`h' = -2`, invariant `h >= 0`) from `h = 0` in `Fill`, the goal `Reach.final.mode = Drain` with `dwell_grid` 1 returns `feasible` with the one step of dwell 8 into `Drain`, `elapsed` 8, `bound` `null`, validated by a `flowpipe-outward-rounded` enclosure. | Test (TC-1966) |
| FR-819-AC-2 | The goal `Reach.final.h >= 12` returns `infeasible` with an accepted enclosure-closure certificate bounding `h` within `[0, 10]`, certification `certified`; a closure with one reset image removed is rejected and the result is `incomplete`. | Test (TC-1966) |
| FR-819-AC-3 | With `hybrid.max_solver_steps` 1 the result is `incomplete` with `limit-reached` naming `hybrid.max_solver_steps`; with no hybrid provider registered it is `unsupported` naming `hybrid-dynamics`. | Test (TC-1966) |
| FR-819-AC-4 | A provider that answers with an `unclassified` method validates no candidate, and the result is never `feasible` on its word; a request without `dwell_grid` refuses at `/dwell_grid`. | Test (TC-1966) |
| FR-819-AC-5 | No hybrid plan result is `optimal`, and a policy `always holds(h <= 9)` keeps the AC-1 plan, which the enclosure shows never exceeds 8. | Test (TC-1966) |

## Dependencies

- [FR-193](FR-193-analyze-continuous-hybrid-models.md) owns hybrid models,
  routing, solver methods and budgets;
  [FR-652](FR-652-check-hybrid-enclosure-and-witness-certificates.md) owns the
  core's enclosure certificate check.
- [FR-800](FR-800-define-the-plan-and-goal-model.md) to
  [FR-806](FR-806-refuse-plan-requests-with-located-causes.md) own the plan
  value, limits, order, outcome and refusals.
