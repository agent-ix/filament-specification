---
id: FR-421
title: "Check probabilistic timed claims exactly"
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
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
---

# FR-421: Check probabilistic timed claims exactly

## Description

When a `probabilistic-satisfaction` item over a timed subject names `exact`
evidence, the exact provider SHALL read the subject through the route "Closed
and strict conditions" admits for its claim: digital clocks, as a
probabilistic timed automaton, or clock regions.

When the exact provider settles such an item, it SHALL settle only a verdict
that holds on the timed runs of FR-416.

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
verdict over the digital process or the region process, as "Closed and strict
conditions" admits; an `incomplete` stop while the admitted route cannot decide
a case; or, for a continuous delay distribution only, `unsupported` with
`delay-distribution{operation}`.

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

This section is the one rule for which route decides a claim. The provider
SHALL read a claim through digital clocks only where this section admits it;
every other claim is read through clock regions. The theorem that justifies
each admitted row is recorded with the route admission table of FSL ADR-038.

**The subject.** Digital clocks require every atomic clock constraint in a
guard, every time invariant and every timed interval of the claim to be
non-strict, no constraint that compares two clocks, and a subject in one of
three delay classes:

- **given**: every delay is given by a `discrete` distribution;
- **free**: no delay is given, and under a workload no two identities race at
  one state;
- **mixed**: both kinds occur, under a workload at most one identity with a
  free delay races at a state, and at every state where a delay is drawn every
  clock that bounds a drawn identity's window holds a whole number of units on
  every run.

**The claim.** Over such a subject, digital clocks decide only:

| Claim form | Delay classes |
| --- | --- |
| Reachability and until over state predicates (predicates over the discrete state, with no clock atom), `expected accumulate ... until`, and a step-bounded or long-run measure that reads only the sequence of discrete steps | all |
| A deadline event `eventually[0, D] holds(P)` or `holds(A) until[0, D] holds(B)`, its `always[0, D]` complement, and a `steps` or `accumulate` measure within a time horizon, each read `on origin` | all |
| `expected elapsed until holds(B)` | all |
| `M <= c` of an elapsed measure whose first position is position 0 on every run | all |
| The three rows above read from a later position (`on each`, or an elapsed measure whose first position can follow position 0) | given only |

**Time-locks.** In the free and mixed classes, every quantity other than the
maximum probability of a reachability, `until` or deadline event (equally, the
minimum of its `always` complement) further requires that no time-lock
(FR-416) is reachable in the subject. That property is established over the
subject's dense runs; a digital process with no time-lock does not establish
it.

Every other claim is read through clock regions, closed or not: a strict
constraint or interval; any other bounded-window formula; an unweighted or
`weighted by delay` fraction over a time window; a `weighted by delay`
long-run fraction; an `elapsed` form this table does not name; and any form
over a subject outside the three classes. Each clock then keeps its integer
part, capped as below, whether its fractional part is zero, and the order of
the clocks' fractional parts, and time advances by symbolic time successor.
The region process gives the exact minimum and maximum on the dense runs;
walking a finer grid of clock values is not a region process.

Digital clocks miss dense values outside these rows. In a model where `a`
happens at a scheduler-chosen time in `[0, 1]` and `b` exactly 2 later,
`always[1, 2] holds(false)` has dense maximum 1 (`a` strictly between 0 and 1)
and digital maximum 0, because integer times put `a` at 1 or `b` at 2.

While the provider has not built the region process that an admitted claim
needs, or cannot decide it within the request's limits (the pending target
times of a nested monitor can grow without bound), the run stops `incomplete`
(FR-411), and settles neither `unsupported` nor a verdict it has not decided,
and never a verdict read from digital clocks. That stop replaces no criterion:
every exact value a criterion states stands.

### Digital clocks

Every constant of the model, the claim and the `discrete` delays is scaled to an
integer by their common denominator. A digital timed state is the discrete
state with an integer value per clock, capped at the clock's largest compared
constant plus one. A delay move adds one unit to every clock below its cap and
is admissible when the time invariants hold after it and the state is not
urgent; a discrete move is FR-181's timed step with integer clocks.

