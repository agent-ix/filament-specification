---
id: FR-410
title: "Statistical verdict and sampled witness"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-408
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-409
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
---

# FR-410: Statistical verdict and sampled witness

## Description

When a statistical run completes its method, the provider SHALL record a
`StatisticalVerdict` with exactly the members below in the FR-331 result of a
`measured` item, and SHALL record each sampled witness so that it replays with
no provider present.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| decision | MeasuredDecision | 1..1 |  |
| bound | ProbabilisticBound | 1..1 |  |
| estimate | JsonObject | 1..1 |  |
| interval | JsonObject | 0..1 |  |
| basis | JsonObject | 1..1 |  |
| provenance | JsonObject | 1..1 |  |
| tests | JsonObject | 0..* |  |
| witnesses | JsonObject | 0..* |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| estimate | JSON object `{value, quantile?}`; `proposals/backend-provider-v1/schema.json` `Estimate` |
| bound | the checked claim's exact FR-413 bound object, including `level` for a quantile; `proposals/backend-provider-v1/schema.json` `ProbabilisticBound` |
| interval | JSON object `{lower, upper}`; absent only for the sequential method (Absence); `proposals/backend-provider-v1/schema.json` `Interval` |
| basis | `StatisticalBasis{method, confidence, samples, draws, tests, coverage, grid}` object |
| provenance | object: seed, sampler identity, per-test entries |
| tests | one object per test, in test-index order |
| witnesses | up to `max_witnesses` sampled witness objects |


The value has two parts: the `StatisticalVerdict` record and the sampled
witnesses it holds, each with its replay rule below.

### `StatisticalVerdict`

| Member | Content |
| --- | --- |
| `decision` | The [FR-409](../protocol/FR-409-measured-decision.md) value: `accepted`, `rejected` or `undecided`, with the undecided cause `indifference-region` or `undecided-successor`, or the rejection cause `undefined-evaluation` with its `where` and `cause`. |
| `bound` | The exact FR-413 claim-bound object judged for this item. Its closed `type` is `probability`, `quantile`, `mean-of-fraction` or `long-run-fraction`; its `comparison` preserves the checked claim's operator (`greater-or-equal` or `less-or-equal`); `threshold` is the exact FR-413 rational, and a `quantile` bound also carries its independent exact `level`. It is present for every accepted, rejected and undecided verdict, including a rejection by `undefined-evaluation`. |
| `estimate` | An exact rational: `p̂` for a probability bound; for a quantile claim, the conditional probability its decision reads, `Pr(M <= c | activated)` for `quantile q of M <= c` and `Pr(M < c | activated)` for `quantile q of M >= c` (FR-407), and the empirical nearest-rank quantile of the counted samples (the `⌈q · n⌉`-th smallest of `n`); the sample mean for a mean of a fraction; the regenerative ratio for a long-run fraction. A quantity estimate carries the threshold's unit. |
| `interval` | The confidence interval `[lower, upper]` in exact rationals for the fixed-sample, exact binomial and regenerative methods (for the exact binomial method, the outward Clopper–Pearson interval of FR-408); absent for the sequential method. |
| `basis` | `StatisticalBasis{method, confidence, samples, draws, tests, coverage, grid}`: `method` is `okamoto`, `sprt`, `clopper-pearson` or `regenerative` with its `min_cycles`; `confidence` is the claim's `α`, `β` and `ι`; `samples` is the count of counted samples; `draws` the count of drawn samples; `tests` is `m` with the per-test `α'` and `β'`; `coverage` is `finite-sample` or `asymptotic`; `grid` is the delay-draw grid exponent `q` of a timed run ([FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)) and absent otherwise. |
| `provenance` | The seed; the sampler identity `filament.simulation.sampler/v1`; and per test its index, initial state, `over` binding, `on each` instance (null for an `on origin` claim) and trace-index range. |
| `tests` | One entry per test, in test-index order, with its own decision, estimate, interval and counts. |
| `witnesses` | Up to the request's `max_witnesses` sampled witnesses, the first in trace-index order. |

The empirical quantile of a quantile claim is a `censored` form,
`{"type": "censored", "horizon": h}`, when the nearest-rank sample is a
measure censored at the horizon `h`, as a witness value is.

