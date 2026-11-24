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

**Hidden candidates carry their own clock blocks.** When the declaration
leaves an abstract field hidden (FR-378), each alive candidate is an abstract
state together with its own **clock block**, the valuation of the abstract
clocks that candidate has run through the concrete behaviour. Candidates that
share a discrete state and a clock block are one candidate. At one concrete
step the abstract operation each candidate takes may reset different clocks
or carry different ones, because the candidates differ in the transition row
that applies to them. The step is therefore applied per candidate: a delay
advances every clock of every block equally; the operation's clock guard is
tested against that candidate's block only; a candidate whose guard is false
at every point of the zone dies and the others stay; each surviving
candidate's resets and carries are applied to its own block after the step.
The product state is the discrete product state, the zone over the concrete
clocks and every alive candidate's abstract-clock block, and the candidate
set is empty (the step fails `AbstractClockGuard` or `NoAbstractMatch`, FR-379)
only when every candidate has died. A reset by one candidate never changes
another candidate's block, and a carry never shares a block between
candidates. The clock block is part of the candidate's state key, ordered
with the abstract state keys of FR-378's product key, so two candidates of
one abstract state with different blocks are two entries and a certificate
position (FR-418) lists both.

**The abstract tick source belongs to the abstract side.** When the abstract
model is timed with `time tick O period p` (FR-415), its abstract time is the
period times the number of abstract `O` steps taken so far, whatever the
concrete model's own time domain. The concrete step that a row maps to `O` is
an abstract tick; a concrete delay, or a concrete tick of a different source,
advances no abstract clock. A concrete tick source never supplies abstract
time and an abstract tick source never supplies concrete time. The liveness
half over such an abstract model reads time divergence on the abstract side
as infinitely many `O` steps along the candidate run, an acceptance set of
the candidate product beside `F_A`, and it is decided exactly (FR-378), never
refused for the abstract side's tick source.

**Undelayed failing steps in a certificate.** A certificate edge is
*undelayed failing* when its step fails at every point of its source zone with
no delay (the abstract guard is false throughout, or the target violates the
abstract invariant). An undelayed failing edge reachable from a certified
initial position refutes the safety half, so no proof certificate contains
one. An honest certificate of another half or form (the zone certificate of
the liveness half over candidates that die, FR-418 "Refinement part") MAY
contain such an edge on a path the half does not need to reach; the checker
SHALL accept it when recomputation produces the same edge with `taken`
`null`, SHALL NOT reject it as `extra-edge` or `certificate-rejected` for being
failing, and settles the item with the verdict the rest of the certificate
supports. A checker that cannot recompute such an edge settles
`inconclusive` and never `proved`; it never turns an undelayed failing edge
into a proof or a refutation without recomputing it.

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
| FR-740-AC-4 | A timed concrete model with a hidden abstract field `mode` whose two candidates both take the abstract `send` but only the `mode = true` row resets `y`: after `send` at concrete delay 2, the `true` candidate's block holds `y = 0` and the `false` candidate's holds `y = 2`; a following abstract guard `y >= 1` kills the `true` candidate when the next step is taken at delay 0 and keeps the `false` one, so the refinement is not refuted; when both blocks fail the guard the step fails and the counterexample replays. Two candidates of one abstract state with different blocks are two entries in the certificate position. Resetting one candidate's clock leaves the other's block unchanged. | Test (TC-1787) |
| FR-740-AC-5 | An abstract model with `time tick Step period 5` and a concrete model with dense time: a concrete delay of 100 advances no abstract clock, a concrete step mapped to `Step` advances abstract time by 5, and an abstract guard `y >= 10` is false after one such step and true after two. A concrete model with its own tick source over the same abstract tick source never supplies abstract time. The liveness half of a hidden-field declaration over this abstract model settles `proved` or `refuted` with a replaying lasso, with the abstract time-divergence acceptance set requiring infinitely many `Step` steps, and never `unsupported`. | Test (TC-1787) |
| FR-740-AC-6 | A zone certificate whose refinement part holds an undelayed failing edge on a path the half does not need to reach, with `taken` `null`, is accepted and settles the verdict its other parts support; the same edge with a non-null `taken`, or one the recomputation does not produce, settles `inconclusive`, `certificate-rejected`; a safety certificate with an undelayed failing edge reachable from an initial position is never accepted as a proof. | Test (TC-1787) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md): model refinement.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md): timed
  models and their symbolic provider.
