---
id: FR-092
title: "Evaluate bounded past temporal operators"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-090
    type: references
  - target: ix://agent-ix/filament-specification/FR-250
    type: references
  - target: ix://agent-ix/filament-specification/FR-253
    type: references
  - target: ix://agent-ix/filament-specification/FR-254
    type: references
  - target: ix://agent-ix/filament-specification/FR-255
    type: references
  - target: ix://agent-ix/filament-specification/FR-257
    type: references
  - target: ix://agent-ix/filament-specification/FR-293
    type: references
  - target: ix://agent-ix/filament-specification/FR-161
    type: references
  - target: ix://agent-ix/filament-specification/FR-367
    type: references
---
## Description

When evaluating an admitted bounded past operator, the temporal evaluator shall
distinguish positions before an authoritative execution origin from required
history that was omitted or evicted.

## Inputs

A temporal subject and anchor, inclusive integer bounds `0 <= a <= b`, total
Boolean predicate valuations, and either an authoritative execution origin or
the complete required history identified by the observation binding.

## Outputs

A `satisfied` or `violated` past-time truth disposition, or a separate
incomplete/refused evaluator outcome that identifies missing history, an
unsupported profile/operator or a resource failure. A non-completed evaluator
outcome carries no truth fact at this boundary; the canonical result adapter
represents that absence as truth `unavailable` with settlement basis
`unavailable`.

## Behavior

For an anchor whose history is complete back to an authoritative origin, the
false-extension profiles shall treat atomic predicates before that origin as
false and Boolean constants as unchanged. They shall use:

| Operator | Exact past meaning |
| --- | --- |
| `once[a,b] p` | Some offset `d` in `[a,b]` makes `p` true at `i-d` |
| `historically[a,b] p` | Every offset `d` in `[a,b]` makes `p` true at `i-d` |
| `p since[a,b] q` | Some `d` in `[a,b]` makes `q` true at `i-d`, and every `e` in `[a,d)` makes `p` true at `i-e` |
| `p triggered[a,b] q` | The Boolean dual of `not p since[a,b] not q` under the same profile |

A supplied `history-cutoff` is not an `execution-origin`. If the cutoff does not
prove that every needed position is inside the available history, then the
evaluator shall report missing history rather than apply false extension.
An admitted state-restoration contract may authenticate complete retained
history through a `history-cutoff`, but it shall not change that boundary kind
or authorize interpretation before the cutoff.

The timestamped finite-window profile shall quantify admitted past instants by
inclusive clock distance.

The timestamped finite-window evaluator shall require complete history over the
window before applying empty-window duals.

The v1 profile shall define strong previous as `once[1,1]`.

If a later profile admits weak previous, then that profile shall define its
boundary separately from strong previous.

### Derived forms

Previous, next and weak until are derived forms, not keywords of the temporal
grammar. Strong previous is written `once[1,1] p`; next is written
`eventually[1,1] p`; `p` weak-until `q` is written `(p until[a,b] q) or
always[a,b] p` with an interval and, where the infinite-trace profile admits
unbounded operators, `(p until q) or always p`. The parser shall refuse the
words `previous`, `next` and `weak` written in a temporal operator position as
a syntax error located at the word, whose message names the derived form. One
spelling per operator keeps a formula's meaning the same in every tool that
reads it, and the derived forms already carry the exact meanings above.

### Positions before the origin

Where history is complete back to an authoritative origin (the false-extension
profiles, the timestamped finite-window profile with an `execution-origin`
boundary, and the infinite-trace profile, whose position 0 is the origin), a
past operator with a closed interval `[a,b]` whose offset reaches a position
before the origin shall read that position as part of a pre-origin extension
of the trace. At every pre-origin position each atomic predicate is false and
each Boolean constant keeps its value, and Boolean and past operators evaluate
there from those values, recursively. A pre-origin offset is never skipped and
never narrows the interval:

- `once[a,b] p` with atomic `p` finds no witness before the origin, so
  `once[1,1] p` is false at position 0;
- `once[1,1] not p` is true at position 0, because `not p` holds at the
  pre-origin position;
- `historically[0,2] p` with atomic `p` is false at positions 0 and 1 whatever
  `p` is at the positions that exist, and `historically[0,2] true` is true;
- `p since[a,b] q` finds no witness for an atomic `q` before the origin, and
  `p triggered[a,b] q` is its dual under the same reading.

The unbounded `once`, `historically`, `since` and `triggered` operators of the
infinite-trace profile, and the lower-bounded `[a,*]` past operators, range
over the positions from 0 through `i - a` only, as FR-161 and FR-367 state; a
window with no upper bound would otherwise consist almost entirely of
pre-origin positions. A closed interval has a fixed width at every position,
so its pre-origin reading mirrors the false extension that the bounded
profiles apply after closure.

This reading binds every evaluator and backend of these profiles, so a formula
has one truth value whichever of them settles it. An evaluator that skips
pre-origin offsets of a closed interval, clipping the window at the origin,
does not conform: the TL ecosystem's infinite-trace evaluator reads closed
past intervals this way to conform, which on a trace where `p0` holds at every
position changes `once[1,1] not p0` at position 0 from false to true and
`historically[0,2] p0` at position 1 from true to false. Its origin-history
evaluator already reads them this way. A `history-cutoff` boundary is not an
origin and keeps the missing-history rule above.

