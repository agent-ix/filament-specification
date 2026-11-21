---
id: FR-412
title: "Probability certificate and its checker"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-411
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-201
    type: references
---

# FR-412: Probability certificate and its checker

## Description

When an exact probabilistic provider proves a bound, it SHALL emit a
`ProbabilityCertificate` with exactly the members below, and the checker SHALL
accept it, by re-enumerating the product and checking the stated conditions in
exact rationals with no provider present, before the item settles `proved`.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| identity | JsonObject | 1..1 |  |
| objective | JsonObject | 1..1 |  |
| kind | String | 1..1 |  |
| values | JsonObject | 0..* |  |
| ranking | JsonObject | 0..* |  |
| policy | JsonObject | 0..* |  |
| components | JsonObject | 0..* |  |
| conditional | JsonObject | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| identity | JSON object `{node_id, model, ordinal}`; `proposals/backend-provider-v1/schema.json` `CertificateIdentity` |
| objective | JSON object; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.objective` |
| kind | string tag lower, upper, exact, long-run or conditional |
| values | array of `{state, value}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.values` |
| ranking | array of `{state, rank}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.ranking` |
| policy | array of `{state, choice}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.policy` |
| components | for `long-run`: per component, its value and a gain-bias pair |
| conditional | for `conditional`: JSON object tagged by `mode`, `ratio`, `optimum` or `vacuous`; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.conditional` |


`ProbabilityCertificate{identity, objective, kind, values, ranking, policy,
components, conditional}`:

| Member | Content |
| --- | --- |
| `identity` | The item's obligation identity, the structural `{node_id, model, ordinal}` of FR-418-AC-9, that binds the proof to the exact subject and claim it proves. |
| `objective` | The target or decided states, the reward, the optimisation, and the claim's fairness set when it states one. The optimisation is `minimum` or `maximum` over every scheduler; `workload` under a workload that leaves no choice; or `workload-minimum` or `workload-maximum` under a workload over a timed subject with free delays, the minimum or maximum over those delays' choices. |
| `kind` | `lower`, `upper`, `exact`, `long-run` or `conditional`. |
| `values` | One exact rational per product state the certificate names (a dyadic value is an exact rational), keyed by product state key (FR-411). |
| `ranking` | For `lower` and `exact`, and for `upper` under a fairness set with the optimisation `minimum`: one natural number per named state. A `lower` certificate under a fairness set with the optimisation `minimum` carries none ("Fair minimum"). For an expected reward, `exact` and `upper` carry it and `lower` carries none ("Expected reward"). |
| `policy` | For a lower bound on a maximum or an upper bound on a minimum, including `workload-maximum` and `workload-minimum`: one choice per named state, a memoryless deterministic scheduler. For the `reset` part of an `optimum` certificate: one choice per waiting state: a transition identity, `{"type": "reset"}`, or with a fairness set a randomized choice over them. |
| `components` | For `long-run`: per component, its value `ρ_C` and a gain–bias pair. |
| `conditional` | For `conditional`, the certificate of a conditioned reading (FR-407 "Conditioned values"): `{mode: "ratio", numerator, denominator}`, `{mode: "optimum", post, reset}` or `{mode: "vacuous"}`, each part an `exact` certificate of this form. |

A certificate names its states and carries no qualitative sets; the checker
recomputes them.

Every state key in `values`, `ranking`, `policy`, each component's `bias`
and a state-located rejection SHALL preserve FR-411's complete product key:
the `over` binding, the `on each` instance and its activation phase, the
model state key, monitor state and accumulator, including the measure's
activation flag. For the intermediate states of FSL
FR-196 / ADR-028 SCH-2, it SHALL also preserve the pending scheduled
identity and drawn vector. For an exact-only form, the absent monitor and
accumulator remain absent. A model state key alone SHALL NOT stand for
a bounded or intermediate product state.

