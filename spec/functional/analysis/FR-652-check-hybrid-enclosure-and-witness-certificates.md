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
   each `{mode, time, box, apriori, step}`, `time` an FR-331 `Interval` of
   seconds and `step` an FR-331 `Rational` number of seconds, both in the
   object form FR-193 "Solver result on the wire" requires of every exact
   number on the hybrid wire. The reader SHALL accept it when
   the first step's box contains every initial state; for each step the
   a-priori box `B` satisfies the Picard condition `box + [0, step] * f(B)
   ⊆ B` and the next step's box contains `box + step * f(B)` (or, at a
   jump, the reset's image of the guarded part of `B`); every transition
   whose guard can hold on some `B` is followed; the steps cover the proof's
   `horizon` and `jumps` with the `exhaustive` dimensions FR-193 "Proof
   coverage" requires; and the claim is definitely true when each of its
   atoms is read over every step's `B` and `time` by three-valued interval
   evaluation (definitely true, definitely false or unknown over the step)
   and the claim's pointwise meaning is applied to those readings over the
   claim region.

   **Guards and boundary invariants.** A step's boxes are contracted by its
   mode's invariant, so no box holds a state past the invariant's boundary:
   a reached state satisfies its mode's invariant at every instant of its
   dwell, and contraction removes only states that no behaviour reaches.
   The *guarded part* of a step's `B` is its contraction by the guard: each
   equality conjunct `x = e` narrows `x` to the meet of `x` and `e` over
   `B`, each other conjunct narrows by interval propagation, and an empty
   meet means the guard cannot hold over `B`. Jumps of one transition from
   one dwell are carried by one *jump step*, the first step of the target
   dwell: its `box` SHALL contain the reset's image of the guarded part of
   every `B` of the source dwell over which the guard can hold, and its
   `time` SHALL contain each such step's `time`. When the guard's equality
   conjunct `x = c`, `c` a constant, has a rate for `x` built only from
   variables whose rate is `0` in the source mode, the jump time of a
   behaviour from the dwell's first box is `(c - x0) / r`, and a jump step
   MAY narrow its `time` to the *narrowed jump time*: the interval value of
   that quotient over the dwell's first box, offset by that box's `time`,
   intersected with `[0, horizon]`, the proof's horizon, when the
   quotient's divisor interval excludes 0. A jump after the horizon lies
   outside the proof region, so the intersection drops no state the proof
   must read. The reader SHALL accept a narrowed `time` exactly when it
   contains the narrowed jump time. When the source invariant has its
   boundary at the guard (a *boundary invariant*, as `v >= 0` with the
   guard `v = 0`), a box never holds states on both sides of the guard: the
   source dwell's boxes stop at the guard by invariant contraction, and the
   target dwell starts at the guard's reset image. When it does not, the
   source dwell continues past the guard, as a behaviour that does not take
   the transition may.
3. **`validated-witness`**, with a `HybridCounterexample` or a
   `solver-inconclusive` trace: a point start state and, for each dwell, a
   list of Picard steps as in form 2 whose boxes enclose the unique solution
   from that point, each flow's conditional (`if`) definitely taking one
   branch over each `B` so the solution is unique, each jump's guard
   definitely true over the box at which the jump is taken, and a
   *decisive box* over which the claim is definitely violated.

   **Readings and the decisive box.** The reader reads each claim atom by
   three-valued interval evaluation (definitely true, definitely false or
   unknown) over the two boxes each witness step holds: its `box`, which
   encloses the solution at the one instant `time.lower` at which the step
   starts, and its a-priori box `B`, which encloses the solution at every
   instant of the step's `time`. A reading over `box` holds at that one
   instant, a reading over `B` at every instant of `time`, and an instant of
   the claim region that no definite reading covers reads unknown. The
   decisive box is the box of the earliest reading at which either the
   prefix up to that reading is a bad prefix of the claim for every value
   the unknown readings could take, by the claim's pointwise meaning over
   the claim region, or an expression the claim reads is definitely without
   value over the box. The witness SHALL be rejected `witness-violation`
   when it has no decisive box. A box that straddles an atom's boundary,
   as `v` in `[1, 41]` against `v < 30`, reads that atom unknown, and an
   unknown reading never refutes, even when the true solution lies on the
   violating side: the reader never settles an unknown reading by a
   midpoint, an endpoint or a sample of the box, because only a definitely
   false reading proves that every solution the box encloses violates the
   claim. A provider that cannot build a witness with a decisive box
   returns its trace without one, and the item stays `inconclusive` as
   FR-193 "Trace replay" states.

