---
id: FR-420
title: "Interpret and sample stochastic timed models"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
  - target: ix://agent-ix/filament-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-408
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
---

# FR-420: Interpret and sample stochastic timed models

## Description

When a probabilistic claim over a timed subject names a workload, the
provider SHALL draw each declared delay from its distribution, with each undeclared
delay left to the scheduler.

## Inputs

A `time dense` timed subject ([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md))
with its operations' `delay` declarations ([FR-405](FR-405-declare-probabilistic-models.md));
a workload; a probabilistic claim ([FR-407](FR-407-state-probabilistic-claims.md));
for statistical evidence, the request's grid exponent `q`, `max_sample_steps`
and `StatisticalLimits`.

## Outputs

The measure on timed behaviours under the workload, statistical results
([FR-408](FR-408-settle-probabilistic-claims-statistically.md)) whose samples
are timed behaviours, or an `unsupported` result with cause `not-stochastic` or
`time-locked-sample`.

## Behavior

### Delay distributions

A `delay` declaration is carried in the checked package as a
`delay_distribution` node ([FR-640](../../objects/interfaces/FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)),
and the provider reads the family and its parameters from that node.

An operation's `delay M::o ~ D;` declaration gives one of these families, every
parameter an exact rational and every delay a time quantity converted exactly
to the model's unit (FR-142):

- `uniform` — uniform over the identity's window;
- `uniform[a, b]` — uniform over the delays in `[a, b]`, conditioned on the
  window;
- `exponential(λ)` — rate `λ`, a positive exact rational per time unit,
  conditioned on the window;
- `discrete { d1: w1, …, dn: wn }` — finitely many distinct non-negative
  delays with positive exact rational weights, conditioned on the window.

Uniform over a set is uniform in length; over a set of zero length it is
uniform over its finitely many points. The delay of an operation with a
`delay` declaration is **given**; the delay of an operation with no `delay` declaration
is **free**: it is nondeterministic, and no distribution is implied for it.

### Windows

At a timed state `(s, v)`, the **window** of a scheduled identity is the set
of delays `d` for which `(s, v + d)` is reached by an admissible delay and the
identity's data precondition and guard hold there. A window is a finite union
of intervals with exact rational ends, open or closed. An identity with an
empty window, or whose given distribution has zero mass in its window, does
not race.

### The race

Under a workload, at each timed state every racing identity with a given
delay draws its delay from its distribution conditioned on its window. The
scheduler then chooses, after those draws, a delay in the window of each
racing identity with a free delay. The smallest delay wins: the model delays
by it and takes the winning identity with its random arguments drawn
(FR-406). When several identities tie at the smallest delay, the workload
orders them, drawing without replacement by operation weight and then
uniformly within an operation, an operation of weight 0 drawn only when no
tied operation of positive weight remains, and then uniformly among such
operations (FR-406); the model delays once and takes them in that
order at the same instant, each only when it is still enabled at its turn.
Every identity redraws at the new state. With every racing delay given, the
timed subject under the workload is a Markov chain over timed states; free
delays make it a Markov decision process, and a bound compares the minimum
(for `>= θ`) or the maximum (for `<= θ`) over the free delays' choices.

A timed state where an identity with an unbounded window has `uniform`, or
where no identity races and delay is bounded, has no defined race and settles
`unsupported`, `not-stochastic{state, identity}`. A quiescent state where no
identity races takes the idle tail (FR-161).

### Events and measures in time

Under the timed profile, an event is a bounded-window formula decided by the
positions with time stamp up to its time horizon; an `elapsed` measure is a
time-stamp difference (FR-407), and `weighted by delay` weights each position
by the delay of the step that leaves it, clipped to the window as FR-407
states (a leaving step that crosses the horizon `h`, and an idle tail, give
`h − stamp`), so a long-run fraction is the fraction of time a predicate holds.

### Sampling timed runs

