---
id: FR-724
title: "Read a model without random parameters under every scheduler"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-407
    type: references
  - target: ix://agent-ix/filament-specification/FR-411
    type: references
---

# FR-724: Read a model without random parameters under every scheduler

## Description

When a probabilistic claim over a model subject is evaluated and an
operation argument of the model carries no random parameter declaration
(FR-405), the provider SHALL read that argument as a choice of the scheduler,
made together with the choice of action. Where no argument of the model is
random, every draw is the empty vector with probability 1, each deterministic
scheduler yields exactly one behaviour per initial state and binding, and
under every scheduler a probability claim's minimum and maximum are each 0 or
1. The claim is admitted and settled on that reading; a model without random
parameters is not refused and not warned about, because it is a
nondeterministic model whose probabilistic claims have true values.

## Inputs

A model subject (FR-405) some or all of whose operation arguments carry no
random parameter declaration, and a probabilistic claim over it (FR-407).

## Outputs

The FR-406 measure on behaviours for each initial state, binding and
resolved scheduler.

## Behavior

- An argument without a random declaration belongs to the scheduled identity
  (FR-405: operation, receiver, non-random argument vector), so each of its
  values gives another action; the draw distribution `D(s, a)` ranges over the
  random arguments only (FR-406 "Actions and draws").
- Under every scheduler (FR-406), a scheduler chooses among those actions, so
  the minimum of a reachability, until or bounded-event probability is
  attained by a deterministic scheduler. Where no argument is random, that
  scheduler yields one behaviour, the minimum is 0 or 1, and `probability >= θ`
  with `0 < θ < 1` holds exactly when the event holds on every behaviour of
  the model, the reading of the corresponding every-behaviour claim.
- When such a claim fails, its counterexample is the behaviour of a
  deterministic scheduler that attains the minimum (for `>=`) or the maximum
  (for `<=`), which replays as a behaviour of the model.
- Under a workload (FR-406), the workload's weights and the uniform identity
  share resolve the choice among actions (FR-406): each enabled scheduled
  identity of the chosen operation, one per non-random argument vector, has an
  equal share, so the workload still defines a Markov chain.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-724-AC-1 | On TC-352's `Link` with both random declarations removed, `probabilistic Every using inf on Link under every scheduler on origin { probability >= 0.9 [ eventually holds(m.delivered) ] }` has minimum 0 and maximum 1; it fails, and its counterexample is the behaviour that takes two sends with `lost = true` and never delivers. | Test (TC-1766) |
| FR-724-AC-2 | `Every` over `Link` with both random declarations has minimum `24/25` and holds. | Test (TC-1766) |
| FR-724-AC-3 | Under `Even` on `Link` with both random declarations removed, each send's `lost` takes `true` and `false` with probability `1/2` each, so the step probabilities at the initial state are `1/4` for each of the four actions and sum to 1. | Test (TC-1766) |

## Dependencies

- FR-405 declares random parameters; FR-406 gives the process and the
  scheduler reading this requirement applies to arguments without one.
