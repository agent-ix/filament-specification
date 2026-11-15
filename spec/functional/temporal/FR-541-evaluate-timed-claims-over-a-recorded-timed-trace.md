---
id: FR-541
title: "Evaluate timed claims over a recorded timed trace"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-540
    type: references
  - target: ix://agent-ix/filament-specification/FR-243
    type: references
---
## Description

When a claim under `filament.temporal.timed/v1` is evaluated over one recorded
timed trace, the evaluator SHALL read it with FR-416's pointwise meaning over the
trace's time-stamped positions and SHALL settle it exactly over a timed lasso and
three-valued over a finite timed prefix.

## Inputs

A timed claim over a timed model subject (bound `model-time`) or over a trace
subject (its role bound to a time unit by FR-540); a timed trace: positions in
order, each with an exact non-negative rational time stamp in the binding's
unit, stamps non-decreasing, either

- a finite prefix, with an optional completeness stamp `through` at or after the
  last stamp: no position with stamp at most `through` exists beyond the supplied
  ones; or
- a timed lasso: a prefix and a non-empty loop with a positive period `P` at
  least the loop's span; repetition `k` of loop position `j` has stamp
  `τ_j + k·P`.

## Outputs

`holds`, `violated` at a position, `pending` (basis `unsettled`), an undefined
evaluation, or a refusal naming the defect.

## Behavior

Over a timed lasso the evaluator SHALL decide the claim exactly: the lasso is one
time-divergent behaviour, and positions of repetitions beyond the claim's largest
finite interval bound plus the loop span repeat their values.

Over a finite prefix the evaluator SHALL report `holds` when every time-divergent
extension consistent with the prefix and `through` satisfies the claim,
`violated` when none does, and `pending` otherwise. An extension appends
positions with stamps at least the last stamp and greater than `through` when it
is given. The prefix-decided results carry `decisive-witness` or
`decisive-counterexample`; `pending` carries `unsettled` (FR-243).

Every timed interval SHALL be compared exactly on rational stamps. Punctual
intervals are admitted over a single trace (FR-416).

The evaluator SHALL refuse decreasing stamps, an empty prefix of a finite trace,
an empty loop, a non-positive or too-short period, and a `through` below the last
stamp, each naming the defect and the position. It SHALL refuse a trace without
stamps for a timed claim: positions are not substituted for time.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-541-AC-1 | `always (holds(req) implies eventually[0, 200] holds(ack))` over the finite trace `req`@0, `ack`@150 is `pending`; over `req`@0, `ack`@250 with `through` 250 it is `violated`; over the lasso prefix `req`@0, loop `ack`@100, `req`@120, period 200 it `holds`, and with period 400 it is `violated`. | Test (TC-1161) |
| FR-541-AC-2 | `eventually[0, 1/2] holds(p)` over stamps 0 and 1/3 with `p` only at the second holds; over stamps 0 and 2/3 with `p` only at the second and `through` 2/3 it is violated. | Test (TC-1161) |
| FR-541-AC-3 | Stamps 0, 5, 3; a loop period of 0; and a timed claim over a trace without stamps each refuse naming the defect. | Test (TC-1161) |

## Dependencies

- [FR-416](FR-416-evaluate-timed-temporal-claims.md) (pointwise timed meaning),
  [FR-090](FR-090-select-temporal-profile-and-clock.md),
  [FR-540](FR-540-supply-clock-configurations-by-clock-role.md).
