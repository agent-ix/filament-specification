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
only when an FR-193 enclosure proof of `always not goal` over the
schedule space shows that no admissible plan exists within the bound, and SHALL otherwise report `incomplete` naming the limit that
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

### From a schedule to an FR-193 item

For hybrid plans, `delay` and `elapsed` have the existing builtin
`scalar_type`/`rational` type, in the model's own time coordinates, as
`dwell_grid` does. This is an explicit hybrid override of FR-800's timed
quantity typing: FR-481 carries no hybrid time-unit node, so no unit or
Quantity node is inferred. Non-hybrid timed planning keeps its unit-bearing
quantity rules. A later language form declaring a hybrid unit would need
to specify its conversion at this override.

FR-800/FR-322's hybrid-state record types describe a point on each admitted
trajectory. The translation below binds `initial` to that trajectory's
initial point and `before`/`after`/`final` to its state at the corresponding
schedule boundaries. Those typed points are universally quantified over
the entire initial set and all admitted nondeterministic parameters, as
the execution rule above requires. They do not materialize the initial
set or enclosure in a Rational field. The scheduled-model FR-193 item and
its sound enclosure evidence discharge the universal goal/policy checks;
a representative or sampled point proves neither. This uses the existing
FR-193 hybrid state/trajectory semantics and the goal translation below,
without adding a value form for an enclosure.

- For a candidate schedule `(d_1, t_1), …, (d_n, t_n)` the planner SHALL
  build the *scheduled model*: the hybrid declaration plus a dwell clock `τ`
  (rate 1, reset to 0 by every transition), an elapsed clock `e` (rate 1)
  and a step counter `k` (initially 0, incremented by every transition). In
  the mode current before step `i`, the scheduled transition `t_i` gets the
  extra guard `k = i - 1 and τ = d_i`, the mode gets the extra invariant
  `k = i - 1 implies τ <= d_i`, and every other transition is removed. The
  last transition `t_n` enters a frozen copy of its target mode whose flows
  are all 0 (`e` included), so the state after the last step stays the
  plan's final state, which is FR-803's terminal stutter.
- Every policy safety clause SHALL become an absorbing flag `bad`, set at
  the first instant that breaks it. Mode invariants stay invariants: a
  dwell they forbid leaves the schedule unable to complete.
- The candidate SHALL be checked as one FR-193 hybrid item over the
  scheduled model, of kind `temporal-satisfaction` with the
  `hybrid-dynamics` requirement, horizon `H = d_1 + … + d_n` and jump bound
  `n`, for the clause `always holds(not bad and ((k = i - 1 and τ = d_i)
  implies guard(t_i)) and (k = n implies goal)) and eventually[0, H]
  holds(k = n)`, conjoined with every non-safety policy clause, the goal's
  reads of `length`, `elapsed` and `final` read as `k`, `e` and the current
  state. The `eventually` conjunct requires every trajectory to complete the
  schedule, so a schedule that a mode invariant blocks is never validated.

### Feasible plans

- The planner SHALL propose candidate schedules whose dwells are multiples
  of the request's `dwell_grid`, a positive exact rational written in the
  FR-181 typed `rational` or `decimal` form FR-805 states and required for a
  hybrid plan, each dwell bounded by the horizon the mode invariants and the
  declaration give, enumerated in FR-804's plan order.
- A candidate is *validated* only when its item settles `proved` under a
  method the FR-193 method table makes sound (`flowpipe-outward-rounded`
  or `inductive-enclosure`). A point estimate, a simulation or an
  `unclassified` method validates nothing.
- The first validated candidate SHALL be reported `feasible`, with its
  item's certification: `certified` when the core accepted the provider's
  FR-652 certificate, otherwise the provider's FR-331 certification.
  `bound` is `null`; no optimality is claimed, because the dwell grid does
  not cover the continuum of schedules.

### Infeasible plans

- The planner SHALL report `infeasible` only from an FR-193 proof of
  `always not goal` over the whole schedule space: the *schedule-space
  model* is the hybrid declaration with `e`, `k` and `bad` as above, every
  transition kept with any dwell, and transitions disabled once
  `k = max D`. Its item has the clause
  `always holds(not (k in D and not bad and goal))`, horizon the
  declaration's time horizon and jump bound `max D`.