Take a closed, diagonal-free probabilistic timed automaton and a scheduler
under which time diverges with probability 1. Rounding every time stamp of a
run with one fixed threshold gives an integer-time run with the same discrete
steps, and for each threshold some integer-time scheduler gives each rounded
run the probability that the dense scheduler gives the runs that round to it.
So the minimum and maximum of a reachability probability and of an expected
time to a target are equal on the digital and dense runs, and every delay a
witness takes is a valid dense delay. This says nothing about a value that
turns on the order or absence of time stamps measured from a position a
scheduler or a draw can place off the integer grid, which is why "Closed and
strict conditions" admits no such value outside the given class.

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

Every form above, deadline events included, reads its predicates at the
positions of the timed run only (FR-416 "The timed run"). The stamps of
terminal stutter positions after a quiescent state are the scheduler's choice
(FR-416 "Idle tail"), so a minimum or maximum ranges over them as over every
other scheduler choice.

The provider SHALL resolve the probabilistic claim's exact
`{declaration NodeRef, occurrence}` through
`checked_term_data.probabilistic_claims` and report its verdict against that
identity. If two claims have equal display names or source spans, the provider
preserves their distinct declaration identities and reports against the
selected identity. A missing, foreign, wrong-kind or mismatched claim identity
is an FR-322 checked-package refusal before FR-421 evaluation.

### Time divergence

A probabilistic item admits schedulers under which time diverges with
probability 1 (FR-416 "Time divergence"), and reads a run that ends in a
time-lock as FR-416 "How claims read a time-lock" states. The provider finds
zero-time cycles by an end-component decomposition and decides the item over
the process with every end component that holds no delay move made transient:
its states take the value of their best exit for the objective. An end
component with no exit, and a state from which time cannot advance and no
discrete move is enabled, is a time-lock. On the region process the
progress test is the one
[FR-753](FR-753-certify-time-divergent-and-elapsed-quantile-verdicts.md)
states. The time-lock-freedom item
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
| FR-421-AC-4 | A strict guard is decided over clock regions and settles an exact verdict; a `discrete` delay under every scheduler is applied and the item settles an exact verdict; a zero-delay cycle is decided with its delay-free end component made transient; `always[0 ms, 4 ms] holds(P)` is decided as a complement; a time-locked state ends its run; only a `uniform` or `exponential` delay with exact evidence settles `unsupported`, `delay-distribution{operation}`. | Test (TC-374) |
| FR-421-AC-5 | Under a workload whose every racing delay is `discrete`, the timed subject is a Markov chain and the exact value of the all-`discrete` `RareLate` variant is `1/20`. | Test (TC-374) |
| FR-421-AC-6 | Under a workload, an operation with no `delay` declaration has its delay chosen by the scheduler, and the bound compares the minimum or maximum over those choices: on the all-`discrete` `RareLate` variant with `reply`'s delay free, the maximum is `1/2` and the minimum 0, and `probability <= 0.01` settles `refuted` with a witness scheduler that takes `reply`'s delay 3 ms. | Test (TC-374) |
| FR-421-AC-7 | For two probabilistic claims with equal display names and source spans but distinct declaration identities, a punctual-interval form and an elapsed form of the selected declaration each settle an exact verdict reported against that declaration's identity, never `unsupported`. The selected declaration and its `claim` occurrence are resolved through FR-322-AC-297; the result retains the selected NodeRef and occurrence. | Test (TC-374) |
| FR-421-AC-8 | Over the `Split` model (TC-374), closed and diagonal-free, whose two branches can each finish only for one end of the first delay's window and time-lock otherwise, `probability >= 0.25 [eventually[0 ms, 3 ms] holds(c.phase = Done)]` under every scheduler never settles `proved`, the minimum being 0: it settles `refuted` with `exact-value{0}`, or `incomplete` while the region process is not built. | Test (TC-374) |
| FR-421-AC-9 | Over the `Burst` model (TC-374), `probabilistic` claim `mean of (fraction holds(phase = 1) over [0, 1 ms]) <= 0.7` never settles `proved`: stutter stamps inside one unit make the fraction `(1 + m)/(2 + m)` for every `m >= 0`, with supremum 1. | Test (TC-374) |

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

- FSL ADR-038 (route admission table); FSL ADR-028 TA-1, TA-1a, TA-2 to TA-7,
  RU-6; FSL ADR-026 SS-7.
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
