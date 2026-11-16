---
id: FR-712
title: "Analyze state-graph claims over a timed model subject"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-390
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-391
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-393
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-418
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-852
    type: traces_to
---
# FR-712: Analyze state-graph claims over a timed model subject

## Description

When a `possible`, `always possible` or `unique path` clause is checked over
a timed model subject (FR-415), the analyzer SHALL evaluate it over the
subject's timed state graph, defined below, and SHALL NOT evaluate it over
the subject's untimed graph. FR-390 defines the forms over an untimed
subject; this requirement gives their meaning when the subject has clocks,
clock guards and time invariants.

### Decision: nodes are timed states, paths are discrete-step sequences

A timed subject's behaviour alternates delays, during which clocks advance
and every time invariant must keep holding, with discrete steps, whose clock
guards must hold. Dropping the clocks would count as reachable a node that
can only be reached by waiting past a deadline, so the graph keeps them: a
node is a timed state, a discrete state with a clock valuation. State-graph
predicates read model state, never clocks (FR-390), so they are evaluated on
a node's discrete state. A path is the sequence of discrete steps it takes;
two runs that take the same steps with different delays are one path, since
a delay is not a choice of the model's operations, and `unique path` asks
how many ways the operations lead from `X` to `Y`.

## Inputs

The checked clause, its form and predicates, its `over` binding, and the
timed model subject: initial states with their clock valuations, universes,
proof bounds, clock guards and time invariants.

## Outputs

One result per claim instance, settled by FR-391, with its FR-393 evidence in
timed form.

## Behavior

- **Timed state graph.** The nodes SHALL be the timed states reachable from an
  initial state of the subject. A delay edge `(s, v) -d-> (s, v + d)`
  exists for a non-negative exact rational `d` when every time invariant of
  `s` holds at `v + t` for every `t` in `[0, d]`; a step edge
  `(s, v) -t-> (s', v')` exists for each discrete step `t` FR-390 gives over
  `s` whose clock guards hold at `v`, `v'` being `v` with the step's clock
  resets applied and every time invariant of `s'` holding at `v'`.
- **Forms.** With `E(P)` the nodes from which some finite path of delay and
  step edges reaches a node whose discrete state satisfies `P`:
  `possible` holds when every initial node is in `E(P)`; `always possible`
  holds when every reachable node whose discrete state satisfies `Q` is in
  `E(P)`; `unique path` holds when, from every reachable node whose discrete
  state satisfies `X`, exactly one sequence of step edges, each preceded by
  some admissible delay, reaches a node satisfying `Y`, and a sequence that
  can repeat a cycle counts as more than one.
- **Symbolic decision.** A conforming analyzer SHALL decide these over zones
  (FR-416): `E(P)` as the least fixpoint of timed and discrete predecessors of
  the `P` zones, and the forward reachable zones by FR-416's zone search,
  with a result equal to the definition above over the concrete timed states.
- **Evidence.** A witness or counterexample path SHALL be concretized with
  exact rational delays by FR-416's canonical choice, carried in FR-393's
  path form with each step's delay before it, and replay with no provider
  present. A `unique path` refutation carries the two step sequences, or the
  cycle and the node it returns to.
- **Proof basis.** A proof settles with basis `zone-certified` and an FR-418
  certificate covering the reachable zones and, for `always possible`, the
  `E(P)` zones, or with basis `exhaustive` by digitization of an all-closed
  subject (FR-416).
- **Routing.** A backend matches a state-graph item over a timed subject only
  when it advertises the form in `state_graph_forms` and also matches the
  subject's timed items under FR-290's matching rules, so a backend that
  decides only untimed graphs is never a candidate.

## Acceptance Criteria

The vector is TC-369's `Rpc` model with `T = 4 ms`.

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-712-AC-1 | `possible` of `phase = 2` holds with a witness of `send` then `reply` whose delay before `reply` is an exact rational in `[1, 3]` and replays; `possible` of `phase = 3` does not hold, because every run takes `reply` by `x = 3` and `timeout` needs `x >= 4` while `phase` is 1. | Test (TC-1742) |
| FR-712-AC-2 | `always possible` of `phase = 0` holds; with `reset` removed it is refuted by a path ending at `phase` 2, and the proof of the first carries the `E(P)` zones in its certificate. | Test (TC-1742) |
| FR-712-AC-3 | `unique path` from `phase = 0` to `phase = 2` is refuted with the cycle through `reset`; without `reset` it holds, the runs with different delays before `reply` counting as one path. | Test (TC-1742) |
| FR-712-AC-4 | A backend that advertises the three forms but matches no timed item is not a candidate for these items, and an item naming it settles `unsupported` `unsupported-requested-capability`. | Test (TC-1742) |

## Dependencies

- FR-390 and FR-391 own the forms and their settlement; FR-393 owns the
  evidence.
- FR-415 and FR-416 own timed models, zones and concretization; FR-418 owns
  the zone certificate.