- That item SHALL be proved by an `inductive-enclosure` or
  `flowpipe-outward-rounded` method whose FR-652 certificate the core
  accepts; `infeasible` carries the certificate with certification
  `certified`. Non-safety policies are left out of this item; dropping a
  constraint can only enlarge the set of admissible plans, so a proof that
  none reaches the goal stays sound. A rejected certificate, or a proof without one, leaves the
  result `incomplete`, recording the rejected rule and entry when there is
  one.

### Incomplete and unsupported

- When neither a validated candidate nor a certified infeasibility proof exists, the
  result SHALL be `incomplete`, with cause `limit-reached` naming the limit
  that stopped the search: a `plan_search` limit (FR-801), the provider's
  `hybrid.max_solver_steps`, or `hybrid.max_solver_time` as `timed-out`
  (FR-193 "Solver budgets"); a candidate set exhausted on the grid with no
  infeasibility proof is `incomplete` with cause `grid-exhausted{dwell_grid}`, naming the
  request member whose refinement may find a plan; `dwell_grid` is a search
  setting, not an FR-461 limit.
- With no hybrid candidate the request SHALL settle `unsupported`,
  `unsupported_projection`/`unsupported-requested-capability`, naming
  `temporal-satisfaction` and `hybrid-dynamics`, as FR-193 states.
- A hybrid plan request with no `dwell_grid` refuses
  `invalid_runtime_input`/`missing-member` at `/dwell_grid` (a pointer into
  the plan request, prefixed by a host surface as FR-805 states).
- With no hybrid candidate in the engine set the caller runs the plan
  search with, the result is `unsupported`. The engine set is a library
  parameter of the planner and of its host surface, never a request member
  (FR-805): a request cannot select an engine the caller did not register.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-819-AC-1 | Over a tank with mode `Fill` (`h' = 1`, invariant `h <= 10`, transition to `Drain` when `h >= 8`) and `Drain` (`h' = -2`, invariant `h >= 0`) from `h = 0` in `Fill`, the goal `Reach.final.mode = Drain` with `dwell_grid` `{"type": "rational", "numerator": "1", "denominator": "1"}` returns `feasible` with the one step of dwell 8 into `Drain`, `elapsed` 8, `bound` `null`, validated by a `flowpipe-outward-rounded` enclosure. | Test (TC-1966) |
| FR-819-AC-2 | The goal `Reach.final.h >= 12` returns `infeasible` from an `inductive-enclosure` proof of `always not goal` over the schedule space, whose FR-652 certificate bounds `h` within `[0, 10]` and is accepted, certification `certified`; the certificate with one reset image removed is rejected and the result is `incomplete`. | Test (TC-1966) |
| FR-819-AC-3 | With `hybrid.max_solver_steps` 1 the result is `incomplete` with `limit-reached` naming `hybrid.max_solver_steps`; with no hybrid provider registered it is `unsupported` naming `hybrid-dynamics`. | Test (TC-1966) |
| FR-819-AC-4 | A provider that answers with an `unclassified` method validates no candidate, and the result is never `feasible` on its word; a request without `dwell_grid` refuses at `/dwell_grid`. | Test (TC-1966) |
| FR-819-AC-5 | No hybrid plan result is `optimal`, and a policy `always holds(h <= 9)` keeps the AC-1 plan, which the enclosure shows never exceeds 8. | Test (TC-1966) |
| FR-819-AC-6 | TC-1966's interval-initial tank and nondeterministic-rate variant retain Rational trajectory-local state fields and builtin Rational delay/elapsed coordinates, with uncertainty represented by FR-193 enclosure evidence. A sound enclosure over every initial point validates the dwell-8 candidate for `h in [0,1]` and rate 1; with rate in `[1,2]`, the same candidate cannot be validated from a representative point because some trajectories violate `Fill`'s invariant. No point-initial restriction or invented hybrid unit is introduced. | Test (TC-1966) |

## Dependencies

- [FR-193](FR-193-analyze-continuous-hybrid-models.md) owns hybrid models,
  routing, solver methods and budgets;
  [FR-652](FR-652-check-hybrid-enclosure-and-witness-certificates.md) owns the
  core's enclosure certificate check.
- [FR-800](FR-800-define-the-plan-and-goal-model.md) to
  [FR-806](FR-806-refuse-plan-requests-with-located-causes.md) own the plan
  value, limits, order, outcome and refusals.
