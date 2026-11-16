---
id: FR-766
title: "Admit a lower-bounded timed interval with no upper bound"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-255
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-250
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-367
    type: references
---

# FR-766: Admit a lower-bounded timed interval with no upper bound

## Description

Under `filament.temporal.timed/v1`, the frontend SHALL admit a temporal
operator written with a timed interval whose lower bound is a time quantity
and whose upper end is `*` (`[q, *]` or `(q, *]`, written `(q, *)` as well),
on every operator the profile admits, past and future. The interval measures
the time stamp distance `τ_j − τ_i` (FR-416) as at least `q` (closed lower
end) or more than `q` (open lower end), with no upper limit.

### Decision: admit `[q, *]` under the timed profile

The timed profile already admits the unbounded temporal operator grammar
(FR-250-AC-6): `eventually holds(p)` and `once holds(p)` are timed claims. An
interval `[q, *]` is that unbounded operator restricted to distances of at
least `q`, so it adds no reading the profile lacks: "a reply arrives no
sooner than 5 ms after the request, at any time after" and "the alarm was
raised at least 10 s ago" are ordinary timed requirements. Refusing the form
while admitting its `[0, *]` case would leave these claims unwritable, and the
infinite-trace profile already admits the step form `[a, *]` (FR-367). The
bounded profiles keep refusing every `[a, *]` interval, because each of them
closes a finite window (FR-091, FR-092).

## Inputs

A temporal operator under the timed profile carrying a timed interval whose
upper end is `*`.

## Outputs

The checked interval (FR-255) with an exact rational lower bound in the clock
binding's unit, its lower-end openness, and no upper bound; its key carries
the timed profile identity and the binding.

## Behavior

- The lower bound SHALL be a time quantity converted exactly to the binding's
  unit, as for a finite timed interval (FR-252 "Intervals"); a bound with no
  unit or of another dimension refuses `ill_typed`/`type-mismatch` at its
  span.
- The upper end `*` SHALL be open. A `]` written after `*` is read as `)`, and
  the two spellings give one interval and one key.
- `eventually_[q,*] p` SHALL hold at position `i` when some `j >= i` has
  `τ_j − τ_i >= q` and `p` at `j`; `until`, `always` and `release` follow from
  it as for a finite interval. `once_[q,*] p` SHALL hold at `i` when some
  `j <= i` has `τ_i − τ_j >= q` and `p` at `j`; `historically`, `since` and
  `triggered` follow likewise. An open lower end replaces `>=` with `>`.
- A claim holding a `[q, *]` operator SHALL NOT be a TT-2 bounded window
  (FR-416), since the operator has no upper bound; it is TT-3 timed safety or
  TT-4 timed liveness by its formula.
- `[0, *]` and the operator with no interval SHALL be one checked operator with
  one key.
- The key of `[q, *]` SHALL differ from the key of every finite interval and
  from the key of the step interval `[a, *]` under the infinite-trace profile.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-766-AC-1 | Under the timed profile with clock unit `ms`, `once[5 ms, *] holds(p)` and `eventually[5 ms, *] holds(p)` check with lower bound `5`, lower end closed and no upper bound; `once(5 ms, *) holds(p)` checks with an open lower end; `once[5 ms, *]` and `once[5 ms, *)` give equal keys; `once[0 ms, *] holds(p)` and `once holds(p)` give equal keys; `once[5, *] holds(p)` refuses `ill_typed`/`type-mismatch`. | Test (TC-1841) |
| FR-766-AC-2 | Over the timed trace with `p` true only at time stamp 0 ms and positions at 0, 4 and 6 ms, `once[5 ms, *] holds(p)` is false at 4 ms and true at 6 ms; `once(6 ms, *) holds(p)` is false at 6 ms. | Test (TC-1841) |
| FR-766-AC-3 | Under the timestamped-event finite-window profile `once[5 ms, *] holds(p)` still refuses as FR-091 and FR-092 state for an unbounded interval under a bounded profile. | Test (TC-1841) |

## Dependencies

- FR-255 owns the interval object and its key; FR-250 owns the timed profile.
- FR-416 owns timed pointwise meaning and the TT forms; FR-367 owns the step
  form `[a, *]` under the infinite-trace profile.
