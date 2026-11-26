---
id: FR-407
title: "State probabilistic claims over events and measures"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-092
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-290
    type: references
  - target: ix://agent-ix/filament-specification/FR-408
    type: references
  - target: ix://agent-ix/filament-specification/FR-411
    type: references
  - target: ix://agent-ix/filament-specification/FR-414
    type: references
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
---

# FR-407: State probabilistic claims over events and measures

## Description

When a package declares a probabilistic claim, the frontend SHALL admit it
only in one of the forms below, over a model subject, with a scheduler clause,
and SHALL record its workload or every-scheduler clause with its fairness set, and any confidence parameters
it states in its obligation identity.

## Inputs

A `probabilistic` clause of the shared grammar's probabilistic forms; its
profile (FR-090), model subject, scheduler clause, activation and optional
confidence parameters; the model's rewards ([FR-405](FR-405-declare-probabilistic-models.md)).

## Outputs

An admitted claim with its form, event or measure, threshold, scheduler
clause, confidence parameters and obligation identity, or a located typed
refusal.

## Behavior

### Subject, kind and scheduler clause

A probabilistic claim's subject is a model subject. A claim whose subject is
a trace subject refuses with `invalid_probabilistic_model`/`trace-subject`,
and the diagnostic names window aggregates
([FR-414](../temporal/FR-414-evaluate-window-aggregates.md)) as the form for
statistics observed over a trace. Its FR-290 kind is
`probabilistic-satisfaction`. The profile its `using` alias selects is a
temporal profile ([FR-090](../temporal/FR-090-select-temporal-profile-and-clock.md));
any other profile refuses with
`invalid_probabilistic_model`/`not-temporal-profile`, naming the profile. An
`over (p: T)` parameter ranges over the objects of the subject's universes
(FR-406), so `T` is an object type that some population of the subject's
model admits; a scalar type, or an object type no population admits, has no
instance and refuses with `invalid_probabilistic_model`/`over-not-object-type`
at the parameter's name. Its scheduler clause is exactly one of
`under W` for a declared workload `W` of the subject's model, or
`under every scheduler`, optionally with a fairness set of weak and strong
constraints ([FR-406](FR-406-interpret-probabilistic-models.md)). The shared
grammar's `under W` takes no fairness items, so source text that writes
`under W fair weak Op` refuses `invalid_syntax`/`unexpected-token` at the
first `fair`; no other input carries a workload together with a fairness
set.
A clause naming a workload of another model refuses with
`invalid_probabilistic_model`/`foreign-workload`.

### Events

An **event** is a Boolean function of a behaviour that a finite prefix
decides. It is one of:

- a formula under a bounded profile, whose truth depends on the positions
  from its activating position up to its horizon `h` after it ("Activation") (FR-091, FR-092); interval
  operators nest in any order;
- under the timed profile, a bounded-window formula
  ([FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md)), decided by
  the positions whose time stamp is at most its time horizon after the
  activating position;
- a comparison `M <= c` or `M >= c` of a bounded measure `M` with a threshold
  `c` of `M`'s type.

A formula event whose operators are not all interval operators with an
interval bounded above, or whose profile is neither a bounded profile nor the
timed profile, refuses with
`invalid_probabilistic_model`/`unbounded-event`, and the diagnostic states
that a probabilistic event needs a finite window. A bounded-window formula
under the timed profile is an admitted event; a strict timed interval is
admitted by the frontend and decided exactly over the region abstraction
([FR-421](../temporal/FR-421-check-probabilistic-timed-claims-exactly.md)).
An interval-less operator under a profile other than the infinite-trace
profile never reaches this check: the temporal frontend refuses it first with
`unsupported_construct` ([FR-048](../foundation/FR-048-bind-native-temporal-syntax.md),
FR-048-AC-9), and so is a lower-bounded interval `[a,*]` under such a profile
(FR-048-AC-10). The bounded, event-position, fixed-sample and timed profiles
included, the operators a profile admits are decided once, by the temporal
frontend, for every formula wherever it stands, a temporal clause or a
probabilistic event alike; one spelling gets one refusal under one profile.
`unbounded-event` is the claim check's own refusal, for a formula whose every
operator its profile admits and which still has no finite window: under the
infinite-trace profile, an event that is not one of the reachability forms,
or that has an interval `[a,*]`. The one exception to `unbounded-event` for the infinite-trace profile is the
reachability forms of the forms table: under the infinite-trace profile an
event whose root is one `eventually`, `until` or `always` with no interval is
read as that form, and every other event under that profile, such as
`eventually[0, 5] holds(P)`, refuses `unbounded-event`. In those reachability
forms each operand is a `holds(P)` state predicate; an operand that is a
temporal operator (`eventually eventually holds(P)`) or a constant
(`eventually true`) refuses with
`invalid_probabilistic_model`/`not-state-predicate` at the operand.