FSL FR-196 / ADR-028 PR-4 now own the complete canonical octet basis:
`["filament.probability-product-key/v1", binding, instance, activation, model, monitor, accumulator, pending]`
in RFC 8785 UTF-8, with every component present and actual None as null.
`instance` is the reference of the `on each` object the state is read for
and `activation` is `"waiting"` or `"active"`; both are null for an
`on origin` claim. Two states that differ only in instance or activation
phase are distinct states.
FR-331 transports those complete octets as `ProbabilityProductKey` in
each existing `state` row member and at `{product_key}` in a rejection.
The model is the full canonical simulation-state body;
formula monitors embed the complete parsed FR-338 array. The checker
SHALL admit every component under the actual subject/claim and compare
complete decoded canonical octets. Distinct accumulator values,
digital tail modes, pending action/draw vectors and genuine empty drawn
vectors SHALL remain distinct; None is not a fabricated zero component.

## Checker

`check_probability_certificate` recompiles the package, re-admits the subject
and re-enumerates the product through the native transition relation and the
monitor: every reachable product state for an `upper`, `exact` or `long-run`
certificate; the named states and their successors for a `lower` one. It
recomputes the decided states, the `Prob0` and `Prob1` sets, the components
and, with a fairness set, the maximal fair end components (FR-411) by its own
graph code, and checks the conditions
below in exact rationals. It solves no equation and runs no iteration.

With `F` the one-step operator of the objective (the workload's average; the
minimum or maximum over actions; for `workload-minimum` or `workload-maximum`,
the workload's average over drawn outcomes with the minimum or maximum over
the free delays' choices; or the policy's action), and values fixed at
decided, target and `Prob0` states:

- **Upper**: `F(y) <= y` at every state, so the value, the least fixed point,
  is at most `y`.
- **Lower**: `x <= F(x)` at every named state, and every non-target state with
  `x > 0` has (under the workload, under the policy, or for a minimum under
  every action) a successor with positive probability, `x > 0` and a smaller
  rank; then `x` is at most the value. An unnamed successor counts as the
  objective's least value: 0 for a probability or an expected reward, `−q`
  for FR-411's shifted reward of a quantile, so an unnamed state never
  raises a lower bound.
- **Exact**: both conditions, with equality.
- **Finite horizon**: the ranking is the remaining horizon, which the checker
  reads from the monitor state.
- **Finite-horizon measure**: over the product of an accumulate, steps or
  fraction measure (FR-407 "Bounded measures"), the decided states take the
  payoff of the form, which the checker computes from the measure's control
  and accumulator in the state's key: 1 or 0 for a comparison event;
  `[activated and M <= c]` (`M < c` for the `>= c` form) for the numerator
  and `[activated]` for the denominator of a quantile's `ratio` under a
  workload; FR-411's shifted reward `[activated] · ([M <= c] − q)` (or
  `[M < c]`) for a quantile under every scheduler; the completed fraction
  for a mean. Censored counts as above every threshold and an unactivated
  measure as not activated. The ranking is the remaining window. A quantile
  under every scheduler is on the claim's side when the shifted value at the
  initial state is at least 0 for `<= c` (an `exact` or `lower`
  certificate), or below 0 for `>= c` (an `exact` or `upper` certificate),
  or 0 with no activated state reachable from the initial state through
  tight actions, those whose `Q(y, a)` equals `y`.
- **Expected reward**: the same conditions with the reward added to `F`, and
  the states whose value exceeds every finite threshold recomputed by graph.
  Bounds swap the roles of the ranking: a `lower` certificate (`x <= F(x)`,
  under every action for a minimum, under the policy for a maximum) needs
  none, since an improper scheduler has value `+∞` and a proper one has a
  unique fixed point that dominates every post-fixed point; an `upper`
  certificate (`F(y) <= y`) carries a properness ranking, every non-target
  state having a successor of positive probability and smaller rank under
  the workload, under the policy for a minimum, or under every action for a
  maximum, because a non-target end component of cost 0 admits the finite
  pre-fixed point 0 although its value is `+∞`.