### Rule names

A rejection names the first failing rule by one of these spellings, and
`at` is the RFC 6901 pointer of the box or step it failed at; for
`witness-violation`, which no single step fails, `at` is the pointer of the
witness's last step. The rules are checked in the order listed within each
form.

| Form | Rule | Spelling |
| --- | --- | --- |
| every form | a box keyed differently from the declaration's variables, or with `lower > upper` | `box-shape` |
| `inductive-enclosure` | (a) initial containment | `a` |
| `inductive-enclosure` | (b) faces | `b` |
| `inductive-enclosure` | (c) transitions | `c` |
| `inductive-enclosure` | (d) claim | `d` |
| `flowpipe` | the first step's box contains every initial state | `initial` |
| `flowpipe` | a step's a-priori box satisfies the Picard condition | `picard` |
| `flowpipe` | the next step's box contains the step's successor | `successor` |
| `flowpipe` | every transition whose guard can hold is followed by a jump step whose `box` and `time` contain what "Guards and boundary invariants" requires | `jump` |
| `flowpipe` | the steps cover the proof's `horizon`, `jumps` and `exhaustive` | `coverage` |
| `flowpipe` | the claim is definitely true over the steps | `claim` |
| `validated-witness` | the start is a point satisfying an initial entry | `witness-start` |
| `validated-witness` | each dwell's steps satisfy the Picard condition and enclose the solution | `witness-picard` |
| `validated-witness` | each conditional takes one branch over each `B` | `witness-unique` |
| `validated-witness` | each jump's guard is definitely true where it is taken | `witness-guard` |
| `validated-witness` | the witness has a decisive box: a reading over a step's `box` or `B` at which the claim is definitely violated, or an expression is definitely without value | `witness-violation` |
| every form | the item's work limit stopped the check | `work-limit` |

### Settlement