The threshold `c` of a comparison over a fraction measure is a rational
strictly between 0 and 1. A dimensionless threshold stands in an `expr`
position of the shared grammar, so it is written `rational(n, d)` or
`decimal(n, s)`, never a bare decimal literal. A threshold of `0`, `1`, above
1 or below 0 refuses with
`invalid_probabilistic_model`/`fraction-bound-out-of-range`, naming the
bound; a bound on a fraction inside an event is no probability threshold, so
the refusal is never `qualitative-threshold`.

### Bounded measures

A **measure** is a value computed from one behaviour over a finite window.

- **Accumulate.** `accumulate R from holds(A) until holds(B) within h` sums
  reward `R` over the steps from the first position `a <= h` where `A` holds
  to the first position `b`, `a <= b <= h`, where `B` holds, that is over the
  steps into positions `a + 1` to `b`. A behaviour with no such `a` is **not
  activated**. A behaviour with `a` and no such `b` is **censored**, and its
  value exceeds every finite threshold.
- **Steps.** `steps from holds(A) until holds(B) within h` is accumulate with
  reward 1 per step.
- **Elapsed.** Under the timed profile with a `model-time` binding,
  `elapsed from holds(A) until holds(B) within h` is the time-stamp
  difference between two positions, and `h` is a time quantity. The first is
  the first position at or after the activating position where `A` holds,
  however late: `h` sets no limit on reaching it. The elapsed time starts
  there. The second is the first position at or after the first where `B`
  holds, and the behaviour is censored when the elapsed time passes `h`
  before it; a behaviour with no position where `A` holds is not activated.
  This is the elapsed clock of
  [FR-753](../temporal/FR-753-certify-time-divergent-and-elapsed-quantile-verdicts.md).
  It changes no window of a formula, `fraction`, `steps` or `accumulate`.
- **Fraction.** `fraction holds(P) over [0, h]` is the number of positions in
  the window where `P` holds over the number of positions in the window. A
  step horizon `h`, an integer, gives the window the positions 0 to `h` after
  the activating position, so the denominator is `h + 1`; a time horizon `h`
  gives the window "Time windows" states, and `h + 1` has no meaning for it.
  With `weighted by R`, each position
  `i` of the window counts with the reward of the step that leaves it, and the value is the
  weighted count over the total weight. Over a timed subject `weighted by
  delay` weights each position by the delay of the step that leaves it,
  clipped to the window: a position whose leaving step crosses the horizon
  `h` is weighted by `h − stamp`, `stamp` being the position's time stamp,
  and an idle-tail position with no leaving step inside the window is
  weighted by `h − stamp` as well, the time the trace spends at that
  position within the window. So the weights of a window sum to the
  duration it covers.

A window that reaches a terminal state continues on its stutter steps.