Statistical evidence requires every racing identity's delay to be given; a
free delay that races settles the item `unsupported`, `not-stochastic`, at the
first state where it races. A sample repeats the race from the initial state.
Each delay draw and each tie-break is one more choice of FR-408's step, with
its own choice index in the sampler preimage. A `discrete` draw selects by
exact integer weights. A `uniform`, `uniform[a, b]` or `exponential` draw
takes a uniform value `u` on the grid of `2^-q` from the generator and returns
the conditioned inverse distribution function at `u`, computed with FR-408's
rational bounds and rounded up, so every sampled delay is an exact rational.
A sampled delay is the declared distribution quantised to that grid, which
changes each draw's distribution function by at most `2^-q`; the `measured`
result is a measurement of the quantised model, and its basis records `q`. A
sample ends when its next position's time stamp passes the event's or
measure's time horizon, or at an idle tail. A sample that reaches a time-locked
state before its horizon, where no positive delay is admissible and no
transition identity is enabled, settles the item `unsupported`,
`time-locked-sample{state}`, naming that state, and keeps no witness from it:
the measure is over time-divergent behaviours only, and the time-lock-freedom
item ([FR-417](../temporal/FR-417-settle-time-lock-freedom.md)) reports the
time-lock. A sample that takes more steps
than `max_sample_steps` before passing its horizon stops the run with
`incomplete`, `limit-reached` naming `max_sample_steps`, its value and the
request setting that raises it. `q`, `max_sample_steps` and the
`StatisticalLimits` members are request settings with published defaults;
the default of `max_sample_steps` is `2^24`.

### Regeneration and witnesses

The regenerative method cuts a timed run at each return to the initial
discrete state with every clock 0. A sampled witness over a timed subject
carries each step's delay and replays by FR-181 timed replay plus the draw
recomputation of FR-410, which recomputes each delay from the seed, trace
index, step, choice index and `q`.

