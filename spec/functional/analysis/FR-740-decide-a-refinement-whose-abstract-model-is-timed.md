---
id: FR-740
title: "Decide a refinement whose abstract model is timed"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-377
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
---
# FR-740: Decide a refinement whose abstract model is timed

## Description

A refinement (FR-377) whose abstract model is timed (FR-415) SHALL be
admitted over a timed or an untimed concrete model. It SHALL be decided
exactly by the symbolic provider (FR-416), as follows.

- **Abstract clocks are derived.** Every abstract clock is 0 in every
  initial state, advances with every delay, and is reset by the abstract
  operation that a step row maps the concrete step to. Its value along a
  concrete timed behaviour is therefore fixed by the step map. No mapping
  row maps an abstract clock. A row whose abstract field is a clock is
  refused `invalid_model_binding`/`malformed-declaration` at the row.
- **Product.** The product's states are (discrete product state, zone over
  the concrete clocks and the abstract clocks).
  - A delay advances every clock equally. It is constrained by the
    concrete target's and the mapped abstract state's time invariants, and
    a delay that leaves the abstract invariant fails the step rules.
  - A discrete step fails when the mapped abstract operation's clock guard
    is false at every point of the zone. The abstract resets are applied
    after the step.
- **Untimed concrete model.** An untimed concrete model admits every delay
  between its steps. The refinement holds when some delay assignment that
  the abstract model admits exists along every concrete behaviour. The zone
  along a path is exactly that set of assignments, and a path whose zone
  becomes empty fails.
- **Results.** A failure refutes under the divergence rule that FR-377 states
  for a timed concrete subject. Counterexamples carry exact rational delays
  that FR-181 timed replay checks on both sides. A proof carries an FR-418
  zone certificate whose clocks include the abstract clocks.

Timed language inclusion is undecidable in general only because an abstract
side may reset a clock at a point of its own choosing. Under a refinement's
step map, every abstract reset is fixed by a concrete step, so the decision
is reachability in a finite product.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-740-AC-1 | A timed concrete model whose `send` needs `x >= 2` refines a timed abstract model whose `send` needs `y >= 1`, with both clocks reset by `send`; the safety half is proved with a certificate over both clocks. With the concrete guard `x >= 0` it is refuted, and the counterexample delay below 1 replays. | Test (TC-1787) |
| FR-740-AC-2 | An abstract invariant `y <= 3` against a concrete model that waits 4 is refuted at the delay that crosses 3. A row mapping `y` is refused at the row. | Test (TC-1787) |
| FR-740-AC-3 | An untimed concrete model refines the timed abstract model of AC-1. With an abstract invariant `y <= 0` and guard `y >= 1`, it is refuted by an empty zone. | Test (TC-1787) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md): model refinement.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md): timed
  models and their symbolic provider.
