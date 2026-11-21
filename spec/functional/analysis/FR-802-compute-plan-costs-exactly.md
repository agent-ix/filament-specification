---
id: FR-802
title: "Compute plan objectives with exact arithmetic"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-194
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-801
    type: references
---

# FR-802: Compute plan objectives with exact arithmetic

## Description

When a plan synthesis declares an objective, the planner SHALL evaluate it
with the language's own value semantics, exactly, in a fixed evaluation
order, and SHALL classify it as a per-step sum or a general objective,
because the classification decides which certificate can prove optimality.

## Inputs

The objective expression (`minimize e` or `maximize e`, FR-194), the plan
value (FR-800) and the evaluation meter.

## Outputs

One exact objective value per admissible plan, its classification, and the
comparison the search and the tie rule (FR-804) use.

## Behavior

### Objective types

- The objective SHALL be numeric: `Integer` or an integer range,
  `Rational`, `Decimal` or a decimal range, a quantity with a unit
  (including `Duration`), or a binary floating-point type. Any other type
  refuses `ill_typed`/`type-mismatch` (FR-194).
- A declaration with no objective SHALL minimize `length`.

**Current FSL behavior:** the planner supports `Integer`, `Rational` and
`Decimal` objectives. A typed floating-point (`Float32` or `Float64`) or
unit-bearing `Quantity` objective returns a typed `unsupported` result
before search, naming the numeric family; it does not produce an undefined
objective or an infeasible result. This implementation boundary does not
change the admitted objective types or the arithmetic, `totalOrder`, NaN,
unit-conversion and certificate requirements below.

### Exact arithmetic

- `Integer`, `Rational` and `Decimal` objectives SHALL be computed exactly:
  integers with no overflow wrap, rationals in lowest terms, decimals
  without rounding. The size of every intermediate value charges the
  evaluation meter's `integer_bits` and `decimal_digits` counters; a denied
  charge stops the search `incomplete` (FR-801), never rounds.
- A quantity objective SHALL be computed in the objective expression's
  unit; values of different units of one dimension are converted by the
  exact rational conversion factor before they are added or compared.
- A floating-point objective SHALL be computed under the package's selected
  floating-point profile (for example IEEE 754-2019 default rounding), each
  operation rounded as that profile states, in the evaluation order below.
  Plans are compared by the profile's `totalOrder` predicate; a plan whose
  objective is a NaN is not admissible, and the result counts such plans in
  `explored.nan_objectives`.
- A plan whose objective evaluation is undefined (a division by zero, an
  out-of-range conversion) SHALL NOT be admissible; the result counts such
  plans in `explored.undefined_objectives`.

### Evaluation order

A sum over `steps` SHALL add its terms in step order, from step 0. Exact
types make the order immaterial to the value; floating-point types make it
part of the definition.

The type checker SHALL determine a step sum's summand type from the summand
expression without reference to the objective, and derive the total's type
from that complete summand type, including its coefficient bounds, scale
bounds and rounding mode for a decimal type.
A decimal literal within the summand takes its unique expected type only
from the typed operand, declared or annotated position, `convert` target or
enclosing typed decimal operation specified by
[FR-140](../type-model/FR-140-evaluate-exact-decimals.md); that complete type
passes through the position's conditional branches and decimal operands.
When no such unique expected type exists, type checking SHALL refuse
`refused { code: ill_typed, cause: ambiguous-literal }`.
Neither literal spelling nor the objective supplies the summand's decimal
type or any of that type's coefficient bounds, scale bounds or rounding mode.

### Classification

- An objective is a *per-step sum* when it is `length`, `elapsed` under tick time (FR-808), or an
  expression `Reach.steps.sum(s => c)` (optionally plus a constant) whose
  term `c` reads only the step record `s`, and the objective's type is an
  exact type. Its value is the sum of the per-step costs.
- A per-step sum is *non-negative* when every step cost is `>= 0`: either
  the term's type has a lower bound `>= 0`, or the search evaluates every
  step it applies and finds no negative cost. A negative step cost found
  during the search reclassifies the objective as general for that run, and
  the result records `objective.negative_step` with the first such step.
- `elapsed` under dense time SHALL be general: its delays range over the
  region abstraction, not a finite step set, so it takes a `closure`
  certificate over the region graph (FR-808).
- Every other objective, including every floating-point objective and every
  objective over `final`, is *general*.
- A `potential` certificate (FR-650) SHALL be used only for a non-negative
  per-step sum, and a `closure` certificate for every other objective.

### Comparison

For `minimize`, a plan *improves on* another when its objective value is
smaller; for `maximize`, when it is greater. Equal values are a tie, broken
by FR-804.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-802-AC-1 | `minimize Reach.steps.sum(s => let cost: Decimal[0,10000;2,2;exact] = if s.operation = M::Op::charge then 0.10 else 0.05 in cost)` over three steps, two `M::Op::charge` and one other, evaluates to exactly `0.25` of type `Decimal[0,10000;2,2;exact]`, with no binary rounding, and the certificate is `potential`. | Test (TC-1952) |
| FR-802-AC-2 | An objective summing `1/3` over three steps is exactly `1` as a `Rational`; an objective whose intermediate value exceeds `accounting.integer_bits` stops the search `incomplete` naming that setting. | Test (TC-1952) |
| FR-802-AC-3 | An objective adding `30 s` and `1 min` per step compares in seconds, so a plan of one `1 min` step and one of two `30 s` steps tie at `60 s`. | Test (TC-1952) |
| FR-802-AC-4 | A step cost `s.after.value - s.before.value` that is negative on some step reclassifies the objective as general, the certificate is `closure`, and `objective.negative_step` names that step. | Test (TC-1952) |
| FR-802-AC-5 | A `Float64` objective that is NaN on one plan excludes that plan, counts it in `explored.nan_objectives`, and ranks the remaining plans by `totalOrder`; an objective undefined on a plan excludes it and counts it in `explored.undefined_objectives`. | Test (TC-1952) |
| FR-802-AC-6 | `maximize Reach.final.value` is general and settles with a `closure` certificate; with no objective the plan minimizes `length`. | Test (TC-1952) |
| FR-802-AC-7 | With `type Cost = Decimal[0,10000;2,2;exact];`, `minimize Reach.steps.sum(s => convert<Cost>(0.10))` over three steps evaluates to exactly `0.30`; the literal and total have the complete `Cost` type, including coefficient bounds `[0,10000]`, scale bounds `[2,2]` and mode `exact`, and the non-negative per-step sum takes a `potential` certificate. | Test |
| FR-802-AC-8 | `minimize Reach.steps.sum(s => if s.operation = M::Op::charge then 0.10 else 0.05)` and the same objective with `decimal(10,2)` and `decimal(5,2)` in place of its decimal primaries both refuse at type checking with `refused { code: ill_typed, cause: ambiguous-literal }`: neither summand has an FR-140 source of a unique expected decimal type, and neither the objective nor the literal spellings supplies one. | Test |

## Dependencies

- [FR-194](FR-194-synthesize-bounded-candidates.md) owns the objective form.
- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the
  certificates; [FR-801](FR-801-bound-and-meter-the-plan-search.md) the
  meter and the incomplete stop.