- **Fair minimum**: with a fairness set and the optimisation `minimum`, let
  `U` be the union of the maximal fair end components of the product
  restricted to the states where the target does not hold (FR-411
  "Reachability"). Values are 1 at target states and 0 at every state of
  `U`. The value is `1 − max Pr(not target until U)`, so the conditions are
  those of that maximum read for its complement. `exact`: at every other
  state `x = min_a Q_a(x)` over every action, and every such state with
  `x < 1` has a tight action (`Q_a(x) = x`) with a successor of positive
  probability and a smaller rank, the states of `U` and the states with
  value 1 having rank 0. `lower`: `x <= Q_a(x)` under every action, with no
  ranking, since a pre-fixed point of the maximum dominates it. `upper`:
  `Q_σ(y) <= y` under the policy and the same progress under the policy
  toward `U` and the states with value 1, so the policy reaches them almost
  surely and a fair continuation exists from each. With a fairness set and
  the optimisation `maximum` the conditions are those without a fairness
  set, because the supremum over fair schedulers is the maximum and a
  scheduler attaining it continues fairly once its value is decided.
- **Long-run**: per component, the gain–bias pair satisfies the multichain
  optimality equations for the reward `R · ([P] − ρ_C)` with gain 0, as
  equalities under the component's policy and as inequalities over every
  action, plus an `exact`, `lower` or `upper` certificate for the expected
  component value.