**Time windows.** Where `h` is a time horizon of a formula, a `fraction`,
`steps` or `accumulate`, the **window** read from the activating position `s`
is every position `i >= s` whose time stamp is at most `h` after `s`'s,
inclusive. Position `s` is in the window, so the window is never empty. The
post-state of each discrete step is its own position, several at one time
stamp included. A delay adds no position, and the instant `h` after `s` adds
none where no position is stamped there. A terminal stutter position
([FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) "Time
stamps") is a position like any other: under `time tick O period p` its stamp
is fixed, so the window holds exactly those stamped within it; under `time
dense` each choice of the stutter stamps is its own behaviour, so a window
that the idle tail enters before the window's end holds any number `m >= 0` of
stutter positions, and one whose idle tail begins at the window's end holds
none. The positions `a` and `b` of `accumulate` and `steps` within a time
horizon are positions of this window: the first position of the window where
`A` holds, and the first position of the window at or after it where `B`
holds. The positions of `elapsed` are those its rule above states. With
`weighted by R`, a window position is weighted by the reward of the step that
leaves it even when that step lands after the window's end, and a position
left by the terminal stutter step has weight 0
([FR-406](FR-406-interpret-probabilistic-models.md)). A statistical sample
whose reading depends on the choice of stutter stamps is settled as
[FR-420](FR-420-sample-stochastic-timed-models.md) "Windows on an idle tail"
states.

### Forms

| Form | Holds when, for every initial state and binding |
| --- | --- |
| `probability >= θ [E]`, `probability <= θ [E]` | The probability of `E` meets the bound. |
| `quantile q of M <= c`, `quantile q of M >= c`, with `M` an accumulate, steps or elapsed measure; a quantile over a fraction measure refuses with `invalid_probabilistic_model`/`quantile-over-fraction` | `Pr(M <= c | activated) >= q`, respectively `Pr(M < c | activated) < q`, with a strict `<`. The `q`-quantile of `M` is `inf {x : Pr(M <= x) >= q}`. It is at most `c` exactly when `Pr(M <= c) >= q`, since a distribution function is right-continuous. It is at least `c` exactly when `Pr(M <= x) < q` for every `x < c`; for a measure with finitely many values that is `Pr(M < c) < q`, and equality `Pr(M < c) = q` makes the claim false. A censored behaviour exceeds every finite threshold. |
| `mean of F >= θ`, `mean of F <= θ`, with `F` a fraction | The expectation of `F` meets the bound. A mean is admitted over a fraction measure only; a latency is stated as a quantile. |
| `long_run_fraction holds(P) >= θ` or `<= θ`, optionally `weighted by R` or `weighted by delay` | The expectation of the limit, as `n → ∞`, of the fraction over `[0, n]` meets the bound. In a finite Markov chain the limit exists with probability 1 and is constant on each bottom strongly connected component. |
| `probability >= θ` or `<= θ` over `[eventually holds(P)]`, `[holds(A) until holds(B)]` or `[always holds(P)]`, `P`, `A`, `B` state predicates, under the infinite-trace profile | The reachability or until probability meets the bound; `always holds(P)` reads `1 − Pr(eventually holds(not P))`. |
| `expected accumulate R until holds(B) <= c` or `>= c`; over a timed subject also `expected elapsed until holds(B) <= c` or `>= c` | The expectation from position 0 of `R` summed over the steps up to the first position where `B` holds meets the bound; where `B` is reached with probability below 1 the expectation exceeds every finite `c`. |

The last two rows are the **exact-only forms**.

A quantile claim reads a probability conditioned on its measure's
activation, and is read as "Conditioned values" below states for every
conditioned reading: under a workload whose activation has probability 0,
and under every scheduler when no scheduler activates, it holds vacuously.

### Thresholds and units

A probability threshold, a quantile level and a mean-of-fraction or long-run
threshold `θ` are exact rationals with `0 < θ < 1`; the frontend refuses `0` or `1` with
`invalid_probabilistic_model`/`qualitative-threshold`, naming the corresponding
every-behaviour or possible-property form. A reward or time threshold has the
reward's or time's dimension and converts exactly by FR-142. A bound with
equality meets a non-strict bound; the reduced bound of `quantile q of M >= c`
is strict, so equality fails it. Every estimate and value in a result is an
exact rational, in the threshold's unit where it has one.

### Confidence parameters

A claim may state three exact rationals: `alpha` (`α`), the greatest
probability of rejecting the claim when its bound holds; `beta` (`β`), the
greatest probability of accepting it when its bound fails; and `indifference`
(`ι`), with `0 < α < 1`, `0 < β < 1`, `0 < ι`, `θ − ι > 0` and `θ + ι < 1`.
A claim states each of the three exactly once, or none; a set that omits or
repeats one refuses with `invalid_probabilistic_model`/`partial-confidence`.
A stated parameter that breaks one of the five bounds refuses with
`invalid_probabilistic_model`/`invalid-confidence`, naming the parameter.
Statistical evidence requires them
([FR-408](FR-408-settle-probabilistic-claims-statistically.md)); exact evidence
reads none of them ([FR-411](FR-411-settle-probabilistic-claims-exactly.md)). A
claim that states none is checked with exact evidence only.

### Activation

A probabilistic claim's activation is `on origin` or `on each (x: T)`,
optionally with `when (g)`. `on origin` reads the claim's event or measure
from position 0 of every run. `on each (x: T) when (g)` makes one instance of
the claim per object of `T`'s population in the subject's universes, in
product with the `over` parameter's instances, and reads each instance from
its run's first activating position, the first position at which `g` holds
for that object (every position, with no `when`). Each instance is
conditioned on activation: a probability form reads
`Pr(event from the activating position | the run activates)`, an
expectation form reads the conditional expectation, and a long-run form,
whose limit does not depend on where it starts, reads the same conditional
value. A run that never activates is not counted. The conditioned value and
its vacuous case are those of "Conditioned values" below.
The claim holds when every instance holds; a population with no object gives
no instance, which settles `inconclusive` with `no-instance` (FR-331). `T`
must be an object type some population of the model admits, and `g` a
Boolean state expression; the frontend refuses any other `T` with
`invalid_probabilistic_model`/`over-not-object-type` and a non-Boolean `g`
with `ill_typed`/`type-mismatch`.

### Conditioned values

A reading is **conditioned** when the claim's activation is `on each`, or
its form is a quantile. Its **activation** `act` is the event that the run
reaches the reading's first activating position: for an `on each` instance,
the first position at which `g` holds for the instance's object; for a
quantile, the first position at which its measure activates, which under
`on each` lies at or after the instance's activating position. `E` stands
for the form's event, or for its measure's quantity in an expectation or
long-run form.

- **Under a workload that leaves no choice** the value is
  `Pr(E and act) / Pr(act)`, and for an expectation or long-run form
  `Ex(X · [act]) / Pr(act)` with `X` the form's quantity. Numerator and
  denominator are both computed exactly over the same Markov chain. The reading holds vacuously exactly when
  `Pr(act) = 0`, and that is decided by graph reachability: no activating
  state is reachable from the initial state along steps of positive
  probability. A vacuous reading reads no ratio.
- **Under every scheduler, and under a workload with free delays,** the
  value for a `>=` bound is the minimum, and for a `<=` bound the maximum,
  of the conditional value over the schedulers (with a fairness set, the
  fair schedulers; under a workload with free delays, the choices of those
  delays) that activate with positive probability. The ratio of a
  separately optimised numerator and denominator is not this optimum. It is computed exactly by the reset construction for
  conditional probabilities in Markov decision processes: every state from
  which activation is no longer reachable steps back to the initial state,
  and every end component that holds no activating state gains a step back
  to the initial state. Each first activating state is terminal and valued
  by the optimal value of the form read from it. The optimal expected
  terminal value of the reset process, over its schedulers that reach a
  terminal state with probability 1, is the conditional optimum. The
  optimum is attained by a scheduler that activates with positive
  probability, so a strict bound compares the value itself; a quantile's
  reduced bound selects the minimum or the maximum by its own direction. The reading holds vacuously exactly when no scheduler (no
  fair scheduler) activates, again decided by graph reachability on the
  product. A scheduler that never activates takes no part in the minimum or
  the maximum.

A vacuous reading holds and is never `refuted` nor `inconclusive`; it
differs from a population with no object, which gives no instance at all.

### Obligation identity

The obligation identity of a probabilistic claim includes its subject, form,
event or measure, threshold, scheduler clause (the workload's identity, or every scheduler with
its fairness set) and, when stated, its confidence parameters. One property
stated under two workloads, two fairness sets or two confidences is two
obligations. The claim's `over` parameter and activation fix the instances
its event or measure is read over and the position each is read from
("Activation"), so they are part of the event or measure in this identity.
The evidence kind, the statistical method and every budget are
request settings and are outside the obligation identity. A checked package
carries every member of this identity in the claim's node
([FR-640](../../objects/interfaces/FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)
"Probabilistic claim node").

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-407-AC-1 | Each form in the forms table is admitted over a model subject with one scheduler clause and requires the FR-290 kind `probabilistic-satisfaction`; a workload of another model refuses `invalid_probabilistic_model`/`foreign-workload`. | Test (TC-353) |
| FR-407-AC-2 | On a vector with activation at a known position, a not-activated behaviour is excluded from a quantile's conditional probability and a censored behaviour exceeds every finite threshold. | Test (TC-353) |
| FR-407-AC-3 | `quantile q of M <= c` and `probability >= q [M <= c]` conditioned on activation give the same verdict on every vector; the `P95` distribution of TC-351 gives `Pr(M <= 5 ms) = 24233/25000`. | Test (TC-353) |
| FR-407-AC-4 | A probability, quantile, mean-of-fraction or long-run threshold of 0 or 1 refuses `invalid_probabilistic_model`/`qualitative-threshold`; a reward threshold in another unit of the same dimension converts exactly. | Test (TC-353) |
| FR-407-AC-5 | A claim with all three confidence parameters is admitted with them in its obligation identity; a partial set refuses `invalid_probabilistic_model`/`partial-confidence`; two confidences of one property are two obligation identities. | Test (TC-353) |
| FR-407-AC-6 | Changing the workload or the fairness set changes the obligation identity. Changing the evidence kind, the method or a budget does not change it. | Test (TC-353) |
| FR-407-AC-7 | A weighted long-run fraction and a `weighted by delay` fraction are admitted and evaluate as the weighted ratio. | Test (TC-353, TC-374) |
| FR-407-AC-8 | `quantile q of M >= c` reduces to the strict bound `Pr(M < c \| activated) < q`: with `M` equal to 1 or 3 with probability `1/2` each, `quantile 0.5 of M >= 3` is false (the quantile is 1, and `Pr(M < 3) = 1/2` is not below `1/2`), and `quantile 0.5 of M >= 1` is true. | Test (TC-353, TC-360) |
| FR-407-AC-9 | A quantile claim, in either form, over a subject whose activation has probability 0 holds, under a workload and under every scheduler. | Test (TC-353) |
| FR-407-AC-10 | A claim whose subject is a trace subject refuses `invalid_probabilistic_model`/`trace-subject` naming window aggregates; one whose `using` alias selects a profile that is not a temporal profile refuses `not-temporal-profile` naming it; and an `over` parameter of a scalar type, or of an object type no population of the model admits, refuses `over-not-object-type` at the parameter's name, while `over (m: Node)` over `Health` is admitted. | Test (TC-353) |
| FR-407-AC-11 | Under the event-position false-extension profile, an event `always holds(n.healthy)` with no interval refuses `unsupported_construct` at the temporal frontend (FR-048-AC-9) and never reaches the claim check; under the timed profile a bounded-window event `eventually[0 ms, 4 ms] holds(P)` is admitted; under the infinite-trace profile `[eventually holds(P)]` is admitted as a reachability form while `[eventually[0, 5] holds(P)]` and `[eventually[2, *] holds(P)]` refuse `invalid_probabilistic_model`/`unbounded-event` and `[eventually eventually holds(P)]` and `[eventually true]` refuse `not-state-predicate` at the operand. | Test (TC-353) |
| FR-407-AC-12 | `Monthly` with the event `fraction holds(v.up) over [0, 10000] >= rational(999, 1000)` is admitted; the same event with threshold `0`, `1` or `rational(3, 2)` refuses `invalid_probabilistic_model`/`fraction-bound-out-of-range` naming the bound, never `qualitative-threshold`. | Test (TC-353) |
| FR-407-AC-13 | A `quantile 0.95 of (fraction holds(v.up) over [0, 10]) <= rational(1, 2)` claim refuses `invalid_probabilistic_model`/`quantile-over-fraction`. | Test (TC-353) |
| FR-407-AC-14 | `NoFault` with `indifference 0.002` (`θ + ι = 1.001`) refuses `invalid_probabilistic_model`/`invalid-confidence` naming `indifference`, and with `alpha 1` it refuses `invalid-confidence` naming `alpha`. | Test (TC-353) |
| FR-407-AC-15 | `LongRun` with its scheduler clause written `under Steady fair weak Server::tick` refuses `invalid_syntax`/`unexpected-token` at `fair`, and with it written `under every scheduler fair weak Server::tick` it is admitted with the fairness set in its obligation identity. | Test (TC-353) |
| FR-407-AC-16 | Under a bounded profile, `probability >= 0.99 [always holds(n.healthy)]` and `probability >= 0.99 [eventually[2, *] holds(n.healthy)]` each refuse `unsupported_construct` at the operator or interval span with the owning claim and selected profile, exactly as the same formula in a temporal clause under that profile refuses, and neither refuses `unbounded-event`; under the timed profile `eventually holds(P)` with no interval refuses `unsupported_construct` and `eventually[0 ms, 4 ms] holds(P)` is admitted; under the infinite-trace profile `[eventually[2, *] holds(P)]` refuses `invalid_probabilistic_model`/`unbounded-event`. | Test (TC-1562) |
| FR-407-AC-17 | On a vector with two objects of `T`, where `g` first holds for the first at position 1 and never for the second, `on each (x: T) when (g)` gives one activated instance, read from position 1 and conditioned on activation, and one instance that never activates and holds vacuously; the same claim over a population with no object settles `inconclusive`, `no-instance`; `on each (k: Integer)` refuses `over-not-object-type`. | Test (TC-1650) |
| FR-407-AC-18 | A model's initial state `s0` has the action `a`, which reaches with probability `1/4` a state where `g` first holds and the event then holds with probability 1, and with probability `3/4` a state from which `g` is unreachable, and the action `b`, which reaches with probability 1 a state where `g` first holds and the event then holds with probability `1/2`. For `on each (x: T) when (g) { probability >= θ [E] }` over every scheduler, the conditioned minimum is `1/2` and the maximum is 1 (the unconditioned values `1/4` and `1/2` are wrong): `>= rational(1, 2)` holds, `>= rational(3, 5)` fails, and `<= rational(9, 10)` fails. Under a workload choosing `a` and `b` with `1/2` each, `Pr(act) = 5/8`, `Pr(E and act) = 3/8` and the value is `3/5`, so `>= rational(3, 5)` holds and `>= rational(2, 3)` fails. A variant where neither action reaches `g` holds vacuously under both scheduler clauses. | Test (TC-2060) |
| FR-407-AC-19 | Over a timed trace with three positions at time stamps 0, 1 and 6, where `P` holds only at the second, `fraction holds(P) over [0, 4] weighted by delay` weights the first position 1 (its leaving step ends at 1) and the second `4 − 1 = 3` (its leaving step crosses the horizon 4), counts no weight for the third (stamp 6 is outside the window), and is `3/4`; the weights sum to the window's duration 4. | Test (TC-353) |
| FR-407-AC-20 | Over a timed trace with two positions at time stamps 0 and 1, the second of which begins the idle tail (no step leaves it), where `P` holds only at the first, `fraction holds(P) over [0, 4] weighted by delay` weights the first position 1 and the idle-tail position `4 − 1 = 3`, the time the trace spends there within the window, and is `1/4`. | Test (TC-353) |
| FR-407-AC-21 | An unweighted fraction over a time window divides by the positions of the window. With positions stamped 0, 1, 5 and 6, `P` true at those stamped 1 and 5, `fraction holds(P) over [0, 5]` is `2/3` (not `2/6`). With positions stamped 0, 5, 5 and 6, `P` true only at the first position stamped 5, it is `1/3`. With positions stamped 0, 2, 7 and 8, `P` true at those stamped 2 and 7, read from the activating position stamped 2, the window is the positions stamped 2 and 7 and the value is 1. With positions stamped 0, 0 and 1, `P` true only at the first, `fraction holds(P) over [0, 0]` is `1/2`. | Test (TC-353) |
| FR-407-AC-22 | Over positions stamped 0, 2 and 7, the step leaving the first having reward 1 and the step leaving the second, which lands at 7, reward 3, with `P` true only at the second, `fraction holds(P) over [0, 5] weighted by R` is `3/(1 + 3) = 3/4`. With the second position instead beginning an idle tail, the value is `0/(1 + 0) = 0` for every choice of stutter stamps. | Test (TC-353) |
| FR-407-AC-23 | Over positions stamped 0, 2, 2 and 7, the three steps having rewards 1, 3 and 2. Where `A` first holds at the first position and `B` first holds at the third, `steps from holds(A) until holds(B) within 5` is 2 and `accumulate R` is `1 + 3 = 4`; where `B` first holds at the fourth, stamped 7, both are censored. Where `A` first holds at the fourth, both are not activated. | Test (TC-353) |
| FR-407-AC-24 | Under `time tick O period 2`, a behaviour whose initial position, where `P` is false, is left by one step of another operation to a terminal state where `P` is true has positions stamped 0, 0, 2, 4, 6 and so on. `fraction holds(P) over [0, 5]` and `over [0, 4]` are both `3/4`, and `over [0, 1]` is `1/2`. | Test (TC-353) |
| FR-407-AC-25 | Over a `time dense` behaviour with positions stamped 0 and 2, the second beginning an idle tail, `P` true only at the second, `fraction holds(P) over [0, 5]` over the behaviour with `m` stutter positions stamped within the window is `(1 + m)/(2 + m)`: `1/2` for `m = 0`, `2/3` for `m = 1`, `3/4` for `m = 2`. Where the second position is stamped 5, `m` is 0 and the value is `1/2`. | Test (TC-353) |
| FR-407-AC-26 | `elapsed from holds(A) until holds(B) within 4` starts its time at the first position where `A` holds. Over positions stamped 0, 3 and 6, `A` holding at the second and `B` at the third, it is `6 − 3 = 3`, while `steps from holds(A) until holds(B) within 4` over the same positions is censored. With the third position stamped 7 the elapsed value is 4; stamped 8, it is censored. Over positions stamped 0, 9 and 12, `A` at the second and `B` at the third, the elapsed value is 3 and the `steps` measure is not activated. With no position where `A` holds, the elapsed measure is not activated. | Test (TC-353) |

## Dependencies

- [FR-406](FR-406-interpret-probabilistic-models.md) supplies the measure each
  form is read under.
- [FR-090](../temporal/FR-090-select-temporal-profile-and-clock.md),
  [FR-091](../temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) supply bounded events.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  units and conversion.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) carries the
  kind `probabilistic-satisfaction`.

## References

- FSL ADR-024 PF-1 to PF-10, PM-8; FSL ADR-028 SP-4, XF-1 to XF-8; FSL ADR-026
  SS-1.
- Owner rulings recorded on QSL-371: confidence parameters appear only for
  statistical evidence, and a claim without them is exact-only; weighted
  long-run claims are kept; mean latency is dropped in favour of quantiles;
  expected reward and time are exact-only.
- C. Baier, J. Klein, S. Klüppelholz and S. Märcker, Computing conditional
  probabilities in Markovian models efficiently, TACAS 2014: the reset
  construction of "Conditioned values".
- Ticket: STD-137 (paired with QSL-371).
