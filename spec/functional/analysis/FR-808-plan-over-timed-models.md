---
id: FR-808
title: "Plan over timed models with exact delays"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-800
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
  - target: ix://agent-ix/filament-specification/FR-418
    type: references
---

# FR-808: Plan over timed models with exact delays

## Description

When the grammar's model is a timed model (FR-415), the planner SHALL let
time pass between steps under the model's time source, SHALL compute
`elapsed` exactly, and SHALL search the finite region abstraction of a
dense-time model so that the bounded search stays finite and complete.

## Inputs

A timed model (tick or dense time source); the plan space of FR-800.

## Outputs

Plans whose steps carry delays, their exact `elapsed`, and, where a minimum
is an unattained infimum, the bound that says so.

## Behavior

### Tick time

Under `time M tick O period p unit u`, each invocation of the tick
operation `O` is an ordinary step; time advances only by it. A step's
`delay` is 0, and `elapsed` is the number of `O` steps times `p`, an exact
quantity of `u`.

### Dense time

- Under `time M dense unit u`, each step SHALL be a delay `d`, an exact
  non-negative rational quantity of `u`, followed by one invocation. During
  the delay every clock advances by `d`; every time invariant SHALL hold
  throughout the delay, an urgent state admits only `d = 0`, and the
  invocation's clock guards are read after the delay.
- `elapsed` SHALL be the exact sum of the plan's delays. The final state is
  reached at the last invocation; no trailing delay is part of a plan.
- The search SHALL explore the region abstraction of FR-418, scaled by the
  model's exact common denominator, in place of individual delays: a search
  node holds the region in place of the clock values. The abstraction is
  finite, so the search stays complete (FR-801).
- A reported plan SHALL carry concrete delays. Each delay is the least
  rational in its region-successor interval when that interval is closed
  below; when it is open below (a strict guard `x > c`), it is the midpoint
  between the open bound and the interval's next boundary. Replay (FR-650)
  re-applies these exact delays.

### Minima that are not attained

- For `minimize` over an objective that reads `elapsed`, the least value may
  be an infimum no plan attains, because of a strict guard. The planner
  SHALL then report `feasible` with the plan above and `bound` `{side:
  lower, value: <infimum>, attained: false}`, the infimum computed exactly
  from the region bounds; it SHALL NOT report `optimal` for a value no plan
  attains, and SHALL NOT round the infimum.
- When the least value is attained, the result is `optimal` with a
  `closure` certificate over the region graph, which the core checks by
  re-deriving each region successor.

### Policies under time

A policy clause under a timed profile is read over the plan's timed
behaviour (positions with their exact times, then the terminal stutter
with time advancing without bound, unless a time invariant of the final
state forbids it, in which case the plan is not admissible: it would
time-lock).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-808-AC-1 | Under `time M tick tick period 10 unit min`, a plan of three `tick` steps and two other steps has `elapsed` exactly `30 min`, and each step's `delay` is 0. | Test (TC-1958) |
| FR-808-AC-2 | Under dense time with clock `x` and operation `go` guarded `x >= 2`, `minimize Reach.elapsed` with goal `Reach.final.done` returns `optimal` with one step of delay `2` and `elapsed` `2`, and an accepted `closure` certificate. | Test (TC-1958) |
| FR-808-AC-3 | With the guard `x > 2` the result is `feasible`, `bound` `{side: lower, value: 2, attained: false}`, and the plan's delay is `5/2`, the midpoint of `(2, 3)`; it is never `optimal`. | Test (TC-1958) |
| FR-808-AC-4 | A time invariant `x <= 1` in a state where `go` requires `x >= 2` makes the goal `infeasible`; a final state whose time invariant forbids the terminal stutter is not admissible. | Test (TC-1958) |
| FR-808-AC-5 | Two runs of AC-3 give byte-identical outcome documents, with the delay written as the exact rational `5/2`. | Test (TC-1958) |

## Dependencies

- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) owns timed
  models; [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) timed
  claims; [FR-418](../../objects/interfaces/FR-418-zone-certificate.md) the
  region and zone abstraction.