- **Conditional**: for a conditioned reading (FR-407 "Conditioned values").
  The checker rebuilds the activating states, the states from which
  activation is reachable and the reset process itself, by its own graph
  code; no part of the certificate supplies them.
  - `ratio`, only under the objective `workload` (a workload that leaves no
    choice): `numerator` and `denominator` are `exact` certificates over the
    same Markov chain, for `Pr(E and act)` (or `Ex(X · [act])`) and for
    `Pr(act)`, the reachability of the first activating states. Each passes
    its own conditions. At each initial state the certified value is the
    numerator's value over the denominator's; where the denominator is 0 the
    checker confirms by its own graph search that no activating state is
    reachable, and that instance holds vacuously.
  - `optimum`, under `minimum`, `maximum`, `workload-minimum` and
    `workload-maximum`: `post` is an `exact` certificate of the form's
    optimal value from every first activating state, over the product read
    from activation on. `reset` is an `exact` certificate over the reset
    process the checker rebuilt. It names waiting states only, and each
    terminal copy of a first activating state carries no row and takes
    `post`'s value at that key. `reset` is checked over proper schedulers,
    those that reach a terminal state with probability 1, with no `Prob0`
    or `Prob1` set computed in the reset process.
    - Its `values` satisfy `x <= F(x)` for a minimum, or `F(y) <= y` for a
      maximum, at every waiting state under every action, the reset action
      included.
    - Its `policy`, one choice per waiting state, attains them with
      equality.
    - Its `ranking` gives every waiting state a successor of positive
      probability under the policy with a smaller rank, so the policy is
      proper.

    `values`, `policy` and `ranking` each cover exactly the waiting states
    the checker rebuilt as reachable from the initial states, one row each.
    The rule that an unnamed successor counts as value 0 does not apply to
    `reset`: a waiting state with no row would let a maximum ignore a
    successor of high value. A missing, extra or duplicate row is rejected
    `malformed` at the incomplete member (`values`, `policy` or
    `ranking`), before any arithmetic, because coverage is a property of
    the certificate's shape and not of one state's value.

    A value reached only by staying in a waiting end component forever,
    such as 0 for a minimum, therefore never passes. The certified value is
    `reset`'s value at each initial state from which activation is
    reachable; at any other initial state the checker confirms vacuity by
    its own graph search.
  - `vacuous`: for a reading in which every instance is vacuous. It carries
    no parts and an empty `values`. The checker accepts it exactly when its
    own graph search finds no activating state reachable from any initial
    state, and under every scheduler with a fairness set none from which a
    fair continuation exists.

  A policy choice in `reset` is a transition identity, `{"type":
  "reset"}` (the reset process's step back to the initial state), or
  `{"type": "randomized", "choices": [{choice, probability}, ...]}` over
  two or more of those with exact positive probabilities that sum to 1.
  Without a fairness set a deterministic memoryless proper policy attains
  the optimum, so a randomized choice is never needed there and is
  rejected `malformed` at `policy`. With a fairness set the attaining fair
  policy may have to mix its choices inside a fair end component, as
  FR-413's fair witness does. So a randomized choice is admitted then, and
  the checker applies its weighted operator and checks the fair-end-
  component condition on the chain it induces.

The checker accepts a certificate that meets its conditions and puts the
initial state's value on the claim's side of the threshold: `lower` for a
proof of `>= θ` or a refutation of `<= θ`; `upper` for a proof of `<= θ` or a
refutation of `>= θ`; `exact` and `conditional` for either. A `conditional`
certificate is on the claim's side when its certified value is, and a
`vacuous` one is on the side of a proof. A `value-bounds` proof (FR-411)
carries a `lower` and an `upper` certificate; the one on the claim's side
meets this rule, and the other is checked by its conditions alone, so that
both ends of the reported interval are certified. A `lower` proof found on a
partial product (FR-411) is the one exception: it carries no `upper`
certificate, and its basis says `exploration: "partial"` and reports the
objective's least upper bound (FR-331). It refuses a certificate whose
`identity` differs from the item's with `invalid_runtime_input`/`conflicting-identity`.
A rejected certificate settles the item `inconclusive`,
`certificate-rejected{rule, at}`, `at` the complete product state key where
it failed, preserving the same components as the certificate's keys
(FR-411, FR-331): rule `inequality-fails` where an
upper, lower, exact or long-run condition above fails; at an initial state
where a `ratio` denominator is 0 although activation is reachable from
it, or where a `vacuous` certificate is offered and an activating state is
reachable; for a `conditional` certificate, the first failing part, checked
in the order `numerator`, `denominator` or `post`, `reset`, with that
part's own rule at `{product_key, part}`, `part` naming the failing part;
`rank-broken` also where a `reset` policy state has no successor of
positive probability under its choice with a smaller rank; `rank-missing` at a
named non-target state with `x > 0` and no rank; `rank-broken` at such a state
with no successor of positive probability, `x > 0` and a smaller rank; and
`threshold-missed` at the initial state when its value is not on the claim's
side of the threshold;
a checker stopped by a budget settles `incomplete`, `limit-reached` naming
the budget. The checker runs under the request's budgets; its cost is the
re-enumeration plus one pass over the edges.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — every member above.
- **Absence** — a `lower` or `exact` certificate without `ranking` (except a
  fair-minimum or expected-reward `lower`), a fair-minimum or expected-reward
  `upper` without `ranking`, a lower
  bound on a maximum or an upper bound on a minimum without `policy`, or a `long-run` certificate without
  `components`, a `conditional` certificate without `conditional`, with a
  `mode` other than `ratio`, `optimum` and `vacuous`, with a `ratio` under
  an objective other than `workload`, without a part its mode requires
  (`numerator`, `denominator`, `post`, `reset`), with a part whose `kind`
  is not `exact`, or with a `reset` part without `policy` or `ranking`, is
  rejected, `certificate-rejected{rule, at}` with rule `malformed` at
  `{member}`, the missing or wrong member's name (`mode` for a wrong or
  misplaced mode).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-412-AC-1 | The checker accepts the `Deliver` `exact` certificate (TC-363) (values `24/25`, `4/5`, 1 at delivered states, 0 at the two-loss state) after checking the minimum over both actions at each live state. | Test (TC-363) |
