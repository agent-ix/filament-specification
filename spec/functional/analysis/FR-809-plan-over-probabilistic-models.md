---
id: FR-809
title: "Plan over probabilistic models with exact probabilities and expectations"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-800
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-406
    type: depends_on
---

# FR-809: Plan over probabilistic models with exact probabilities and expectations

## Description

When the grammar's model declares random parameters (FR-405), the planner
SHALL plan a fixed sequence of actions whose random arguments are drawn as
the model states, SHALL require every rule to hold on every outcome of
positive probability, and SHALL compute the goal's probability and the
objective's expectation exactly as rationals.

## Inputs

A probabilistic model and its draw distributions (FR-405, FR-406); the plan
space of FR-800.

## Outputs

An open-loop plan, its exact `probability`, and its exact expected
objective value.

## Behavior

### The plan

- A step SHALL be an *action*: an operation and its non-random arguments.
  The step's random arguments are drawn from the action's draw distribution
  `D(s, a)` (FR-406) at the state `s` the step is applied to; they are not
  chosen by the plan. A plan is the same action sequence on every outcome:
  it does not observe the draws.
- Where a drawn vector admits several post-states, the choice SHALL be
  adversarial: the plan must be admissible whichever post-state is chosen,
  and the objective takes its worst value (largest for `minimize`, smallest
  for `maximize`) over the choices.
- The plan's *outcomes* are the paths of states its draws and choices
  produce; the exact probability of a path is the product of its draw
  probabilities.

### Admissibility, goal and objective

- A plan is admissible when, on every outcome of positive probability,
  every step is enabled, every model rule holds and every policy clause is
  kept (FR-803).
- `probability` SHALL be the exact rational probability, over the outcomes,
  that the goal holds, with adversarial choices taken against the goal.
  When neither the goal nor the objective reads `probability`, a plan
  reaches the goal only if `probability` is 1. When the objective reads
  `probability`, the goal block is the event whose probability is measured,
  and a plan reaches the goal when `probability` is greater than 0.
- The objective's value SHALL be its exact expectation over the outcomes,
  each outcome weighted by its probability, after the adversarial choice.
- A search node SHALL be the exact distribution over model states the plan
  prefix induces (its support with rational weights) together with the
  monitors and accumulators of FR-801; the node set is finite because the
  support graph is finite.
- A floating-point objective over a probabilistic model is averaged in the
  outcomes' canonical order under the floating-point profile, as FR-802
  states for sums.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-809-AC-1 | With an action `try` that sets `done` with probability 1/2 (a random Boolean) and `retry` that sets it with probability 1, the goal `Reach.final.done` is reached with `probability` 1 by `retry` (length 1) and not by `try`, and the result is `optimal` `retry`. | Test (TC-1959) |
| FR-809-AC-2 | `maximize Reach.probability` with goal `Reach.final.done` over plans of length 1 to 2 using only `try` returns `optimal` `try, try` with objective exactly `3/4`. | Test (TC-1959) |
| FR-809-AC-3 | A policy `always holds(not c.broken)` where `try` breaks the device with probability 1/10 excludes every plan containing `try`, although 9/10 of its outcomes keep the policy. | Test (TC-1959) |
| FR-809-AC-4 | `minimize Reach.steps.sum(s => s.after.cost)` over draws of cost 1 or 3 with probability 1/2 each gives the exact expectation 2 per step, written as an exact value. | Test (TC-1959) |

## Dependencies

- [FR-405](FR-405-declare-probabilistic-models.md) and
  [FR-406](FR-406-interpret-probabilistic-models.md) own probabilistic models
  and draw distributions.
