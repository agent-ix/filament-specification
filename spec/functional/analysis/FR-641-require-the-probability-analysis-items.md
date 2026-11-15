---
id: FR-641
title: "Require the probability analysis items and their evidence"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-407
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-408
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-411
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-412
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-413
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-421
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-640
    type: depends_on
---

# FR-641: Require the probability analysis items and their evidence

## Description

An implementation that claims the probabilistic part of the standard SHALL
accept every `probabilistic-satisfaction` item below in the FR-331 request
over a checked package that carries its model's probabilistic declarations
(FR-640), and SHALL return for each the result members and evidence listed.
No item class, method or certificate kind below is optional for that claim.

## Inputs

A checked package with a probabilistic claim (FR-407) and its declarations
(FR-640); an FR-331 request naming the claim's item, its evidence kind and the
settings of that kind.

## Outputs

One FR-331 terminal record per item, carrying the result members below.

## Behavior

### Statistical items

A statistical item names `statistical` evidence and one of the four methods of
FR-408, spelled in the request as FR-331 spells them: `okamoto`, `sprt`,
`clopper-pearson` and `regenerative` with `min_cycles`, together with its
seed, `max_witnesses`, `workers` and `StatisticalLimits`, and over a timed
subject its grid exponent `q` and `max_sample_steps` (FR-420). Every one of the
four methods is required. The exact binomial method (`clopper-pearson`) is
required beside the fixed-sample method because it holds its error bounds at
every true probability, inside the indifference region too, where the
fixed-sample and sequential tests state no error bound (FR-408); a user who
needs that guarantee has no substitute for it.

The result is a `measured` result (FR-409, FR-410): the verdict, the method,
the seed, per test its initial state, binding, counted samples, drawn samples,
estimate and interval (no interval for `sprt`), and up to `max_witnesses`
sampled witnesses, each of which replays to the sampled outcome it records.

### Exact items

An exact item names `exact` evidence and its `ExactProbLimits`. It is
required for every form of FR-407 and for both scheduler clauses:

| Class | Forms | Scheduler clause |
| --- | --- | --- |
| Finite horizon | probability bounds over bounded events, quantiles, means of fractions | workload; every scheduler, with or without a fairness set |
| Unbounded reachability | `eventually`, `until` and `always` over state predicates | workload; every scheduler, with or without a fairness set |
| Expected reward | `expected accumulate R until holds(B)` | workload; every scheduler, with or without a fairness set |
| Long-run | `long_run_fraction`, unweighted and weighted | workload; every scheduler, with or without a fairness set |
| Timed | FR-421's forms through digital clocks, `expected elapsed until holds(B)` included | workload; every scheduler |

The result of a proof is `proved` with an `exact-value` or `value-bounds`
proof basis and one FR-412 certificate of kind `exact`, `lower`, `upper` or
`long-run`, which an FR-412 checker, run without the provider, accepts before
the item settles `proved`. Under every scheduler, the certificate's `policy`
names the memoryless scheduler that attains the bound where one is required
(FR-412). The result of a refutation is `refuted` with an FR-413
counterexample that replays to the refuting probability before the item
settles `refuted`.

### Wording of an unbounded exact claim

An unbounded exact claim is written under the infinite-trace profile as
`probability >= θ` or `<= θ` over exactly one of `[eventually holds(P)]`,
`[holds(A) until holds(B)]` and `[always holds(P)]`, or as
`expected accumulate R until holds(B) <= c` or `>= c`; over a timed subject
also `expected elapsed until holds(B) <= c` or `>= c`. There is no other
spelling: an interval `[a, *]` in such an event refuses `unbounded-event`, and
the same operator under a bounded profile refuses at the temporal frontend
(FR-407). These forms are exact-only: a statistical item for them settles
`unsupported`, `exact-only-form` (FR-408), whatever confidence parameters the
claim states.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-641-AC-1 | For TC-351's `P95` under `Steady` with its confidence parameters, one statistical item for each of `okamoto`, `sprt`, `clopper-pearson` and `regenerative` (the last for `LongRun`) is accepted and returns a `measured` result naming its method and seed with the members above; the same request and seed return the same result. | Test (TC-1564) |
| FR-641-AC-2 | One exact item for each class row of the table, under a workload and under every scheduler, is accepted and settles `proved` with a certificate that a checker run in a separate process without the provider accepts, or `refuted` with a counterexample that replays; the certificates seen across the items include each of the kinds `exact`, `lower`, `upper` and `long-run`. | Test (TC-1564) |
| FR-641-AC-3 | `probability >= 0.99 [eventually holds(p.done)]` under the infinite-trace profile is admitted as an unbounded exact claim and a statistical item for it settles `unsupported`, `exact-only-form`; `[eventually[0, *] holds(p.done)]` refuses `unbounded-event`. | Test (TC-1564) |

## Dependencies

- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the items, their settings and their records.
- [FR-408](FR-408-settle-probabilistic-claims-statistically.md),
  [FR-411](FR-411-settle-probabilistic-claims-exactly.md),
  [FR-420](FR-420-sample-stochastic-timed-models.md) and
  [FR-421](../temporal/FR-421-check-probabilistic-timed-claims-exactly.md)
  define the methods and engines;
  [FR-412](../../objects/interfaces/FR-412-probability-certificate.md) and
  [FR-413](../../objects/interfaces/FR-413-probabilistic-counterexample.md)
  the evidence.
- [FR-640](../../objects/interfaces/FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)
  carries the declarations the items read.
