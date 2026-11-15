---
id: FR-652
title: "Check hybrid enclosure and validated-witness certificates"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-193
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-481
    type: references
---

# FR-652: Check hybrid enclosure and validated-witness certificates

## Description

A hybrid provider MAY return, with a `proved` entry or with a trace, a
certificate stated in the names of the source `hybrid-decl`. When the reader
that holds the checked hybrid model receives one, it SHALL check it in exact
rational interval arithmetic, and SHALL use an accepted certificate to settle
the proof `certified` or to refute through a mode whose flow is not
polynomial.

## Inputs

A hybrid item, its checked hybrid model and claim (FR-481), the provider's
FR-193 result entry, and the entry's optional `certificate` member.

## Outputs

The certificate outcome `accepted` or `rejected{rule, at}`, and the item's
settlement as below.

## Behavior

### The provider contract

A hybrid provider is a registered backend whose descriptor's `dynamics` holds
`hybrid` (FR-193 "Routing a hybrid item"). It receives one FR-331 request
item per hybrid claim and returns one FR-193 result entry per item. Its
entry MAY carry `certificate`, one of the three forms below; an entry without
one is settled exactly as FR-193 states. The certificate is checked by the
reader, never by the provider, and a provider's word about its own soundness
changes no settlement.

### Certificate forms

All boxes are maps from each continuous variable of the declaration to an
FR-331 `Interval` of exact rationals; a box keyed differently from the
declaration's variables, or with `lower > upper`, is rejected at its pointer.

1. **`inductive-enclosure`**, for an `inductive-solver` proof: one box `E_m`
   per mode `m`. The reader SHALL accept it when, evaluated by interval
   arithmetic over the boxes, (a) every initial entry's condition and mode
   invariant implies membership in that mode's box; (b) for every mode and
   every face of `E_m`, either the mode invariant is definitely false just
   beyond the face (the face lies on the invariant's boundary) or the flow's
   interval evaluation on the face points into `E_m` (the outward component
   is definitely `<= 0` on an upper face and `>= 0` on a lower face); (c) for every transition
   from `m` to `n`, the reset's image of `E_m` intersected with the guard lies
   in `E_n`; and (d) no state in any `E_m` is a violating state of the claim,
   the claim being an invariant `always holds(e)` whose `e` is definitely true
   over every `E_m`.
2. **`flowpipe`**, for a `bounded-solver` proof: an ordered list of steps,
   each `{mode, time, box, apriori, step}`. The reader SHALL accept it when
   the first step's box contains every initial state; for each step the
   a-priori box `B` satisfies the Picard condition `box + [0, step] * f(B)
   ⊆ B` and the next step's box contains `box + step * f(B)` (or, at a
   jump, the reset's image of the guarded part of `B`); every transition
   whose guard can hold on some `B` is followed; the steps cover the proof's
   `horizon` and `jumps` with the `exhaustive` dimensions FR-193 "Proof
   coverage" requires; and the claim's atoms are definitely satisfied over
   every `B` within the claim region.
3. **`validated-witness`**, with a `HybridCounterexample` or a
   `solver-inconclusive` trace: a point start state and, for each dwell, a
   list of Picard steps as in form 2 whose boxes enclose the unique solution
   from that point, each flow's conditional (`if`) definitely taking one
   branch over each `B` so the solution is unique, each jump's guard
   definitely true over the box at which the jump is taken, and, at the
   decisive instant, the violated atom definitely false (or an expression
   definitely without value) over the enclosing box.

### Settlement

- A `proved` entry whose certificate is accepted SHALL carry the FR-331
  certification `certified`, whatever provider produced it. A rejected
  certificate SHALL leave the entry as FR-193 settles it without one (a
  first-party proof `uncertified`, a plugin's `trusted`) and SHALL record
  `certificate-rejected{rule, at}` beside it; it never turns a proof into a
  refutation.
- A trace through a mode whose flow is not polynomial (FR-193 "Trace replay"),
  which exact replay cannot decide, SHALL settle `refuted` when its
  `validated-witness` is accepted, with the trace as its
  `HybridCounterexample`; when the witness is absent or rejected, the item
  settles as FR-193 states (static replay only, no refutation).
- A certificate form that does not match its entry (a `flowpipe` on an
  `inductive-solver` proof, a `validated-witness` on a `proved` entry) SHALL
  refuse `invalid_result`/`invalid-value` at `/certificate`.
- Checking is bounded by the item's work limit; a check stopped by it settles
  the certificate `rejected{rule: "work-limit"}`, never `accepted`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-652-AC-1 | Over FR-193's `Braking` without its variable `t`, an `inductive-solver` proof of `always holds(h.v < 30)` with boxes `Moving: v [0, 20], a [1, 2]` and `Stopped: v [0, 0], a [1, 2]` is accepted (the `v = 0` face of `Moving` lies on its invariant's boundary) and settles `certified`; with `Moving: v [0, 19]` it is rejected naming rule (a), since the initial state `v = 20` lies outside, and the proof stays `uncertified` with `certificate-rejected` recorded. | Test (TC-1605) |
| FR-652-AC-2 | A `flowpipe` certificate whose step violates the Picard condition is rejected naming that step, and the `bounded-solver` proof stays `uncertified` with `certificate-rejected` recorded; a well-formed one covering horizon 10 settles `certified`. | Test (TC-1605) |
| FR-652-AC-3 | In a mode with flow `v' = v * v / (1 + v * v)`, which is not polynomial, a point trace with an accepted `validated-witness` reaching `v > 30` against `always [0, 12] holds(h.v < 30)` settles `refuted`; without the witness, or with a witness whose last box straddles `v = 30`, it stays `inconclusive`. | Test (TC-1605) |
| FR-652-AC-4 | A `flowpipe` certificate on an `inductive-solver` entry refuses `invalid_result`/`invalid-value` at `/certificate`; a box missing variable `a` is rejected at its pointer; a check stopped by the work limit is `rejected{rule: work-limit}`. | Test (TC-1605) |

## Dependencies

- [FR-193](FR-193-analyze-continuous-hybrid-models.md) owns hybrid models,
  claims, verdicts, traces and replay.
- FR-331 owns the result entry and its certification values.
