---
id: FR-416
title: "Evaluate timed temporal claims under the timed profile"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-252
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-255
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-418
    type: references
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-541
    type: references
---

# FR-416: Evaluate timed temporal claims under the timed profile

## Description

When a temporal claim selects the `filament.temporal.timed/v1` profile, the
evaluator and every provider SHALL read it pointwise over the time-stamped
positions of the subject's time-divergent behaviours, with each interval
measuring time-stamp differences, and SHALL settle it only in one of the four
timed forms below.

## Inputs

A temporal clause under `filament.temporal.timed/v1` (FR-090) with a
`model-time` clock binding (FR-252) over a timed subject
([FR-415](FR-415-declare-clocks-and-timed-models.md)), or over a timed trace;
its timed intervals (FR-255).

## Outputs

A verdict over every admitted behaviour of the timed subject, a pointwise
value over one timed trace, or a typed refusal or `unsupported` cause.

## Behavior

### Time stamps

Each position of a timed behaviour carries its **time stamp**, a
non-negative exact rational in the binding's unit, non-decreasing along the
behaviour (FR-181): under `time dense` the sum of the delays before it; under
`time tick O period p` the period times the number of `O` steps up to it.
Positions with equal time stamps keep their step order, which is the admitted
order FR-090 requires of order-sensitive operators.

A behaviour whose discrete steps end at a quiescent state continues with
FR-161's terminal stutter positions, each stamped after a positive delay of
any length (one tick period under a tick source), every such choice an
admitted behaviour; a behaviour that ends at a state that is not quiescent is
not admitted.

### Pointwise meaning

This reading applies to the `filament.temporal.timed/v1` profile; the `filament.temporal.timestamped-event.finite-window/v1` profile follows the timestamped rule of [FR-091](FR-091-evaluate-bounded-future.md) and [FR-092](FR-092-evaluate-bounded-past.md); the integer-position profiles, infinite-trace, event-position and fixed-sample, read an interval as position offsets with the meaning of FR-091, FR-092 and [FR-367](FR-367-evaluate-interval-operators-under-infinite-trace.md).

`eventually_I p` holds at position `i` when some `j >= i` has `τ_j − τ_i` in
`I` and `p` at `j`; `p until_I q` holds when some such `j` has `q` and every
`k` with `i <= k < j` has `p`; `once`, `historically`, `since` and
`triggered` read `τ_i − τ_j` over `j <= i`; `always` and `release` are the
duals. Atoms read the discrete state and atomic clock constraints at the
position's clock valuation. A data predicate is constant between positions,
so a timed invariant over data reads every instant; an atomic clock constraint
in a claim is read at positions, and a bound that must hold at every instant
is a time invariant of the model. A timed interval measures time stamps, so
inserting a step that repeats the discrete state at an equal time stamp
leaves every timed formula's value unchanged.

### Timed forms over a timed subject

A claim over a timed subject has one of four forms:

- **TT-1 timed invariant**: `always holds(I)`;
- **TT-2 bounded window**: activation `on origin` and only interval operators;
  its truth depends on the positions with time stamp at most its time
  horizon, the sum of its upper bounds, which time divergence makes finitely
  many on each behaviour;
- **TT-3 timed safety**: a formula in the safety fragment with timed
  intervals;
- **TT-4 timed liveness**: any other admitted formula, with its fairness.

TT-2 admits punctual intervals (`[a, a]`). TT-1, TT-3 and TT-4 admit every
formula with no punctual interval; such a claim with a punctual interval
settles `unsupported`, `punctual-interval{locus}`, over a timed subject. Over a
single timed trace every admitted formula evaluates, punctual intervals
included. [FR-541](FR-541-evaluate-timed-claims-over-a-recorded-timed-trace.md)
states how a claim settles over one recorded timed trace, a finite prefix or a
timed lasso.

### Verdicts

A timed item settles with the verdict kinds of FR-161 and FR-360. The length
of a timed counterexample is its number of discrete steps. A proof carries
the proof basis `zone-certified`, with an [FR-418](../../objects/interfaces/FR-418-zone-certificate.md)
zone certificate the checker accepts before the item settles, or `exhaustive`
when every clock constraint, time invariant and timed interval is closed and
the provider decides the subject over integer-valued clocks with constants
scaled to integers (digitization), which gives the same verdict for TT-1 over
the discrete state, deadlock freedom and location reachability.

Every timed refutation carries a counterexample with exact rational delays
that replays by FR-181 timed replay. A symbolic provider concretizes a
symbolic path canonically: at each step the least admissible delay when the
delay set has a least element, otherwise the midpoint of a bounded delay set,
otherwise its lower end plus one unit. A symbolic lasso is concretized by
solving its loop's linear constraints over rational delays; when no accepting
cycle concretizes, the item settles `inconclusive`, `lasso-not-concretized`.

