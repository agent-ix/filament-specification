---
id: FR-421
title: "Check probabilistic timed claims exactly through digital clocks"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
  - target: ix://agent-ix/filament-specification/FR-641
    type: references
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-411
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
---

# FR-421: Check probabilistic timed claims exactly through digital clocks

## Description

When a `probabilistic-satisfaction` item over a timed subject names `exact`
evidence, the exact provider SHALL read the subject through digital clocks as
a probabilistic timed automaton.

When the exact provider settles such an item through digital clocks, it SHALL
settle only a verdict that holds on the dense model.

When such an item's subject has a continuous delay distribution, whose
probabilities exact evidence cannot state, the exact provider SHALL settle the
item `unsupported` with `delay-distribution{operation}`; every other subject and
claim of this class SHALL settle an exact verdict.

## Inputs

A timed subject ([FR-415](FR-415-declare-clocks-and-timed-models.md)) with
its operations' `delay` declarations, and a probabilistic claim with its scheduler
clause ([FR-420](../analysis/FR-420-sample-stochastic-timed-models.md)).

## Outputs

An [FR-411](../analysis/FR-411-settle-probabilistic-claims-exactly.md) exact
verdict over the digital process, or over the region process when a
constraint is strict or the claim is a bounded-window formula other than a
deadline event or its `always[0, D]` complement; an `incomplete` limit stop
while a case cannot be decided within the request's limits; or, for a continuous delay distribution only,
`unsupported` with `delay-distribution{operation}`.

## Behavior

### Delays

The provider reads each `delay` declaration from its `delay_distribution`
node of the checked package ([FR-640](../../objects/interfaces/FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)),
and the items it settles are the timed rows of FR-641.

`under every scheduler`, with or without a fairness set, chooses every delay
no distribution gives and every scheduled identity, and random arguments are
drawn by FR-406. A `discrete` delay distribution applies under every
scheduler as it does under a workload: the race draws given delays with their
probabilities, so the subject is a Markov decision process decided by its
minimum and maximum.