Every count member (`samples`, `draws`, the test `index` and `m`, a
witness's `test`) is a JSON integer, never a string.

A statistical run whose test counted no sample, or a regenerative run that
completed no positive-weight cycle, has no estimate: it does not produce a
`measured` verdict and settles `incomplete` with the FR-331 cause
`{"type": "no-estimate", "reason": "no-counted-sample"}` or
`{"type": "no-estimate", "reason": "no-positive-weight-cycle"}`, which names
no limit because nothing was exhausted.

`estimate` is required, in the verdict and in each test entry, except in
two cases where it is absent: the decision is a rejection by an undefined
sample (FR-409, cause `undefined-evaluation`), or it is `undecided` with
cause `undecided-successor`. In both the decision does not read an
estimate, so none is written. A present `estimate` always carries `value`.

### Sampled witness

A sampled witness is a counted sample on which the event is false (for a
`>= θ` bound) or true (for a `<= θ` bound), as in FR-413's path set, or whose
measure falls on the violating side of the threshold, or a drawn sample on
which the claim evaluates undefined (FR-408). It carries:

- the test index and the trace index;
- the steps, each as the FR-181 trace step content (transition identity,
  post-state key) plus its drawn scheduler choice, its drawn random
  argument values and, over a timed subject, its delay. The drawn scheduler
  choice is the step's `scheduled` member, the scheduled identity of
  FR-405: the step's transition identity carrying only its non-random
  arguments (operation, receiver, non-random argument vector), in the
  `TransitionIdentity` shape. The random arguments are not in it; they are
  the step's `random` member. A step that continues a window past a terminal
  state on its stutter steps (FR-407) is a stutter step: its `transition`
  and its `scheduled` are both the `stutter` arm `{"type": "stutter"}`,
  which names no operation and no arguments, and its `random` is empty;
- each step's `draws`: every draw the step consumed, in consumption order
  (the sampler definition's "Choices within a step"). A delay draw is
  `{"choice": c, "delay": {"identity": i, "delay": d}}`: `i` the racing
  scheduled identity in the `TransitionIdentity` shape, `d` its drawn delay
  as an exact `Rational` in the model's unit, and `c` its choice index, or
  `null` for a delay that drew no draw value (a one-item choice, such as a
  window of one point with all its mass). Any other choice that drew is
  `{"choice": c, "item": k}`, `k` the 0-based index of the selected item in
  that choice's canonical order. Over a timed subject `draws` holds one
  delay draw for every identity that raced at the step, the losers included,
  so the step's `delay` is the least of them. A one-item choice other than
  a delay draws nothing and is not listed. A stutter step's `draws` is
  empty, a schema rule;
- each step's `rewards`: one `{"name": r, "value": v}` per reward the
  step's operation declares ([FR-405](../../functional/analysis/FR-405-declare-probabilistic-models.md)),
  in the bytewise order of `r`, each name once, `v` the reward's
  `TypedValue` on that step. A stutter step's `rewards` is empty, a schema
  rule; the one-entry-per-declared-reward, order and no-duplicate rules are
  checked at replay, not by the schema ("Witness replay");
- the run end `end`, present exactly when the verdict's `basis` has a
  `grid` (a timed run) and absent otherwise. A `time tick` subject is a timed
  run: its basis records the request's `grid` although it draws no delay
  ([FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)
  "Tick subjects"), and its witnesses carry `end`. The run ends at the first
  of these, and `end` is exactly one of:
  - `{"type": "crossing", "step": s}`: the run's window has started and the
    next step drawn from the last position, a race winner or a tick step,
    would be stamped strictly after the window's end. `s` is that step,
    written as a witness step with its draws and rewards; it adds no
    position. It is written for every claim form, whether or not a
    `weighted by R` measure reads its reward (FR-420 "Sampling timed
    runs");
  - `{"type": "time-lock", "state": k, "stamp": t}`: no step is drawn from
    the last position and its timed state is time-locked (FR-420), before
    the window's end or before the window has started: a run that time-locks
    before its window starts also ends `time-lock`, since FR-420 ends a
    sample at a time-lock reached before its horizon and no stutter position
    follows a time-lock. `k` is the key of the time-locked timed state and
    `t` the exact time stamp, a `Rational` in the model's unit, at which the
    run reaches it, so a race that advances time and takes no step is
    recorded;
  - `{"type": "idle-tail"}`: no step is drawn, the state is quiescent and not
    time-locked, the subject is `time dense`, and either the window has
    started and its end lies strictly after the last position's stamp, or
    the window has not started and a `when` guard or an `elapsed` measure's
    `A` that reads a clock can still start it at a stutter position of the
    idle tail. Its readings are then those of every completion (FR-420
    "Windows on an idle tail");
  - `{"type": "complete"}`: no step is drawn and none of the above holds:
    a `time tick` terminal state whose stutter positions fill the window, a
    quiescent state stamped at or after the window's end, or a quiescent
    state before a window that never starts because its predicates and
    `when` guard read the discrete state only;
- the path probability: the exact rational product of its step probabilities;
- the event's value, or the measure's value, on the sample, or the undefined
  value with its position, expression locus and catalogued reason. A measure
  censored at the horizon (FR-407: the window ended before its end event, so
  its value exceeds every finite threshold) has no finite value, and is
  written `{"type": "censored", "horizon": h}`, `h` the horizon bound it
  exceeded as a `Quantity` (`{"value": 10}` for `within 10` steps, with its
  `unit` for a time horizon).

A sampled witness is a behaviour of positive probability on the violating
side of the bound: the event is false on it for `>= θ` and true for `<= θ`.
So it is also a counterexample for the every-behaviour claim over the same
subject (FR-161), "the event holds on every behaviour" for `>= θ` and "the
event fails on every behaviour" for `<= θ`, and replays as one.
A sampled witness leaves the probability bound's `measured` decision as the
run's counts set it.

### Witness replay

A sampled witness replays by FR-181 replay (re-execution of each step through
the native transition relation, successor selection by post-state key, and
evaluation of the event, which for an undefined witness reproduces the
undefined value with its position, locus and reason), plus two checks:

1. each drawn random argument lies in its parameter's support;
2. each recorded draw, re-derived at its recorded choice index from the
   recorded seed, trace index and step (and `q` for a delay) by the sampler
   definition (`proposals/filament-v1/definitions/simulation-sampler.md`)
   and FR-420, without running the sampler over the witness, gives the
   recorded item or delay; the recorded draws are the draws the step
   consumes, in that order; they select the recorded scheduled identity and
   values; over a timed subject the racing identities are exactly those that
   race at the replayed timed state, the least recorded delay is the step's
   `delay`, ties ordered by the recorded tie-break draws, and the replayed
   run ends as `end` records. A delay draw with `choice` `null` is checked
   by confirming that its identity's window at the replayed timed state holds
   all of its distribution's mass in one point and that the recorded delay
   is that point. The checker applies the sampler definition's draw-derivation
   rule for one draw in its own code, from the recorded seed, trace index,
   step and `q`, and never runs the sampler's step or race procedure, so the
   record checker is independent of the sampler (FSL FR-1051).

Agreement settles the witness reproduced. A value outside its support refuses
`invalid_runtime_input`/`invalid-value`; a recomputed draw that selects another
identity, value, delay or run end, or a recorded draw list that is not the
step's draws, settles `inconclusive`, `replay-parity`, for that witness, as
does a `null`-choice delay draw whose window is not one point holding all
the mass, or whose delay is not that point. A step's `rewards` that is not one
entry per reward the operation declares, in bytewise name order with each
name once, refuses `invalid_runtime_input`/`invalid-value`; a reward value
other than the replayed step's settles `inconclusive`, `replay-parity`. The item's `measured` decision is unchanged by any witness replay.

A replay takes each draw from the witness record and does not run the sampler,
but a recorded draw is never trusted: the replayer re-derives it as in check 2
and compares. One changed recorded delay fails the recomputation, a losing
racer's delay as well as the winner's.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — every member above; the per-test entries and witnesses in their
  stated orders.
- **Absence** — an absent `interval` is admitted only for the sequential
  method, and an absent `estimate` only in the two decisions above; every other absent member refuses the result with
  `invalid_result`/`missing-member`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-410-AC-1 | A completed statistical run records every `StatisticalVerdict` member with exact rational values, one test entry per test, and `interval` absent exactly for the sequential method. Its `bound` equals the exact FR-413 claim object for accepted, rejected and undecided decisions, preserving type, comparison and threshold; for a quantile it also preserves the independent `level`. | Test (TC-358) |
| FR-410-AC-2 | A quantile run records both the conditional probability estimate and the nearest-rank empirical quantile. | Test (TC-355) |
| FR-410-AC-3 | The result keeps at most `max_witnesses` sampled witnesses, the first in trace-index order, each with its exact path probability. | Test (TC-359) |
| FR-410-AC-4 | A sampled witness replays as an FR-161 counterexample for the every-behaviour claim over the same subject. | Test (TC-359) |
| FR-410-AC-5 | Replay of a witness with a value outside its support refuses `invalid_runtime_input`/`invalid-value`; a recomputed draw selecting another identity settles `inconclusive`, `replay-parity`; neither changes the item's decision. | Test (TC-359) |
| FR-410-AC-6 | A witness step of `Server::send(x, r)` with `r` a random argument drawn as `3` writes `scheduled` as the transition identity of `send` whose `arguments` hold `x` only, and `random` `[3]`; a `scheduled` written as a bare typed value refuses at the schema. A witness that continues past a terminal state writes each stutter step with `transition` and `scheduled` `{"type": "stutter"}` and `random` `[]`, and is admitted; a stutter arm with an `operation` member refuses at the schema. | Test (TC-359) |
| FR-410-AC-7 | A sampled witness whose `steps from holds(A) until holds(B) within 10` measure is censored writes `value` `{"type": "censored", "horizon": {"value": 10}}` and is admitted; a quantile claim's estimate whose nearest-rank sample is censored writes `quantile` in the same form; a `censored` value without `horizon` refuses at the schema. | Test (TC-355) |
| FR-410-AC-8 | A statistical run whose test counted no sample settles `incomplete` with cause `{"type": "no-estimate", "reason": "no-counted-sample"}`, and a regenerative run that completed no positive-weight cycle with `{"type": "no-estimate", "reason": "no-positive-weight-cycle"}`; neither writes a `measured` verdict, and a `no-estimate` cause with a `limit` member refuses at the schema; an `accepted` `StatisticalVerdict` without `estimate`, or with an `estimate` lacking `value`, refuses at the schema. | Test (TC-358) |
| FR-410-AC-9 | A `StatisticalVerdict` whose `basis.samples` is the JSON integer `40` is admitted, and the same with `samples` the string `"40"` refuses at the schema. | Test (TC-358) |
| FR-410-AC-10 | A `StatisticalVerdict` whose decision is `rejected` with cause `undefined-evaluation` (a drawn sample evaluated the claim undefined, FR-409) and which carries no `estimate`, in the verdict or in its test entry, is admitted; the same verdict with an `estimate` added refuses at the schema. | Test (TC-358) |
| FR-410-AC-11 | A `StatisticalVerdict` whose decision is `undecided` with cause `undecided-successor` and which carries no `estimate`, in the verdict or in its test entry, is admitted; the same verdict with an `estimate` added refuses at the schema, while one `undecided` with cause `indifference-region` and no `estimate` refuses at the schema. | Test (TC-358) |
| FR-410-AC-12 | Accepted, rejected and undecided verdicts each carry the same closed form-specific bound as FR-413. The four type spellings and allowed members match FR-413; comparison preserves the checked claim's operator; a quantile includes its separate `level`. A missing/extra member, unknown type/comparison, non-rational threshold or level, or a bound inconsistent with the originating claim is refused by the result reader. | Test (TC-358) |
| FR-410-AC-13 | A sampled witness over a timed subject replays from its recorded draws without running the sampler, and the same witness with one recorded delay changed settles `inconclusive`, `replay-parity`, because the delay re-derived from the recorded seed, trace index, step, choice index and `q` differs. | Test (TC-374) |
| FR-410-AC-14 | A `StatisticalVerdict` whose `basis` has a `grid` and whose witness step records `draws` `[{"choice": 0, "delay": {"identity": reply, "delay": 1}}, {"choice": 1, "delay": {"identity": timeout, "delay": 5/2}}]` (the losing `timeout` included), `rewards` `[]` and `end` `{"type": "time-lock", "state": k, "stamp": 7/2}` is admitted, as are the `crossing` (with a witness step), `idle-tail` and `complete` ends, a delay draw with `choice` `null`, and, with no `grid`, a witness with `{"choice": 0, "item": 1}` draws and no `end`. Each of these refuses at the schema: a step without `draws` or without `rewards`; a draw with both `delay` and `item`, an item draw with `choice` `null`, a delay draw without `identity`; a reward without `name`; a witness without `end` under a `grid`, or with `end` and no `grid`; a `time-lock` end without `stamp`, a `crossing` end without `step`, an unknown end type, and a stutter step with a non-empty `draws` or `rewards`. A step whose `rewards` names one reward twice is admitted by the schema: reward order, duplicates and one entry per declared reward are checked at replay, not by the schema. | Test (TC-431) |
| FR-410-AC-15 | A sampled witness over a timed subject whose step records two racing identities, with the losing racer's recorded delay replaced by another delay of its window and support, settles `inconclusive`, `replay-parity`, because the delay re-derived at that draw's recorded choice index differs; the winner, the step and the item's `measured` decision are unchanged. A witness whose step's `rewards` names a declared reward twice, lists two rewards out of bytewise name order or omits a declared reward refuses `invalid_runtime_input`/`invalid-value`, and one whose recorded reward value differs from the replayed step's settles `inconclusive`, `replay-parity`; these are checked at replay (the FSL record checker), not by the schema. | Test (TC-374) |

## Dependencies

- [FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)
  produces the verdict.
- [FR-409](../protocol/FR-409-measured-decision.md) supplies the decision.
- [FR-331](FR-331-backend-provider-envelope.md) carries it in the result.
- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md)
  supplies the trace step content and replay.

## References

- FSL ADR-024 SV-2, SV-6 to SV-8, RP-1; FSL ADR-026 SS-3, SS-6; FSL ADR-038
  RL-6 (the draw record and run end).
- Ticket: STD-137 (paired with QSL-371).