### Non-Zeno premise

A timed claim is read over time-divergent behaviours only (FR-161-AC-13), so
a liveness verdict can rest on excluding Zeno cycles. Every result of an item
that a provider decides on a liveness product, with time divergence as an
acceptance condition (a TT-3 or TT-4 claim, or an infinite-trace liveness
claim over a timed subject, on the zone route or the digital-clock route),
SHALL carry the result-entry member `non_zeno`, which states what the verdict
owes to that premise:

| `non_zeno` | Result | Meaning |
| --- | --- | --- |
| `not-required` | `proved` | The product has no accepting cycle even without the divergence acceptance condition: the claim holds on every behaviour, Zeno ones included. |
| `assumed` | `proved` | The product has accepting cycles without the divergence acceptance condition and none with it: every behaviour that would violate the claim is Zeno, and the proof holds only because Zeno behaviours are not admitted. |
| `checked` | `refuted` | The counterexample's accepting loop satisfies the divergence acceptance condition, and its timed replay (FR-181) reproduces its delays, so the refutation is over an admitted behaviour. |
| `refuted` | `inconclusive`, `no-admitted-behaviour` | No time-divergent behaviour exists from the initial state, so the premise fails and no verdict is given. |

On the zone route the divergence acceptance condition is the one the
non-Zeno emptiness construction adds to the product: a cycle passes it only
when time diverges along it. A cycle of positive but bounded total delay,
such as a self-loop that resets no clock under an upper-bound time invariant,
does not pass it. On the digital-clock route, where every delay is an integer
number of ticks, a cycle passes it exactly when its total delay is positive.
Any other result of such an item (`incomplete`, `unsupported`, `cancelled`, a
rejected certificate, a replay that refuses, and a `refuted` result with
cause `undefined-evaluation`, whose finite prefix decides no liveness cycle,
FR-416-AC-8) and every result of a TT-1 claim, a TT-2 claim, a
time-lock-freedom item or a deadlock-freedom item carries no `non_zeno`. A
zone-certified proof states its value in the certificate's `non_zeno` member
(FR-418), and the certificate checker SHALL recompute it from the
certificate's nodes, edges and acceptance members and reject a certificate
whose stated value differs, so the member is covered by the certification
like the verdict. The member lets a
reader tell a proof that holds outright from one that holds only under the
timed semantics' exclusion of Zeno runs, which a downstream consumer whose
own semantics admit Zeno runs must not reuse as its own proof.

### Strong fairness over time

A timed behaviour SHALL satisfy a strong fairness constraint when, if the
constraint is enabled at arbitrarily late instants (its positions and the
instants of the delays between them), one of its transitions is taken
infinitely often. On a timed lasso the constraint is taken somewhere in the
loop or enabled at no instant of the loop. A constraint enabled only inside
delays is therefore enabled for strong fairness although it is enabled at no
position. The symbolic provider SHALL split each symbolic state by the guards
of the claim's strong constraints as it splits by weak ones, so enabledness at
an instant is exact on the split graph, and SHALL apply the strong fairness
filter to the components of that graph, its certificate stating the nested
numbering of [FR-418](../../objects/interfaces/FR-418-zone-certificate.md).

### Undefined evaluation

An undefined evaluation refutes by the FR-363 rule for every model engine.
When the claim evaluates undefined at a position of an admitted timed
behaviour, every provider SHALL settle the item `refuted` with cause
`undefined-evaluation`, whose `where` names the position, its time stamp, its
discrete state key, its clock valuation and the locus of the expression that
has no value, and whose `cause` names the undefined value's catalogued
reason. Its counterexample is a finite prefix ending at that position, with
exact rational delays concretized canonically as above, and FR-181 timed
replay reproduces the undefined value there.

### Clock binding

