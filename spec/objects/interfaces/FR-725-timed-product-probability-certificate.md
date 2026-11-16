---
id: FR-725
title: "Timed-product probability certificate"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-412
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-421
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
---

# FR-725: Timed-product probability certificate

## Description

When an exact probabilistic provider proves a bound on a
`probabilistic-satisfaction` item over a timed subject (FR-421), it SHALL emit
an FR-412 `ProbabilityCertificate` whose objective also names the clock
reading the proof used, and whose state keys are complete timed product
states. The certificate attests one fact: the exact value it gives at the
initial state is the claim's value on the dense-time semantics of the subject,
over the time-divergent behaviours FR-416 admits. For an every-scheduler
claim that value is the minimum or maximum over those behaviours' schedulers;
under a workload it is the workload's value, or its minimum or maximum over
the delays no distribution gives.

The certificate states values only. It carries no clock route the checker
trusts, no region successor relation, no race outcome, no end component, no
time-lock and no activation set: the checker re-derives each of them from the
checked package and the claim (FR-751), so a certificate cannot make an
unsound reading pass by naming it.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| identity | JsonObject | 1..1 |  |
| objective | JsonObject | 1..1 |  |
| kind | String | 1..1 |  |
| values | JsonObject | 0..* |  |
| ranking | JsonObject | 0..* |  |
| policy | JsonObject | 0..* |  |

## Field sources

| Field | Derivation note |
|---|---|
| identity | FR-412 `identity` |
| objective | FR-412 `objective` plus the `timed` member below |
| kind | FR-412 `kind`: `lower`, `upper` or `exact` |
| values | FR-412 `values`, keyed by the timed product state keys below |
| ranking | FR-412 `ranking` |
| policy | FR-412 `policy` |

## Members

The members are FR-412's, with these additions and restrictions:

| Member | Content |
| --- | --- |
| `objective.timed` | `{route, unit, scale, form}`. `route` is `digital` or `region`. `unit` is the subject's time unit (FR-415). `scale` is the positive integer by which every time constant of the model, the claim and its `discrete` delays is multiplied to become an integer: their least common denominator in that unit. `form` names the claim form: `reach`, `until`, `always`, `deadline`, `bounded-window`, `expected-elapsed` or `elapsed-quantile`. |
| `kind` | `lower`, `upper` or `exact`. A `long-run` certificate is not a timed-product certificate. |
| `values`, `ranking`, `policy` | Keyed by the complete timed product state key below. |

The `route`, `unit`, `scale` and `form` are stated so that a reader of the
certificate can see which reading it claims; the checker computes each of them
itself and refuses a certificate whose stated member differs
(`invalid_runtime_input`/`invalid-value`, objective mismatch), before it reads
any value.

## Timed product state keys

A timed product state key is FR-412's complete product state key whose model
component is a timed state:

- **Digital.** The discrete state; per clock its integer value in scaled
  units, at most its cap; the deadline or elapsed clock of the claim when its
  form has one, with its bound; and the TS-5 tail mode (running, idle-ready
  or idle-delayed). A clock's cap is one plus the largest scaled constant it
  is compared with; a clock never compared has cap one.
- **Region.** The same, except that each clock, the deadline clock and the
  elapsed clock carry their integer part (at most the cap), whether the
  fractional part is zero, and the position of the fractional part in the
  order of the nonzero fractional parts of all clocks of the state (equal
  fractions share a position, positions are numbered from one without gaps).
  A clock at its cap carries a zero fraction.
- **Boundary region.** For an expected-elapsed form on the region route, the
  region key together with its corner: one integer per clock, in clock order,
  that is a vertex of the closure of the region the state approaches. Corners
  make the infimum and supremum of an expected elapsed time exact on regions,
  which a region alone does not give, because a region does not fix how long
  a delay inside it lasts.

An elapsed-quantile form's elapsed clock is reset when the measure activates,
stops when the measure completes or is censored, and is capped at the
measure's window plus one; its activation flag and phase are the monitor
component of the key. Two keys that differ in any of these members are
different states.

## Carriage

The certificate travels in the FR-331 terminal record of the item it proves,
as every FR-412 certificate does. It is not a checked-package member: a
checked package (FR-322) carries the inputs the checker re-derives the timed
product from (the timed-model nodes, the `delay_distribution` nodes of FR-640
and the probabilistic claim), never a proof about them.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — every member above.
- **Absence** — FR-412's rules apply. A certificate with any member beyond
  those above, such as a list of components, time-locks, races or regions,
  is rejected `certificate-rejected` with rule `malformed` at `{member}`, the
  extra member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-725-AC-1 | On TC-374's `Retx`, the exact proof of `Deadline` at 0.99 over every scheduler carries a certificate whose `objective.timed` is `{route: digital, unit: ms, scale: 1, form: deadline}`, whose keys give `x` the values 0 to 2 and the deadline clock 0 to 5, and whose initial value is `99/100`; a checker run without the provider accepts it. | Test (TC-1767) |
| FR-725-AC-2 | With `send`'s guard `x > 1 ms`, the proof of `Deadline` carries `route: region`; two of its keys that differ only in the fractional order of `x` and the deadline clock stay distinct after serialization and reading, and replacing a region key with the digital key of the same integer parts fails the comparison. | Test (TC-1767) |
| FR-725-AC-3 | A certificate with an added `components` or `time_locks` member is rejected `certificate-rejected`, rule `malformed`, at that member; a checked package emitted for a timed claim contains no certificate node. | Test (TC-1767) |

## Dependencies

- [FR-412](FR-412-probability-certificate.md) defines the certificate and its
  checker.
- [FR-421](../../functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md)
  defines the digital and region readings.
- [FR-751](../../functional/analysis/FR-751-check-a-timed-product-certificate-independently.md)
  defines what the checker re-derives.

## References

- M. Kwiatkowska, G. Norman, D. Parker and J. Sproston, "Performance analysis
  of probabilistic timed automata using digital clocks", 2006.
- M. Kwiatkowska, G. Norman, R. Segala and J. Sproston, "Automatic
  verification of real-time systems with discrete probability
  distributions", 2002 (the region MDP).
- M. Jurdziński, M. Kwiatkowska, G. Norman and A. Trivedi,
  "Concavely-priced probabilistic timed automata", 2009 (boundary regions).
