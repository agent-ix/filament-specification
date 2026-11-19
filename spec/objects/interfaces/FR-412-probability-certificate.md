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
| conditional | for `conditional`: JSON object tagged by `mode`, `workload`, `every-scheduler` or `vacuous`; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.conditional` |


`ProbabilityCertificate{identity, objective, kind, values, ranking, policy,
components, conditional}`:

| Member | Content |
| --- | --- |
| `identity` | The item's obligation identity, the structural `{node_id, model, ordinal}` of FR-418-AC-9, that binds the proof to the exact subject and claim it proves. |
| `objective` | The target or decided states, the reward, the optimisation, and the claim's fairness set when it states one. The optimisation is `minimum` or `maximum` over every scheduler; `workload` under a workload that leaves no choice; or `workload-minimum` or `workload-maximum` under a workload over a timed subject with free delays, the minimum or maximum over those delays' choices. |
| `kind` | `lower`, `upper`, `exact`, `long-run` or `conditional`. |
| `values` | One exact rational per product state the certificate names (a dyadic value is an exact rational), keyed by product state key (FR-411). |
| `ranking` | For `lower` and `exact`: one natural number per named state. |
| `policy` | For a lower bound on a maximum or an upper bound on a minimum, including `workload-maximum` and `workload-minimum`: one choice per named state, a memoryless deterministic scheduler. |
| `components` | For `long-run`: per component, its value `ρ_C` and a gain–bias pair. |
| `conditional` | For `conditional`, the certificate of a conditioned reading (FR-407 "Conditioned values"): `{mode: "workload", numerator, denominator}`, `{mode: "every-scheduler", post, reset}` or `{mode: "vacuous"}`, each part an `exact` certificate of this form. |

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
  rank; then `x` is at most the value. An unnamed successor counts as value 0.
- **Exact**: both conditions, with equality.
- **Finite horizon**: the ranking is the remaining horizon, which the checker
  reads from the monitor state.
- **Expected reward**: the same conditions with the reward added to `F`, and
  the states whose value exceeds every finite threshold recomputed by graph.
- **Long-run**: per component, the gain–bias pair satisfies the multichain
  optimality equations for the reward `R · ([P] − ρ_C)` with gain 0, as
  equalities under the component's policy and as inequalities over every
  action, plus an `exact`, `lower` or `upper` certificate for the expected
  component value.
- **Conditional**: for a conditioned reading (FR-407 "Conditioned values").
  The checker rebuilds the activating states, the states from which
  activation is reachable and, under every scheduler, the reset process
  itself, by its own graph code; no part of the certificate supplies them.
  - `workload`: `numerator` and `denominator` are `exact` certificates over
    the same product and the same workload, for `Pr(E and act)` (or
    `Ex(X · [act])`) and for `Pr(act)`, the reachability of the first
    activating states. Each passes its own conditions. At each initial
    state the certified value is the numerator's value over the
    denominator's; where the denominator is 0 the checker confirms by its
    own graph search that no activating state is reachable, and that
    instance holds vacuously.
  - `every-scheduler`: `post` is an `exact` certificate of the form's
    optimal value from every first activating state, over the product read
    from activation on; `reset` is an `exact` reachability certificate over
    the reset process the checker rebuilt, whose terminal values are
    `post`'s values at the first activating states and whose steps back
    lead to each instance's own initial state. The certified value is
    `reset`'s value at each initial state from which activation is
    reachable; at any other initial state the checker confirms vacuity by
    its own graph search.
  - `vacuous`: for a reading in which every instance is vacuous; it carries
    no parts and an empty `values`, and the checker accepts it exactly when
    its own graph search finds no activating state reachable from any
    initial state, and under every scheduler with a fairness set none from which
    a fair continuation exists.

The checker accepts a certificate that meets its conditions and puts the
initial state's value on the claim's side of the threshold: `lower` for a
proof of `>= θ` or a refutation of `<= θ`; `upper` for a proof of `<= θ` or a
refutation of `>= θ`; `exact` and `conditional` for either. A `conditional`
certificate is on the claim's side when its certified value is, and a
`vacuous` one is on the side of a proof. It refuses a certificate whose
`identity` differs from the item's with `invalid_runtime_input`/`conflicting-identity`.
A rejected certificate settles the item `inconclusive`,
`certificate-rejected{rule, at}`, `at` the complete product state key where
it failed, preserving the same components as the certificate's keys
(FR-411, FR-331): rule `inequality-fails` where an
upper, lower, exact or long-run condition above fails; at an initial state
where a `workload` denominator is 0 although activation is reachable from
it, or where a `vacuous` certificate is offered and an activating state is
reachable; for a `conditional` certificate, the first failing part, checked
in the order `numerator`, `denominator` or `post`, `reset`, with that
part's own rule and locus; `rank-missing` at a
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
- **Absence** — a `lower` or `exact` certificate without `ranking`, a lower
  bound on a maximum without `policy`, or a `long-run` certificate without
  `components`, a `conditional` certificate without `conditional` or without
  a part its mode requires (`numerator`, `denominator`, `post`, `reset`), or
  a part whose `kind` is not `exact`, is rejected,
  `certificate-rejected{rule, at}` with rule `malformed` at `{member}`, the
  missing or wrong member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-412-AC-1 | The checker accepts the `Deliver` `exact` certificate (TC-363) (values `24/25`, `4/5`, 1 at delivered states, 0 at the two-loss state) after checking the minimum over both actions at each live state. | Test (TC-363) |
| FR-412-AC-2 | The checker accepts the `LongRun` `long-run` certificate (TC-362) (gain `1800/1801`, bias 0 at `up` and `−2000/1801` at `down`) by exact equality. | Test (TC-362) |
| FR-412-AC-3 | A certificate that violates one inequality settles `inconclusive`, `certificate-rejected` with rule `inequality-fails` at that state; a `lower` certificate with a missing rank, `rank-missing` at the unranked state; one with a rank that does not decrease, `rank-broken` at that state; one whose initial value is on the wrong side of the threshold, `threshold-missed` at the initial state; and a `lower` certificate without `ranking`, `malformed` at member `ranking`. | Test (TC-363) |
| FR-412-AC-4 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-363) |
| FR-412-AC-5 | A `lower` certificate is checked over its named states and their successors only, with each unnamed successor at value 0. | Test (TC-363) |
| FR-412-AC-6 | The checker recomputes every qualitative set and component itself and reads no other output of the provider. | Test (TC-363) |
| FR-412-AC-7 | The checker accepts the `FairTerminates` `exact` certificate, whose objective names the fairness set, after recomputing that no fair end component exists. | Test (TC-363) |
| FR-412-AC-8 | Two named product states with the same model state but different monitor states or accumulators retain distinct keys in `values`, `ranking`, `policy`, component `bias` and state-located rejections after serialization and reading. Intermediate states that differ only in their pending action or draw also remain distinct; replacing either complete key with the model state key alone fails the comparison. | Test (TC-363) |
| FR-412-AC-9 | Over FR-407-AC-18's model, the `every-scheduler` `conditional` certificate whose `post` values are 1 and `1/2` at the two first activating states and whose `reset` value at `s0` is `1/2` is accepted for `>= rational(1, 2)` and fails `threshold-missed` at `>= rational(3, 5)`; with `reset`'s value at `s0` set to the unconditioned `1/4` it is rejected `inequality-fails` at `s0`, and with `post`'s `1/2` set to `3/5` it is rejected `inequality-fails` at that state. The `workload` certificate with numerator `3/8` and denominator `5/8` is accepted for `>= rational(3, 5)`; with denominator 1 it is rejected `inequality-fails`. | Test (TC-2061) |
| FR-412-AC-10 | A `vacuous` certificate is accepted over FR-407-AC-18's variant where `g` is unreachable and rejected `inequality-fails` at the initial state over the model itself; a `conditional` certificate without `conditional` is rejected `malformed` at `conditional`, and a `workload` one without `denominator`, `malformed` at `denominator`. Two states that differ only in `instance` or only in `activation` keep distinct keys after serialization and reading. | Test (TC-2061) |

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
- Ticket: STD-137 (paired with QSL-371).
