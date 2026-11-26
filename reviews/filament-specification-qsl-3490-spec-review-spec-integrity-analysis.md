---
id: SR-85914
title: "QSL-3490 spec review of the FSpec timed run section and digital-clock admission rule (filament-specification PR 861)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification@4e6f89d0ef7bda694064676bef3c86cbe441ed2b; spec/functional/temporal/FR-416, FR-417, FR-421, FR-161, FR-361, FR-753; spec/functional/analysis/FR-407, FR-408, FR-420, FR-641, FR-712, FR-751, FR-752; spec/test-cases/TC-369, TC-374; spec/spec.md"
review_set: subset
date: 2026-11-26
---

## Summary

Ticket: QSL-3490 (parent QSL-3489). Independent soundness review of filament-specification#861 at head 4e6f89d0ef7bda694064676bef3c86cbe441ed2b, read against DESIGN.md C1-C12, the QSL-3490 comments and the arch rulings of 2026-11-25 (Q1 adopt FR-921 amendment as data, Q2 FR-407 infinite, Q3 FR-420 to the exact side, Q4 and Q5 as written, mixed delays and the minimum side with a reachable time-lock leave digital).

Checked as a mathematician against KNPS 2006 (FMSD 29, author PDF fetched): Lemma 17 (<=, =, >= survive every threshold), Definition 8 (almost-sure divergent adversaries, for every state), Proposition 23 (randomized integral adversary per threshold), Theorem 24 (location reachability, min and max), Theorem 27 (expected cost, uniform rate) state what the rows say, except as FSL FND-007 notes. Arguments A (FR-243 (a)-(e): threshold frac(r) for future upper bounds, just below it for lower-bounded and past cases; operand restrictions make one violating position suffice), B (round-up keeps t <= D, round-down t < D, from position 0), D (deadlock at capped clocks; closed guards from an integer valuation are met by a whole delay) and G (all given: every pre-tail stamp whole) held against my attempts with small closed models. Argument M failed (counterexample in FND-001). Hand vectors recomputed: Gap (dense max 1, integer 0), later-position (dense min 0, integer 1/2), Split (dense min 0, integer 1/2), Burst ((1+m)/(2+m), whole stutters <= 2/3). Changed values recomputed: FSpec FR-420-AC-7 / TC-374 ST-07 and FSL FR-254-AC-4 / TC-709 step 4 (position at 1/2 ms in B: estimate 1, rejected; fraction unsupported); FR-416-AC-12 to AC-16 hold. Almost-sure divergence agrees with FR-204-AC-16 / FR-924-AC-4 (min = max = 1; under sure divergence no scheduler is admitted) and with ADR-028 13a's probability-1 reading, apart from FSL FND-002. C9 untouched (ADR-026 CK-4, EZ-4, RT-8 not in the diff). Not verified: Alur-Dill 1994, Behrmann et al. 2006, KNSS 2002 and Henzinger-Manna-Pnueli 1992 statements (not fetched); FR-1051 (#4035, not in tree); no build or test run; quoin sub-analysis tools not run, review done by hand.

## Verdict