A claim that writes no `clock` member binds the default of its selected
profile (FR-252, FR-765): `model-time` under `filament.temporal.timed/v1` and
`model-steps` under every other profile, whether or not the model is timed. A `model-time` binding selects the timed profile. A `model-time`
binding over an untimed model refuses with
`invalid_timed_model`/`model-time-on-untimed-model`. Writing the default
binding changes nothing: the binding, not its spelling, enters the obligation
identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-416-AC-1 | On the TC-369 `Rpc` vector with `T = 3 ms`, `NoLateReply` settles `refuted` with the three-step counterexample of delays 0, 3, 0 and `trace_position` 3; with `T = 4 ms` it settles `proved`. | Test (TC-369) |
| FR-416-AC-2 | `Settles` with `eventually[0 ms, 3 ms]` settles `proved` for both `T`; with `eventually[0 ms, 3 ms)` it settles `refuted` with the reply at exactly 3 ms. | Test (TC-369) |
| FR-416-AC-3 | A TT-1, TT-3 or TT-4 claim with a punctual interval settles `unsupported`, `punctual-interval`; a TT-2 claim with one is decided, and a single timed trace evaluates it. | Test (TC-369) |
| FR-416-AC-4 | Equal time stamps keep step order; inserting a stutter step at an equal time stamp leaves every timed formula's value unchanged. | Test (TC-369) |
| FR-416-AC-5 | A proof settles only with an accepted zone certificate (`zone-certified`) or by digitization of an all-closed subject (`exhaustive`). | Test (TC-369, TC-371) |
| FR-416-AC-6 | A claim with no `clock` member binds `model-time` under the timed profile and `model-steps` under the event-position false-extension profile, over the same timed model, and under the timed profile over an untimed model refuses `invalid_timed_model`/`model-time-on-untimed-model`; `clock "model-steps"` binds steps; `model-time` over an untimed model refuses `invalid_timed_model`/`model-time-on-untimed-model`; writing the default leaves the obligation identity unchanged. | Test (TC-368) |
| FR-416-AC-7 | Every timed counterexample has exact rational delays chosen canonically and replays with no provider present; an unconcretizable lasso settles `inconclusive`, `lasso-not-concretized`. | Test (TC-369) |
| FR-416-AC-8 | A timed claim that evaluates undefined at a position of an admitted behaviour settles `refuted`, cause `undefined-evaluation` naming the position, time stamp, discrete state key, clock valuation, expression locus and catalogued reason, with a finite prefix of exact rational delays ending there that replays to the same undefined value. | Test (TC-369) |
| FR-416-AC-9 | A `Blink` model (`time dense`; object `b` with `fired: Bool` and clock `x`; `tick` with guard `x >= 1` and reset `x`; `fire` with guard `x > 0 and x < 1` and postcondition `self.fired = true`; time invariant `when true { x <= 1 }`; initial `fired = false`, `x = 0`), in which `tick` fires exactly at `x = 1`, and `eventually holds(b.fired)` under infinite-trace: under `fair { weak Blink::fire }` it settles `refuted` with a lasso of `tick` steps of loop delay 1; under `fair { strong Blink::fire }` it settles `proved` with an accepted zone certificate, because `fire` is enabled at instants inside every delay of that lasso, though at no position of it. | Test (TC-963) |
| FR-416-AC-10 | On the zone route over `Rpc` with `T = 4 ms`, the response claim proves with `non_zeno` `not-required`; the variant in which `reply` loops on itself with no reset, under the deadline `x <= 3 ms`, proves with `non_zeno` `assumed`; the variant with an `echo` step that resets `x` refutes with `non_zeno` `checked`; `always eventually holds(true)` over `Stall` settles `inconclusive`, `no-admitted-behaviour`, with `non_zeno` `refuted`; `NoLateReply` (TT-1), the time-lock-freedom item and FR-416-AC-8's `undefined-evaluation` refutation carry no `non_zeno`; the reply-loop proof's FR-418 certificate states `non_zeno` `assumed`, and the same certificate with that member changed to `not-required` is rejected; and the closed MITL claim `Within` proved on the digital-clock route carries the same `non_zeno` as on the zone route. | Test (TC-2041) |
| FR-416-AC-11 | Over a dense timed model whose initial state is quiescent and enables no operation, the admitted behaviours are the idle tails whose stutter positions follow positive delays, one behaviour per delay choice, so `always[2 ms, 3 ms] P` with `P` false there is refuted by the idle tail with a stutter position at 2 ms; under `time tick O period 1 ms` each stutter position adds 1 ms; a model whose only dead end is urgent, or under an applying time invariant, has no admitted behaviour that ends there. | Test (TC-369) |

## Dependencies

- [FR-415](FR-415-declare-clocks-and-timed-models.md) declares the timed model.
- [FR-090](FR-090-select-temporal-profile-and-clock.md),
  [FR-252](../../objects/temporal/FR-252-temporal-clock-binding.md) and
  [FR-255](../../objects/temporal/FR-255-bounded-temporal-interval.md) select
  the profile, binding and intervals.
- [FR-161](FR-161-analyze-infinite-trace-properties.md) and
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) supply timed
  behaviours, verdicts, lassos and replay.
- [FR-418](../../objects/interfaces/FR-418-zone-certificate.md) supplies the
  proof certificate.

## References

- FSL ADR-026 DF-1 to DF-7, TV-1, CB-1, CB-5, EZ-8 to EZ-10, CF-4, §15; FSL
  FR-430 (strong fairness over time).
- R. Alur, T. Feder and T. A. Henzinger, 1996; P. Bouyer, N. Markey,
  J. Ouaknine and J. Worrell, 2007; J. Ouaknine and J. Worrell, 2005.
- Ticket: STD-139 (paired with QSL-373).