| FR-412-AC-2 | The checker accepts the `LongRun` `long-run` certificate (TC-362) (gain `1800/1801`, bias 0 at `up` and `−2000/1801` at `down`) by exact equality. | Test (TC-362) |
| FR-412-AC-3 | A certificate that violates one inequality settles `inconclusive`, `certificate-rejected` with rule `inequality-fails` at that state; a `lower` certificate with a missing rank, `rank-missing` at the unranked state; one with a rank that does not decrease, `rank-broken` at that state; one whose initial value is on the wrong side of the threshold, `threshold-missed` at the initial state; and a `lower` certificate without `ranking`, `malformed` at member `ranking`. | Test (TC-363) |
| FR-412-AC-4 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-363) |
| FR-412-AC-5 | A `lower` certificate is checked over its named states and their successors only, with each unnamed successor at value 0 (at `−q` for a shifted reward). | Test (TC-363) |
| FR-412-AC-6 | The checker recomputes every qualitative set and component itself and reads no other output of the provider. | Test (TC-363) |
| FR-412-AC-7 | The checker accepts the `FairTerminates` `exact` certificate, whose objective names the fairness set, after recomputing that no fair end component exists. | Test (TC-363) |
| FR-412-AC-8 | Two named product states with the same model state but different monitor states or accumulators retain distinct keys in `values`, `ranking`, `policy`, component `bias` and state-located rejections after serialization and reading. Intermediate states that differ only in their pending action or draw also remain distinct; replacing either complete key with the model state key alone fails the comparison. | Test (TC-363) |
| FR-412-AC-9 | Over FR-407-AC-18's model, the `optimum` `conditional` certificate whose `post` values are 1 and `1/2` at the two first activating states and whose `reset` value at `s0` is `1/2` with policy `b` is accepted for `>= rational(1, 2)` and fails `threshold-missed` at `>= rational(3, 5)`; with `reset`'s value at `s0` set to the unconditioned `1/4` it is rejected `inequality-fails` at `{s0, part: reset}`, and with `post`'s `1/2` set to `3/5` it is rejected `inequality-fails` at that state with `part` `post`. The `ratio` certificate with numerator `3/8` and denominator `5/8` is accepted for `>= rational(3, 5)`; with denominator 1 it is rejected `inequality-fails` with `part` `denominator`. | Test (TC-2061) |
| FR-412-AC-10 | A `vacuous` certificate is accepted over FR-407-AC-18's variant where `g` is unreachable and rejected `inequality-fails` at the initial state over the model itself; a `conditional` certificate without `conditional` is rejected `malformed` at `conditional`, a `ratio` one without `denominator` `malformed` at `denominator`, a `ratio` one under the objective `minimum` `malformed` at `mode`, and one with mode `"workload"` `malformed` at `mode`. Two states that differ only in `instance` or only in `activation` keep distinct keys after serialization and reading. | Test (TC-2061) |
| FR-412-AC-11 | Over FR-407-AC-18's model with a further action `stay` that loops at `s0` without activating, for `>= rational(1, 2)` under every scheduler: the `optimum` certificate whose `reset` has value `1/2` at `s0` and at the state after `a` from which `g` is unreachable, policy `b` at `s0` and ranks 1 and 2 is accepted (`1/2 <= 1/2` under `stay`, the reset choice and `b`, `1/2 <= 1/4 + (3/4)(1/2) = 5/8` under `a`). A `reset` with value 0 at both states is rejected: with policy `stay` or the reset choice `rank-broken` at `{s0, part: reset}`, with policy `b` `inequality-fails` there (`F_b = 1/2`). The minimum over all schedulers, 0, is therefore never certified. | Test (TC-2061) |
| FR-412-AC-12 | On FR-412-AC-11's model, a maximum `reset` certificate for `<= rational(9, 10)` with `y(s0) = 1/2`, policy `b` and no row for the state `u` after `a` from which `g` is unreachable is rejected `malformed` at `values`, and does not prove the claim. Read with `u` counted as 0, it would pass `F_a(y) = 1/4 + (3/4)(0) = 1/4 <= 1/2`, although the true maximum is 1. With `u` at `1/2` it is rejected `inequality-fails` at `{s0, part: reset}`, since `F_a(y) = 1/4 + (3/4)(1/2) = 5/8 > 1/2`. Without a fairness set, a `reset` policy entry that is randomized is rejected `malformed` at `policy`. | Test (TC-2061) |
| FR-412-AC-13 | A model `W` whose live state `s` has the actions `wait`, back to `s` with probability 1, and `go`, to the target `g` and the non-target terminal `f` with `1/2` each, under `fair strong go`, minimum: the `exact` certificate with values `s` `1/2`, `g` 1, `f` 0 and ranks 1, 0, 0 is accepted for `>= 1/2` (`f`'s terminal stutter is a fair end component, so it lies in `U`); with `s` at 0 or `1/4`, which satisfy the minimum through `wait`, it is rejected `rank-broken` at `s`, and with its fairness set removed the `1/2` certificate is rejected `rank-broken` at `s`. The `lower` certificate `s` `2/5` with no ranking is accepted for `>= 2/5`, and `s` `3/5` is rejected `inequality-fails` at `s` (`Q_go = 1/2`). The `upper` certificate `s` `3/5` with policy `go` and rank 1 is accepted as the upper side of a value-bounds proof; `s` `1/4` with policy `wait` is rejected `rank-broken` at `s`. Over the variant whose `s` has a further action `drift` to a state `u` whose only action loops at `u`, `u` lies in `U` and the minimum at `s` is 0: values `s` 0, `u` 0 with ranks 1, 0 are accepted, and `u` at `1/2` is rejected `inequality-fails` at `u`. | Test (TC-2110) |
| FR-412-AC-14 | A workload chain whose state `s` goes to itself, to the target `g` and to the non-target terminal `f` with `1/3` each (value `1/2`): the `lower` certificate `s` `7/16`, rank 1, is accepted for `>= 2/5` and the `upper` certificate `s` `9/16` for `<= 3/5`; `lower` `5/8` and `upper` `7/16` are rejected `inequality-fails` at `s`, `lower` `7/16` for `>= 1/2` `threshold-missed` at `s`, and `lower` `7/16` without `ranking` `malformed` at `ranking`. Over `W` with no fairness set, maximum `1/2`: `upper` `s` `3/5` is accepted for `<= 3/5` (every action satisfies `Q_a(y) <= y`); `lower` `s` 1 with policy `wait` and rank 1 is rejected `rank-broken` at `s`, and `lower` `7/16` with policy `go` and rank 1 is accepted as the lower side of a value-bounds proof. | Test (TC-2110) |
| FR-412-AC-15 | `Retry`'s undelivered state has the action `try`, delivering with `1/2` and otherwise staying; delivered is terminal. For `quantile 0.5 of (steps from holds(true) until holds(delivered) within 4) <= 2` under its workload, the `ratio` certificate whose numerator has values `3/4`, `1/2`, 0, 0 at the active undelivered states of amounts 0, 1, 2 and above the bound, ranked by the remaining window 4, 3, 2, 1, and whose denominator is 1 there, is accepted. With `try` split into `fast` (`1/2`) and `slow` (`1/4`), for `quantile 0.4 of (…) <= 2` under every scheduler the `exact` shifted-reward certificate with values `3/80`, `−3/20`, `−2/5`, `−2/5` is accepted with side `AtLeast`; for `quantile 0.5` its values `−1/16`, `−1/4`, `−1/2`, `−1/2` are rejected `threshold-missed` at the initial state, and the level-0.4 certificate with `3/80` replaced by `fast`'s `9/40` is rejected `inequality-fails` there. `mean of fraction holds(delivered) over [0, 2]` under the workload has the exact initial value `5/12`. | Test (TC-2110) |
| FR-412-AC-16 | Over FR-412-AC-15's `fast`/`slow` model at level 0.5 (minimum `Pr(M <= 2) = 7/16`, shifted minimum `−1/16`), a `lower` certificate that names only the initial state, at value 0, is rejected `inequality-fails` there: each unnamed successor counts as `−1/2`, so `Q_fast = Q_slow = −1/2 < 0`. Counted as 0 instead, it would pass (`Q_fast = 1/4`, `Q_slow = 1/8`) and prove a claim that fails. | Test (TC-2110) |

## Dependencies

- [FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)
  produces the certificate and defines the product.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines the structural
  identity of the obligation.
- [FR-331](FR-331-backend-provider-envelope.md) carries the certificate in
  the result.

## References

- FSL ADR-028 CE-1 to CE-7, FS-9.
- K. Chatterjee, T. Quatmann et al., fixed-point certificates for MDP model
  checking, TACAS 2025; F. Funke, S. Jantsch and C. Baier, TACAS 2020.
- C. Baier, J. Klein, S. Klüppelholz and S. Märcker, conditional
  probabilities in Markovian models, TACAS 2014: the reset process of a
  `conditional` certificate.
- Ticket: STD-137 (paired with QSL-371); the fair-minimum, value-bounds and
  finite-horizon measure conditions: QSL-2158.