**The timed regeneration point.** A timed run regenerates at a position whose
**regeneration key** equals the initial one. The key is the pair of the
discrete state key and the clock-state key: the exact rational value of every
clock, with the delay already drawn for each racing identity that is still
pending (FR-196's pending schedule and draw). The initial key is the initial
discrete state with every clock 0 and no pending draw, so a return to the
initial discrete state with any clock above 0, or with a draw still pending,
is not a regeneration point and does not cut the run. After a regeneration
point the future depends only on the key, because each racing identity's
delay is drawn afresh when it becomes enabled, so the cycles between
successive regeneration points are independent and identically distributed.
A clock that no step resets keeps the key from recurring, and a run that
reaches no regeneration point within `max_cycle_steps` steps stops with
`incomplete`, `limit-reached` naming `max_cycle_steps` (FR-408).

**Delay-weighted cycles.** For `long_run_fraction holds(P) weighted by delay`
(FR-407), cycle `k` contributes `Y_k`, the sum of the delays, clipped to the
cycle, of the positions where `P` holds, and `L_k`, the cycle's total delay,
each an exact rational from the sampled delays, so `Y_k / L_k` is the fraction
of the cycle's time spent in `P`. The estimate and its interval are those of
FR-408's regenerative method with these `Y_k` and `L_k`. A behaviour that reaches
a terminal state ends in an idle tail whose weight is unbounded, so its
long-run fraction is the limit `1` when `P` holds at the terminal position and
`0` otherwise, never a finite `Y_k / L_k`. When any sampled behaviour
terminates, the sampler draws independent behaviours from `s0`, each giving one
value `v_j` in `[0, 1]`: the terminal limit for a behaviour that terminates,
and its own ratio `Σ Y_k / Σ L_k` for a behaviour that completes at least
`min_cycles` cycles without terminating. The estimate is the mean of the
`v_j` and its interval is FR-408's exact rational Hoeffding bound for a mean
of values in `[0, 1]`, not the Chow–Robbins half-width; only when no sampled
behaviour terminates does FR-408's single-behaviour regenerative estimate
apply unchanged. A cycle of total delay 0 (a run of zero-delay steps
that returns to the initial key) contributes `Y_k = L_k = 0` and does not
enter the denominator; a run whose every cycle has total delay 0 takes the
unweighted fraction `Σ [P] / Σ 1` over positions, the same limit FR-411
uses for a zero-weight component. The witness of a sampled run lists each
cycle's regeneration positions and delays and replays by FR-181 timed replay
and FR-410's draw recomputation.

### Every-behaviour claims

Non-probabilistic claims over a stochastic timed model read it as a timed
model and ignore distributions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-420-AC-1 | On the `RareLate` vector, `reply` with `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };` ties with `timeout` at 3 ms with probability `1/10`, the workload orders the tie evenly, and the event's exact probability is `1/20`. | Test (TC-374) |
| FR-420-AC-2 | Every sampled delay is an exact rational from the `2^-q` grid's conditioned inverse distribution function, `q` is recorded in the basis, and each sample replays with its delays. | Test (TC-374) |
| FR-420-AC-3 | A racing identity of an operation with no `delay` declaration settles a statistical item `unsupported`, `not-stochastic`; under exact evidence its delay is a scheduler choice and the bound compares the minimum or maximum. | Test (TC-374) |
| FR-420-AC-4 | An identity with an unbounded window and `uniform`, or a state with bounded delay and no racing identity, settles `unsupported`, `not-stochastic{state, identity}`. | Test (TC-374) |
| FR-420-AC-5 | A sample exceeding `max_sample_steps` stops the run with `incomplete`, `limit-reached` naming `max_sample_steps`, its value and how to raise it; an omitted `max_sample_steps` is `2^24`. | Test (TC-374) |
| FR-420-AC-6 | With `reply` under `delay Call::reply ~ uniform;` the event's probability is 0, the statistical item settles `measured: accepted`, and the every-behaviour claim `NoLateReply` settles `refuted`. | Test (TC-374) |
| FR-420-AC-7 | A sample that reaches a state with no admissible positive delay and no enabled transition identity before its horizon settles the item `unsupported`, `time-locked-sample{state}` naming that state, with no witness kept. | Test (TC-374) |
| FR-420-AC-8 | Under a workload with weight 0 on `Call::retry` and weight 1 on `Call::reply`, when both tie at the smallest delay `reply` is taken first and `retry` second; when two weight-0 operations tie with no positive-weight operation, each order has probability `1/2`. The model's `delay` declarations are read from its `delay_distribution` nodes. | Test (TC-1561) |
| FR-420-AC-9 | A timed model with two states `Up` and `Down`, `Up` lasting `delay ~ discrete { 1 ms: 1 }` and `Down` `discrete { 3 ms: 1 }`, each step resetting the one clock, has `long_run_fraction holds(phase = Up) weighted by delay` equal to `1/4`: every cycle has `Y_k = 1` and `L_k = 4`, the estimate is `1/4` with a zero-variance cycle sample and a positive half-width, `>= 1/5` settles `measured: accepted` and `>= 1/3` `measured: rejected`, and each sampled run replays with its delays; the unweighted fraction of the same model is `1/2`. | Test (TC-374) |
| FR-420-AC-10 | A return to the initial discrete state with a clock above 0 is not a regeneration point. Over `Loop` (state `A` initial with `x = 0`; `A` to `B` after `delay ~ discrete { 1 ms: 1 }`, no reset; `B` to `A` with weight 1 after 1 ms with no reset, or to `Z` with weight 1, `Z` to `A` after 1 ms resetting `x`), the return `B` to `A` has `x = 2 ms` and cuts no cycle, and every cut is a return `Z` to `A` where `x = 0` and no draw is pending; a variant of `Loop` in which no step resets `x` never reaches the initial key and settles `incomplete`, `limit-reached` naming `max_cycle_steps`, its value and the request setting. | Test (TC-374) |
| FR-420-AC-11 | A model whose initial state moves with weight 1 to a terminal state where `P` holds and with weight 1 to a terminal state where `P` does not settles a delay-weighted long-run fraction `>= 1/4` with estimate the mean of the run limits `1` and `0` (value `1/2`), an interval from the Hoeffding bound, and `measured: accepted`; the same model's `>= 3/4` settles `measured: rejected`. A cycle of total delay 0 contributes nothing to the denominator, and a run whose every cycle has total delay 0 takes the unweighted fraction; a time-locked state settles `unsupported`, `time-locked-sample{state}` as in FR-420-AC-7. | Test (TC-374) |

## Dependencies

- [FR-405](FR-405-declare-probabilistic-models.md) declares `delay` declarations;
  [FR-406](FR-406-interpret-probabilistic-models.md) supplies the scheduler
  resolution; [FR-408](FR-408-settle-probabilistic-claims-statistically.md)
  supplies the statistical methods.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) supply timed
  moves and replay.

## References

- FSL ADR-026 SD-1 to SD-5, SS-1 to SS-6, OV-8; FSL ADR-028 RU-6, TA-1a.
- Owner rulings: stochastic delays with statistical checking (recorded on
  QSL-373); a workload that gives delays resolves delay choice and leftover
  delay nondeterminism is a minimum or maximum (recorded on QSL-371).
- A. David et al., "Statistical model checking for networks of priced timed
  automata", FORMATS 2011.
- Ticket: STD-139 (paired with QSL-373); STD-137 (paired with QSL-371).