- The record of a `proved` entry whose certificate is accepted SHALL carry
  the FR-331 certification `certified`, whatever provider produced it. The
  entry itself keeps the label its provider wrote, `uncertified` from a
  first-party provider or `trusted` from a plugin (FR-331 "Certification"),
  because the provider cannot vouch for the core's check. A rejected
  certificate SHALL leave the record as FR-193 settles it without one (a
  first-party proof `uncertified`, a plugin's `trusted`), still `proved`,
  and SHALL record `certificate-rejected{rule, at}` beside it; it never
  turns a proof into a refutation or an `inconclusive`.
- A `certificate` member SHALL be one of the three forms, each a closed
  object with a `type` member: `{"type": "inductive-enclosure", "boxes":
  {mode: box}}`, `{"type": "flowpipe", "steps": [step, ...]}` and
  `{"type": "validated-witness", "dwells": [[step, ...], ...]}`, one list
  of Picard steps per dwell of the trace it accompanies, whose start point
  is that trace's `start`. A `proved` entry with a `bounded-solver` or
  `inductive-solver` basis MAY carry one; a `refuted` entry and an
  `inconclusive` entry MAY carry only a `validated-witness`; no other entry
  carries one.
- A trace that exact replay cannot decide (FR-193 "Trace replay": a mode
  whose flow is not polynomial, or a trace that is not a point trace only
  because its `end` holds the decisive box) SHALL settle `refuted` when its
  `validated-witness` is accepted, from a `refuted` entry or a
  `solver-inconclusive` cause alike, with the trace as its
  `HybridCounterexample` and one `counterexamples` entry; its replay
  settlement is FR-364's `reproduced-with-validated-witness`, distinct from
  `reproduced-with-evaluated-witness` so a reader can tell a refutation
  proved by exact interval enclosure from one replayed exactly. The
  counterexample's kind is `undefined-evaluation` when the decisive box is
  one over which an expression is definitely without value, and `formula`
  otherwise. When the witness is absent or rejected, the item settles as
  FR-193 states: static replay only and no refutation, a `refuted` entry
  settling `inconclusive`, `replay-parity`. A trace exact replay decides
  settles by exact replay alone, whatever witness it carries.
- A trace whose witness was accepted MAY end with FR-193's `end` holding the
  decisive box, because the solution's values at the decisive instant are
  in general irrational: `end.time` is the instant of a `box` reading as a
  point interval, or the step's `time` for an a-priori box reading, and
  `end.enclosure` is that box.
- A replay request (FR-323) for an item refuted through an accepted witness
  SHALL carry that `validated-witness` beside the `HybridCounterexample`,
  and the replay SHALL check it again by this requirement, so the
  refutation replays from the request alone.
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
| FR-652-AC-3 | Over `Grow`, one mode `Up` with invariant `v >= 1 and v <= 100`, initial `v = 1` and flow `v' = 1 - 1 / (1 + v * v)`, which is not polynomial, against `always [0, 40] holds(h.v < 30)`: a point trace from `v = 1` with a `validated-witness` of nine steps of 4 s, step `k` at time `[4k, 4k + 4]` with `box` `v [l_k, 1 + 4k]` and `B` `v [l_k, 5 + 4k]`, `l` being 1, 3, 33/5, 21/2, 72/5, 183/10, 111/5, 261/10, 30, is accepted, its decisive box the `box` `v [30, 33]` at time 32 over which `v < 30` is definitely false, and settles `refuted` with one `counterexamples` entry. The same trace stays `inconclusive` without a witness; with that witness cut after its step at `[28, 32]`, whose `B` `v [261/10, 33]` straddles 30; and with one step at `[0, 40]` whose `box` is `v [1, 1]` and whose `B` `v [1, 41]` satisfies the Picard condition but straddles 30. Each cut witness is rejected `witness-violation` at its last step, `/certificate/dwells/0/7` and `/certificate/dwells/0/0`. | Test (TC-1605) |
| FR-652-AC-4 | A `flowpipe` certificate on an `inductive-solver` entry refuses `invalid_result`/`invalid-value` at `/certificate`; a box missing variable `a` is rejected at its pointer; a check stopped by the work limit is `rejected{rule: work-limit}`. | Test (TC-1605) |
| FR-652-AC-5 | Over `Braking` without `t`, whose `Moving` invariant `v >= 0` is a boundary invariant of its guard `v = 0`, a `flowpipe` certificate whose `Moving` steps of `step` 1/16 are each contracted to `v >= 0` and whose one jump step is `Stopped: v [0, 0], a [1, 2]` with `time` `[0, 10]`, the narrowed jump time (the value `[0, 20]` of `(0 - v0) / (-a)` over `v0 [0, 20], a [1, 2]`, offset by time 0 and intersected with the horizon `[0, 10]`), is accepted against `always [0, 10] holds(h.v < 30)` and settles `certified`; the same with jump-step `time` `[1, 10]` is rejected with rule `jump` at that step, since it omits the jump at time 0 from `v0 = 0`; the same with `time` `[0, 20]` is accepted, since it contains the narrowed jump time; against `always [0, 10] holds(h.v > 5)` the claim is not definitely true over the first `Moving` step and the certificate is rejected with rule `claim`. | Test (TC-1605) |
| FR-652-AC-6 | The schema admits a first-party `proved` `inductive-solver` entry labelled `uncertified` carrying an `inductive-enclosure` certificate and a `bounded-solver` entry carrying a `flowpipe` certificate whose `step` is `{"numerator": "1", "denominator": "16"}`, and refuses the same with `step` `"1/16"`, a box bound written `"0"`, a certificate `type` outside the three forms, the entry labelled `certified`, and a `flowpipe` on an `inconclusive` entry; when the reader accepts the `inductive-enclosure` the record is `certified` while the entry stays `uncertified`, and when it rejects it the record stays `proved`, `uncertified`, with `certificate-rejected{rule, at}` beside it and not `inconclusive`. | Test (TC-2049) |

## Dependencies

- [FR-193](FR-193-analyze-continuous-hybrid-models.md) owns hybrid models,
  claims, verdicts, traces and replay.
- FR-331 owns the result entry and its certification values.