Where a bounded temporal profile is selected, the evaluator shall refuse an unbounded past interval. Where the infinite-trace profile is selected, the evaluator shall admit the unbounded historically, once, since and triggered operators with the meaning [FR-161](FR-161-analyze-infinite-trace-properties.md) states. Where the infinite-trace profile is selected, the evaluator shall admit the interval operators above with their past offset meaning over the infinite trace, whose position 0 is the authoritative origin ([FR-367](FR-367-evaluate-interval-operators-under-infinite-trace.md)). Implicit history reconstruction is refused under every profile. The refusal and the authoritative-origin meaning above bind the evaluators of those profiles; the tick monitor of filament-spec-language FR-251 is a separate evaluator whose unbounded past operators have the history-present meaning with a `HistoryCutoff` lower boundary (an unknown prefix, filament-spec-language FR-251 "Unbounded past operators"), which refines the authoritative-origin result: wherever the monitor's verdict is definite it equals the infinite-trace verdict.

Strong previous is `once[1,1]` under the infinite-trace profile as under every
other profile: true at position `i > 0` exactly when its operand holds at
`i - 1`. At position 0 it reads its operand at the pre-origin position
("Positions before the origin"), so it is false for an atomic operand and true
for `not p`. It has no unbounded form. It counts
positions, so a formula that contains it is not stutter-invariant.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-092-AC-1 | Independent truth tables distinguish all four past operators, inclusive endpoints, zero-width intervals, nonzero lower bounds and each origin boundary. | Test (TC-112) |
| FR-092-AC-2 | The same visible suffix yields a Boolean when its start is the authoritative execution origin and missing-history incomplete when it is merely a cutoff. | Test (TC-112) |
| FR-092-AC-3 | `p since[1,2] q` uses `p` only between the lower bound and its witness, symmetrically with the selected future `until` convention. | Test (TC-112) |
| FR-092-AC-4 | Missing, evicted, stale or binding-mismatched history never becomes false padding or a conclusive Boolean. | Test (TC-118) |
| FR-092-AC-5 | The selected `tl-syntax.past-operators/v1` and `mltl.origin-complete-history/v1` contracts admit the bounded past operators through their exact public readers; a future-only, unknown or mismatched profile remains unsupported/refused without substitute output. | Test (TC-117) |
| FR-092-AC-6 | Exactly the FR-293 boundary kinds are admitted; restoration may prove cutoff coverage but never converts a cutoff into an execution origin. | Test (TC-112, TC-118) |
| FR-092-AC-7 | Where the infinite-trace profile is selected, the unbounded past operators evaluate under their FR-161 infinite-trace meaning, `once[1,1]` is false at position zero, and history remains explicitly bound. | Test (TC-200) |
| FR-092-AC-8 | Where the infinite-trace profile is selected, `once[a,b]`, `historically[a,b]`, `since[a,b]` and `triggered[a,b]` evaluate with their past offset meaning, reading atomic predicates before position 0 as false, and inserting one repeated state changes the truth of a formula containing `once[1,1]`. | Test (TC-298) |
| FR-092-AC-9 | Under the event-position false-extension, infinite-trace and timestamped finite-window (`execution-origin`) profiles, on a trace where atomic `p` holds at every position: `once[1,1] p` at position 0 is false, `once[1,1] not p` at position 0 is true, `historically[0,2] p` at position 1 is false, `historically[0,2] true` at position 1 is true, and `true since[1,1] not p` at position 0 is true; under the infinite-trace profile the unbounded `historically p` at position 1 is true and `once[2,*] not p` at position 1 is false. A `history-cutoff` boundary gives missing history for the closed-interval cases instead. | Test (TC-1041) |
| FR-092-AC-10 | `previous holds(p)`, `next holds(p)` and `holds(p) weak until holds(q)` each refuse as a syntax error at the word `previous`, `next` or `weak`, with a message naming `once[1,1]`, `eventually[1,1]` and `(p until q) or always p` respectively; `once[1,1]` evaluates as strong previous, `eventually[1,1] p` holds at `i` exactly when `p` holds at `i + 1`, and `(p until[a,b] q) or always[a,b] p` holds exactly when `q` holds at some offset `d` in `[a,b]` with `p` at every offset in `[a,d)`, or `p` holds at every offset in `[a,b]`. | Test (TC-1041) |

## Dependencies

- [FR-090](FR-090-select-temporal-profile-and-clock.md).
- The TL ecosystem owns the exact formula-v2, `tl-syntax.past-operators/v1`,
  `mltl.origin-complete-history/v1`, position-history and past-evaluator
  contracts, and the `tl-syntax.formula-unbounded/v1`,
  `tl-syntax.lasso-trace/v1` and `mltl.infinite-trace/v1` contracts of the
  infinite-trace evaluator FR-095 maps closed-past infinite-trace requests to; this requirement consumes their selected public interfaces.

## Status

Accepted by FS04 for bounded origin-complete past/history meaning. TL-owner and
native-bridge execution remain independently versioned and tracked.
