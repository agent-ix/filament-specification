---
id: FR-752
title: "Certify strict-clock and every-scheduler delay verdicts"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-725
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-751
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-421
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-406
    type: depends_on
---

# FR-752: Certify strict-clock and every-scheduler delay verdicts

## Description

When a timed item's subject or claim has a strict clock constraint, the
provider SHALL prove it over the region process and certify it with
region-keyed values (FR-725), and the checker SHALL rebuild the region process
itself. When an every-scheduler claim's subject declares `discrete` delays,
the provider SHALL prove it over the race those delays define, with the
remaining choices the scheduler's, and the checker SHALL rebuild that race
itself. Neither case settles `unsupported`.

## Inputs

An FR-421 item and its FR-725 certificate.

## Outputs

FR-751's result.

## Behavior

### Strict constraints

- The region route SHALL apply when any guard, time invariant, timed interval
  or elapsed-threshold comparison is strict, and to every bounded-window
  formula FR-421 "Closed and strict conditions" reads through clock regions
  (FR-751). Its states are the
  region keys of FR-725; delay moves go from a region to its time successor;
  a discrete move applies a guard to the region, which decides a diagonal-free
  constraint exactly, and resets set the integer part and fraction to zero.
- The region process SHALL give the exact minimum and maximum probability of
  every admitted form on the dense semantics. A supremum or infimum that no
  dense scheduler attains is the value the region process attains, and the
  certificate states it.
- An expected-elapsed form on the region route SHALL use boundary-region keys,
  each delay edge carrying the exact elapsed time between its corners; the
  infimum and supremum are the boundary-region minimum and maximum.
- The checker SHALL refuse a `digital` certificate for a subject or claim that
  has a strict constraint as an objective mismatch, and a `region` certificate
  where every constraint is non-strict the same way: the route is a function
  of the model and claim.

### Delay distributions under every scheduler

- Under every scheduler, a `discrete` delay SHALL apply as under a workload:
  each racing identity draws its delay from its distribution conditioned on
  its window, and the least delay wins.
- What a workload's weights decide, the scheduler decides: an identity with no
  `delay` declaration has a delay the scheduler chooses, after the given
  delays are drawn (FR-420's free delays), and when two or more identities tie
  at the least delay, the scheduler chooses the winner, after the draw. Each
  action at a racing state is one function from drawn vectors to a choice of
  every free delay and of the tie's winner; the action's distribution is the
  sum over drawn vectors of their probability times the race outcome. The
  reason is that without a workload no weight orders the tie, and a choice
  among enabled identities is exactly what FR-406 gives the scheduler.
- The checker SHALL rebuild each race's drawn vectors and their exact
  probabilities from the `delay_distribution` nodes, and every action above,
  and SHALL apply FR-412's minimum or maximum over them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-752-AC-1 | TC-374's `Retx` with `send` guarded `x > 1 ms`: `Deadline` at 0.99 over every scheduler settles `proved`, `exact-value{99/100}` at equality, with a `region` certificate the checker accepts; the maximum is `999/1000` (three attempts, each just after 1 ms past the last, fit in 4 ms), and `probability <= 0.999` settles `proved` at equality with a `region` certificate the checker accepts. | Test (TC-1769) |
| FR-752-AC-2 | For the claims of AC-1, a `digital` certificate is refused as an objective mismatch; a `region` `exact` certificate of the maximum whose initial value is `9999/10000`, the value of the non-strict guard, is rejected `certificate-rejected`, rule `inequality-fails`. | Test (TC-1769) |
| FR-752-AC-3 | `Retx` with `delay Msg::send ~ discrete { 1 ms: 1, 2 ms: 1 }`: `Deadline` over every scheduler has minimum and maximum `159129/160000`, settles `proved`, and its `digital` certificate is accepted; at threshold 0.995 the item settles `refuted`, and a certificate with that minimum's values bound to the `>= 0.995` claim is rejected `threshold-missed` at the initial state. | Test (TC-1769) |
| FR-752-AC-4 | For AC-3's claim, a certificate whose values are those of the reading that ignores the distribution (initial value `99/100`) is rejected `certificate-rejected`, rule `inequality-fails`. | Test (TC-1769) |
| FR-752-AC-5 | A model with `phase` `Idle` or `Done`, a clock `x` and a Boolean `b`, where `A` (guarded `Idle`, sets `Done`) declares `delay A ~ discrete { 2 ms: 1 }` and `B` (guarded `Idle and x >= 2 ms` under the invariant `x <= 3 ms` while `Idle`, sets `Done` and `b`) declares no delay: under every scheduler the maximum of `eventually[0 ms, 5 ms] holds(b)` is `1` (`B` at 2 ms wins the tie by the scheduler's choice) and the minimum `0`, so `probability <= 0.5` settles `refuted`; under a workload weighting `A` and `B` equally the maximum is `1/2`. | Test (TC-1769) |

## Dependencies

- [FR-725](../../objects/interfaces/FR-725-timed-product-probability-certificate.md),
  [FR-751](FR-751-check-a-timed-product-certificate-independently.md).
- [FR-421](../temporal/FR-421-check-probabilistic-timed-claims-exactly.md)
  defines the readings; [FR-420](FR-420-sample-stochastic-timed-models.md) the
  race.

## References

- M. Kwiatkowska, G. Norman, R. Segala and J. Sproston, "Automatic
  verification of real-time systems with discrete probability
  distributions", 2002.
- M. Jurdziński, M. Kwiatkowska, G. Norman and A. Trivedi,
  "Concavely-priced probabilistic timed automata", 2009.
