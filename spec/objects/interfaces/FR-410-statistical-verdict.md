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
| `bound` | The exact FR-413 claim-bound object judged for this item. Its closed `type` is `probability`, `quantile`, `mean-of-fraction` or `long-run-fraction`; its `comparison` preserves the checked claim's operator: `greater-or-equal`, `less-or-equal`, `greater`, or `less`; `threshold` is the exact FR-413 rational, and a `quantile` bound also carries its independent exact `level`. It is present for every accepted, rejected and undecided verdict, including a rejection by `undefined-evaluation`. |
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

A sampled witness is a counted sample on which the event is false, or whose
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
- the path probability: the exact rational product of its step probabilities;
- the event's value, or the measure's value, on the sample, or the undefined
  value with its position, expression locus and catalogued reason. A measure
  censored at the horizon (FR-407: the window ended before its end event, so
  its value exceeds every finite threshold) has no finite value, and is
  written `{"type": "censored", "horizon": h}`, `h` the horizon bound it
  exceeded as a `Quantity` (`{"value": 10}` for `within 10` steps, with its
  `unit` for a time horizon).

A sampled witness is a behaviour of positive probability on which the event is
false, so it is also a counterexample for the every-behaviour claim "the event
holds on every behaviour" over the same subject (FR-161) and replays as one.
A sampled witness leaves the probability bound's `measured` decision as the
run's counts set it.

### Witness replay

A sampled witness replays by FR-181 replay (re-execution of each step through
the native transition relation, successor selection by post-state key, and
evaluation of the event, which for an undefined witness reproduces the
undefined value with its position, locus and reason), plus two checks:

1. each drawn random argument lies in its parameter's support;
2. each draw, recomputed from the seed, trace index, step and choice index
   with the sampler, selects the recorded scheduled identity, values and,
   over a timed subject, the recorded delay.

Agreement settles the witness reproduced. A value outside its support refuses
`invalid_runtime_input`/`invalid-value`; a recomputed draw that selects another
identity, value or delay settles `inconclusive`, `replay-parity`, for that
witness. The item's `measured` decision is unchanged by any witness replay.

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

## Dependencies

- [FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)
  produces the verdict.
- [FR-409](../protocol/FR-409-measured-decision.md) supplies the decision.
- [FR-331](FR-331-backend-provider-envelope.md) carries it in the result.
- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md)
  supplies the trace step content and replay.

## References

- FSL ADR-024 SV-2, SV-6 to SV-8, RP-1; FSL ADR-026 SS-3, SS-6.
- Ticket: STD-137 (paired with QSL-371).