CHANGES REQUESTED. One high finding (the mixed delay class admits an unsound digital value) and the arch rulings of 2026-11-25 (Q1, Q3, mixed and time-lock) leave text to change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The `mixed` delay class admits digital clocks for a race of a given delay against a free one, which is unsound; arch has ruled mixed subjects leave digital, so the class and its 'all' rows must go. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:100-103,115-120 |
| FND-002 | medium | A reachable time-lock removes both extrema from digital clocks; the arch ruling names the minimum side; the text must state per side what leaves, and the two definitions of 'no time-lock' differ in wording. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:104-108 |
| FND-003 | medium | C6 residue: FR-416 Verdicts keeps its own open-ended digitisation list ('claims the provider's requirement shows it preserves') beside the new admission table, so two rules decide what digital clocks may settle. | spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md:201-205 |
| FND-004 | medium | C1 residue outside the diff: FR-725 says a timed-product certificate attests the value over time-divergent behaviours, but FR-416 now keeps time-locked runs in probabilistic items. | spec/objects/interfaces/FR-725-timed-product-probability-certificate.md:31-36 |
| FND-005 | medium | Arch Q3: a bounded-delay state whose windows hold no mass is a time-lock (exact 0, FR-753-AC-6); FR-420 'The race', FR-420-AC-4 and TC-374 ST-04 still settle it not-stochastic. | spec/functional/analysis/FR-420-sample-stochastic-timed-models.md:100-105,311 |
| FND-006 | medium | Arch Q1 adopted sharing the declared route table and the locked-run value mapping as data; FR-751's closed list of what may be shared does not include them, so FR-751 forbids what was adopted. | spec/functional/analysis/FR-751-check-a-timed-product-certificate-independently.md:35-42 |
| FND-007 | medium | Lost capability: a reward-weighted fraction over [0, h] read on origin is preserved by ADR-038's own argument B in class free, yet it is declined there; no vector shows the weighted-by-delay forms unsound either. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:120 |
| FND-008 | low | TC-374's purpose still says exact checking is through digital clocks; FR-421 is now digital or regions (DG-08, DG-09 are region vectors). | spec/test-cases/TC-374-stochastic-and-probabilistic-timed.md:22 |

## Coverage

- FR-416 (examined): The timed run section: positions, delays and strictness, idle tail, time-locks, almost-sure divergence, how claims read a time-lock, outcome order; ACs 12-16.
- FR-416-AC-12 (examined): Lock under One: eventually[0,2] B exact 1 refuted; eventually[3/4,2] B exact 0 proved; always[3/4,2] A exact 1 refuted.
- FR-416-AC-13 (examined): always holds(c.phase = A) over Lock proved; time-lock-freedom refuted.
- FR-416-AC-14 (examined): Retx variant, no guard/reset on send: eventually[0,4] delivered proved exact 1 (almost-sure divergence).
- FR-416-AC-15 (examined): Idle: eventually[3,5] phase=1 refuted on max 1 and on min 0.
- FR-416-AC-16 (examined): Initial local time-lock: no-admitted-behaviour; probabilistic exact 0 proved; no-instance first.
- FR-421 (examined): Closed and strict conditions: subject classes, time-lock condition, claim table, region fallback; Digital clocks paragraph; Time divergence.
- FR-421-AC-8 (examined): Split: eventually[0,3] Done >= 0.25 never proved, minimum 0.
- FR-421-AC-9 (examined): Burst: mean of fraction over [0,1] <= 0.7 never proved, (1+m)/(2+m).
- FR-420-AC-7 (examined): Lock sample ends at the lock: eventually estimate 1 rejected; fraction unsupported time-locked-sample.
- FR-420-AC-11 (examined): Long-run fraction reaching a time-lock settles unsupported time-locked-sample.
- FR-417 (examined): Time-locks defer to FR-416.
- FR-161 (examined): Time divergence, idle tail, vacuity point at FR-416.
- FR-361 (examined): Bounded-delay terminal state is a time-lock read per FR-416.
- FR-712 (examined): State-graph items never settle no-admitted-behaviour.
- FR-751 (examined): Shared run and wire types; item 1 route.
- FR-752 (examined): Region route applies to every claim FR-421 does not read digitally.
- FR-753 (examined): Almost-sure divergence; time-lock read per FR-416.
- FR-407 (context_only): Idle tail window; censored measure exceeds every finite threshold.
- FR-408 (examined): time-locked-sample only for fraction and long-run fraction.
- FR-641 (examined): Timed row over digital clocks or regions.
- TC-369 (examined): TS-16 to TS-20.
- TC-374 (examined): Split, Burst models; ST-07; DG-08, DG-09.
- FR-725 (context_only): Certificate attests the value over time-divergent behaviours.

## New findings (disposition pass 1)

Head checked: 7a42b7ba1e35739ea36be9c4408d610abc5b6818.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | high | Digital rows admit claims with a fairness set; no cited statement covers fairness, and a strong fairness set gives a wrong digital minimum on a closed, lock-free free subject. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:63,112-117 |
| FND-010 | medium | Arch ruling (2) on the mixed race: FR-421-AC-6 is a mixed subject and stays unedited, but it must be marked in the admission rule as pending the region route; FR-421's table says nothing about it. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:242 |
| FND-011 | low | Argument E's admission says 'deadline event' without 'on origin'; the rounding it relies on (argument B) keeps a deadline only from position 0. | spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:119-124 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
| FND-008 | fixed | - |
| FND-009 | fixed | - |
| FND-010 | fixed | - |
| FND-011 | fixed | - |