`under W` resolves the scheduled identity and, through FR-420's race, every
delay a `discrete` distribution gives: each race outcome at a digital timed
state (each racing identity's delay drawn from its distribution conditioned on
its window, the smallest winning, ties ordered by `W`'s weights) is one
probabilistic edge with an exact rational probability. When every racing
identity's delay is so given, the subject under `W` is a Markov chain over
digital timed states. The delay of an operation with no `delay` declaration stays
the scheduler's choice, and the provider decides the claim over every
resolution of those delays, the bound read on their minimum or maximum. A
`uniform`, `uniform[a, b]` or `exponential` distribution makes the subject a
continuous-time process whose probabilities are in general not rational; it
has no exact reading and settles `unsupported`, `delay-distribution{operation}`,
and such claims are checked with statistical evidence (FR-420). A digital
timed state with no racing identity and a bounded delay is a time-lock, decided
below.

### Closed and strict conditions

When every atomic clock constraint in a guard, every time invariant and every
timed interval of the claim is non-strict, and the claim is unbounded
reachability or until over state predicates, a deadline event or its
`always[0, D]` complement over state predicates (predicates over the
discrete state, with no clock atom), an elapsed form, or a `steps` measure
or an `accumulate` of per-step rewards within a time horizon (a reachability
over the closed automaton: its count or sum is discrete state and its
horizon a closed clock), the provider reads the subject through digital
clocks. This rule does not settle the route of a `weighted by delay` fraction
over a window, a `weighted by delay` long-run fraction or an unweighted
fraction over a time window, which keep the rule they had before it (digital
when every constraint is closed, clock regions otherwise) pending a ruling
(QSL-3324). When one is strict, or the claim is any
other bounded-window formula, it reads the subject through clock regions
instead, closed or not: each clock keeps its integer part, capped as below,
whether its fractional part is zero, and the order of the clocks' fractional
parts. The region process gives the exact minimum and maximum of every
admitted form on the dense semantics, which digital clocks do not once a
constraint is strict, so a strict constraint keeps an exact verdict. Digital
clocks also miss the dense extremum of a bounded-window formula whose truth
turns on a position being absent from a window, even when every constraint
is closed: in a model where `a` happens at a scheduler-chosen time in
`[0, 1]` and `b` exactly 2 later, `always[1, 2] holds(false)` has dense
maximum 1 (`a` strictly between 0 and 1) but digital maximum 0, because
integer time stamps put `a` at 1 or `b` at 2. The equality of digital and
dense extrema below covers reachability and expected time only.

For some admitted forms the number of pending target times a bounded-window
monitor keeps can grow without bound on the region process (an interval
operator nested under another, read at arbitrarily many positions of its
window). While the provider cannot decide such a case within the request's
limits, the run stops `incomplete`, `limit-reached` naming the
`ExactProbLimits` member it reached (FR-411), as any exact run does, and
settles neither `unsupported` nor a verdict it has not decided. That stop is
what the provider may do while it cannot decide a case; it replaces no
criterion, and every exact value a criterion states stands.

### Digital clocks

Every constant of the model, the claim and the `discrete` delays is scaled to an
integer by their common denominator. A digital timed state is the discrete
state with an integer value per clock, capped at the clock's largest compared
constant plus one. A delay move adds one unit to every clock below its cap and
is admissible when the time invariants hold after it and the state is not
urgent; a discrete move is FR-181's timed step with integer clocks. For a
closed, diagonal-free probabilistic timed automaton the minimum and maximum of
a reachability probability and of an expected time to a target are equal on
the digital and the dense semantics, so the verdict holds on the dense model,
and every delay a witness takes is a valid dense delay.

### Admitted forms

Every bounded-window formula FR-407 admits under the timed profile is decided.
Unbounded reachability and until over state predicates; deadline events
`eventually[0, D] holds(P)` and `holds(A) until[0, D] holds(B)` with `D`
closed, by one extra digital clock that no step resets, capped at `D + 1`; and
`expected elapsed until holds(B)`, each delay move carrying one unit of time
as its reward. `always[0, D] holds(P)` is the complement of
`eventually[0, D] holds(not P)`, a minimum read as one minus the other's
maximum. Every other bounded-window formula, a punctual interval and an
interval operator nested under another included, is decided over the product
of the region process with the claim's monitor. The monitor SHALL give the formula's FR-416
pointwise value on every behaviour of the product, and SHALL keep whatever
pending target times that value needs.

Every form above, deadline events included, reads its predicates at
positions only (FR-416 "Pointwise meaning"): the initial state, the
post-state of each discrete step and each terminal stutter position of an
idle tail. A delay move is not a position: it advances time, so a window it
passes closes, but it reads no predicate. The time stamps of terminal stutter
positions after a quiescent state are the scheduler's choice (FR-416 "Time
stamps"), so a minimum or maximum ranges over them as over every other
scheduler choice.

The provider SHALL resolve the probabilistic claim's exact
`{declaration NodeRef, occurrence}` through
`checked_term_data.probabilistic_claims` and report its verdict against that
identity. If two claims have equal display names or source spans, the provider
preserves their distinct declaration identities and reports against the
selected identity. A missing, foreign, wrong-kind or mismatched claim identity
is an FR-322 checked-package refusal before FR-421 evaluation.

### Time divergence

A behaviour of a timed subject lets time diverge, so no scheduler stays
forever in a cycle of discrete moves alone. The provider finds such cycles by
a strongly connected component decomposition of the discrete-move subgraph
and decides the item over the process with every end component that holds no
delay move made transient: its states take the value of their best exit for
the objective. An end component with no exit, and a state from which time
cannot advance and no discrete move is enabled, is a time-lock: the run ends
there with time frozen, so a deadline not yet met is never met, and the
verdict is exact. The time-lock-freedom item
([FR-417](FR-417-settle-time-lock-freedom.md)) reports time-locks as a
property of their own.

### Evidence

Under every scheduler, a witness scheduler chooses between a unit delay and
an identity at each digital state. Under a workload with free delays, the
witness scheduler chooses each free delay, in units, per drawn vector of the
given delays (FR-413). A timed path of a path set carries each step's delay and
replays by FR-181 timed replay; at its last timed state no admissible discrete
step precedes the event's time horizon, which replay checks from the guards
and time invariants. A certificate is an FR-725 timed-product certificate, keyed by digital or
region product states, and the checker rebuilds the digital or region process
itself (FR-751, FR-752, FR-753). The digital process's size grows
with the scaled constants and is bounded only by the request's `max_states`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-421-AC-1 | On the `Retx` vector, `Deadline` at 0.99 over every scheduler settles `proved`, `exact-value{99/100}` at equality, with maximum `9999/10000`. | Test (TC-374) |
| FR-421-AC-2 | `Deadline` at 0.995 settles `refuted` by a one-path set of probability `1/100` with losses at 2 ms and 4 ms, and replay checks that the next `send` needs time past 4 ms. | Test (TC-374) |
| FR-421-AC-3 | `MeanTime` settles `proved`, `exact-value{20/9 ms}` for the maximum expected time, with minimum `10/9 ms`. | Test (TC-374) |
| FR-421-AC-4 | A strict guard is decided over clock regions and settles an exact verdict; a `discrete` delay under every scheduler is applied and the item settles an exact verdict; a zero-delay cycle is decided with its delay-free end component made transient; `always[0 ms, 4 ms] holds(P)` is decided as a complement; a time-locked state ends its run; only a `uniform` or `exponential` delay with exact evidence settles `unsupported`, `delay-distribution{operation}`. Temporary (QSL-3324 fallback, FSL FR-204 F-7, while QSL-3444 is open): the strict-guard item decided over clock regions settles `incomplete`, `timed-route-withheld{reason: region-walk-unchecked}` instead of `proved`, and its `refuted` verdict is kept when every time invariant is a conjunction of clock bounds and no race draws a given delay; the other clauses are unchanged. | Test (TC-374) |
| FR-421-AC-5 | Under a workload whose every racing delay is `discrete`, the timed subject is a Markov chain and the exact value of the all-`discrete` `RareLate` variant is `1/20`. | Test (TC-374) |
| FR-421-AC-6 | Under a workload, an operation with no `delay` declaration has its delay chosen by the scheduler, and the bound compares the minimum or maximum over those choices: on the all-`discrete` `RareLate` variant with `reply`'s delay free, the maximum is `1/2` and the minimum 0, and `probability <= 0.01` settles `refuted` with a witness scheduler that takes `reply`'s delay 3 ms. Temporary (QSL-3324 fallback, FSL FR-204 F-4, owner decision pending): `reply`'s free delay has the window `[1 ms, 3 ms]`, so the FSL provider settles this item `incomplete`, `timed-route-withheld{reason: workload-free-delay}`, while the fallback holds; the values above stay the target. | Test (TC-374) |
| FR-421-AC-7 | For two probabilistic claims with equal display names and source spans but distinct declaration identities, a punctual-interval form (proposed pin, QSL-3324: `eventually[2 ms, 2 ms] holds(m.delivered)` at the origin over TC-374's `Retx`, whose minimum over every scheduler is 0 and maximum `99/100`) and an elapsed form of the selected declaration each settle an exact verdict reported against that declaration's identity, never `unsupported`. The selected declaration and its `claim` occurrence are resolved through FR-322-AC-297; the result retains the selected NodeRef and occurrence. Temporary (QSL-3324 fallback, FSL FR-204 F-1 to F-3 and F-7, owner decision pending): a `proved` verdict of the punctual-interval item settles `incomplete`, `timed-route-withheld{reason: digital-proof-unsound}` on the digital route or `region-walk-unchecked` on the region route; a `refuted` verdict is kept once FSL's single-operator position fix has landed (`position-read-unchecked` before). | Test (TC-374) |

## Dependencies

- [FR-411](../analysis/FR-411-settle-probabilistic-claims-exactly.md) supplies
  the exact methods, verdicts and evidence.
- [FR-415](FR-415-declare-clocks-and-timed-models.md) and
  [FR-420](../analysis/FR-420-sample-stochastic-timed-models.md) supply the
  timed model and delays.
- [FR-416](FR-416-evaluate-timed-temporal-claims.md) supplies the timed
  profile.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  supplies the checked probabilistic claim declaration identity and occurrence
  through `checked_term_data.probabilistic_claims` (FR-322-AC-297).

## References

- FSL ADR-028 TA-1, TA-1a, TA-2 to TA-7, RU-6; FSL ADR-026 SS-7.
- Owner ruling recorded on QSL-371: a workload that gives delays makes the
  timed subject a Markov chain, exact on the exact engine, and leftover delay
  nondeterminism is a minimum or maximum.
- M. Kwiatkowska, G. Norman, D. Parker and J. Sproston, "Performance analysis
  of probabilistic timed automata using digital clocks", 2006.
- Ticket: STD-139 (paired with QSL-373); STD-137 (paired with QSL-371).
- QSL-3324 (draco:QSL:arch ruling of 2026-11-25): FR-416's pointwise meaning
  governs the monitor; the earlier "one deadline clock per interval operator
  occurrence" could not hold it and is withdrawn. Second ruling (route):
  every bounded-window form other than a deadline event (or its complement)
  over state predicates is read through clock regions.
