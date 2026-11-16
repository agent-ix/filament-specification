---
id: FR-751
title: "Check a timed-product certificate independently"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-725
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-412
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-421
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-640
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
---

# FR-751: Check a timed-product certificate independently

## Description

When an FR-725 timed-product certificate is checked, the checker SHALL
rebuild the timed product from the checked package and the claim alone, with
its own clock, race and graph code and no part of the provider that produced
the certificate, and SHALL accept the certificate only when FR-412's
conditions hold on that rebuilt product. A `proved` result over a timed
subject SHALL count only after such a checker accepts its certificate.

The checker exists so that a reviewer need not trust the provider's timed
reasoning, which is where the subtle errors live: a digital reading of a
strict guard, a distribution dropped from a race, a Zeno cycle counted as a
behaviour, an elapsed clock that keeps running after its measure completed.
Each of these changes the value; the checker catches each by deriving the
object itself rather than reading it from the certificate.

## Inputs

An FR-725 certificate, the checked package (FR-322) with its timed-model and
`delay_distribution` nodes (FR-640), the item's claim identity, and the
request's budgets.

## Outputs

FR-412's accepted, rejected or stopped result.

## Behavior

### What the checker re-derives

The checker SHALL compute each of the following from the package and the
claim, and compare it with the certificate only where the certificate states
it:

1. **The route.** Digital when every atomic clock constraint of every guard
   and time invariant, every timed interval of the claim and every comparison
   of an elapsed measure with its threshold is non-strict; region otherwise
   (FR-752). The strict comparison `M < c` that the `>=` quantile transform
   introduces counts as strict.
2. **The scale and caps.** The least common denominator of every time
   constant in the model's unit, including resets, delay supports, the claim's
   bounds and thresholds; each clock's cap from its largest scaled compared
   constant.
3. **The moves.** Delay moves with their admissibility under the time
   invariants and urgency, discrete moves by FR-181's timed step, TS-5 idle
   tails, and on the region route each region's time successor.
4. **The races.** At every state where an identity with a `discrete` delay
   races, each racing identity's distribution conditioned on its window, the
   exact probability of each drawn vector, and the choices that remain to the
   scheduler (FR-752).
5. **Time divergence.** The maximal end components whose actions all take
   zero time, their exits, and the time-locks (FR-753).
6. **The measure.** The deadline or elapsed clock, the activation of an
   elapsed measure, its completion and censoring, and the quantile reward
   (FR-753).
7. **The qualitative sets.** Targets, decided states, `Prob0` and `Prob1`,
   as FR-412 requires.

### What the checker does not trust

The checker SHALL NOT read, as a fact about the subject, any of: the
certificate's stated route, unit, scale or form (compared only); the set of
keys the certificate names (every reachable state of the rebuilt product is
required for `upper` and `exact`); any transition, race outcome or
probability; any component, exit or time-lock; any activation or phase; a
product, value or classification the provider computed; or the provider's
claim that a verdict holds on the dense semantics. A key in the certificate
that the rebuilt product does not reach SHALL be rejected `malformed` at
`values`; a reachable state the certificate omits SHALL be rejected
`malformed` at `values` for an `upper` or `exact` certificate.

### Order of checks

The checker SHALL refuse an identity mismatch, then check the member set
(FR-725), then compare the stated route, unit, scale and form, then rebuild
the product, then apply FR-412's conditions with FR-752's and FR-753's
timed rules, then the threshold. The first failure in that order, at the
first state in canonical key order, is the result.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-751-AC-1 | FR-725-AC-1's certificate with one value altered is rejected `certificate-rejected`, rule `inequality-fails`, at the first failing state; with its identity changed it is refused `invalid_runtime_input`/`conflicting-identity` before the product is rebuilt. | Test (TC-1768) |
| FR-751-AC-2 | The same certificate with `route` changed to `region`, or with `scale` changed to `2`, is refused as an objective mismatch, `invalid_runtime_input`/`invalid-value`; a key with the cap of `x` changed from 2 to 3 is rejected `malformed` at `values`. | Test (TC-1768) |
| FR-751-AC-3 | A certificate that omits one reachable state is rejected `malformed` at `values`; one that adds a key the rebuilt product does not reach is rejected the same way. | Test (TC-1768) |
| FR-751-AC-4 | The checker, run in a process that contains no part of the provider, accepts the certificates of FR-752-AC-1, FR-752-AC-3, FR-753-AC-1 and FR-753-AC-4 from their bytes and the package, and rejects each with one value altered. | Test (TC-1768) |

## Dependencies

- [FR-725](../../objects/interfaces/FR-725-timed-product-probability-certificate.md)
  defines the certificate.
- [FR-412](../../objects/interfaces/FR-412-probability-certificate.md)
  defines the conditions and rejections.
- [FR-752](FR-752-certify-region-and-every-scheduler-race-verdicts.md) and
  [FR-753](../temporal/FR-753-certify-time-divergent-and-elapsed-quantile-verdicts.md)
  define the timed rules.
