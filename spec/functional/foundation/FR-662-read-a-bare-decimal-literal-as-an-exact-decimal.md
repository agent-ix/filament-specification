---
id: FR-662
title: "Read a bare decimal literal as an exact decimal value"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-005
    type: implements
  - target: ix://agent-ix/filament-specification/FR-140
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-039
    type: references
  - target: ix://agent-ix/filament-specification/FR-407
    type: references
---

# FR-662: Read a bare decimal literal as an exact decimal value

## Description

When a `decimal-literal` such as `0.9`, `5.0` or `0.90` stands in a `primary`
position, an implementation SHALL read it as exactly the decimal constructor
`decimal(c, s)` that spells the same number: `c` the literal's digits with the
point removed, read as an integer, and `s` the count of digits after the
point. `0.9` is `decimal(9, 1)`, `5.0` is `decimal(50, 1)` and `0.90` is
`decimal(90, 2)`. The literal is typed, limited, lowered and evaluated exactly
as that constructor ([FR-140](../type-model/FR-140-evaluate-exact-decimals.md)).

## Rationale

The same spelling already denotes an exact rational in every `exact-value`
position (probability thresholds, quantity magnitudes, weights), so authors
write `self.level < 0.9` and `expected ... <= 0.5` and expect the same
reading. The literal is exact by construction; reading it as the constructor
it spells adds no value kind and no typing rule, and it is never floating
point.

## Inputs

A complete-V1 source unit holding a `decimal-literal` token in a `primary`
position.

## Outputs

The constructor's typed value or its typed refusal, exactly as for
`decimal(c, s)`.

## Behavior

- A `decimal-literal` followed by a qualified name is a `quantity-literal`
  (the longer match); any other `decimal-literal` in a `primary` position is
  the decimal primary.
- The decimal primary takes its `Decimal[..]` type from its position as the
  constructor does, the other operand's static type when that operand is not a
  literal included, so `self.level < 0.9` over a field `level: Decimal[..]`
  checks ([FR-140](../type-model/FR-140-evaluate-exact-decimals.md)); a position that gives no type, or expects a type that is
  not a `Decimal[..]`, refuses exactly as the constructor refuses there.
- Where a rule takes an exact rational literal (a dimensionless threshold of
  a probabilistic measure or expectation, [FR-407](../analysis/FR-407-state-probabilistic-claims.md)),
  the decimal primary is the exact rational it denotes, so `0.5`,
  `decimal(5, 1)` and `rational(1, 2)` give equal thresholds.
- An implementation SHALL refuse an exponent spelling (`1e5`, `1.5E-3`)
  `invalid_syntax`/`invalid-token` at the whole spelling, the unsupported
  numeric-form diagnostic.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-662-AC-1 | `0.9`, `5.0` and `0.90` as primaries parse with no diagnostic and denote `decimal(9, 1)`, `decimal(50, 1)` and `decimal(90, 2)`; `0.5 M::ms` is a quantity literal. | Test (TC-1648) |
| FR-662-AC-2 | At a `Decimal[-1000, 1000; 0, 2; exact]` result `0.9` checks; compared with an operand of that type (`l < 0.9`) it checks, typed by the operand; compared with another literal (`0.9 < decimal(1, 0)`) it refuses `ill_typed`/`ambiguous-literal` at `0.9`; compared with an `Integer` operand it refuses `ill_typed`/`type-mismatch`; bound by a `let` with no type it refuses `ill_typed`/`ambiguous-literal`; each outcome equals the outcome with `decimal(9, 1)` in its place. | Test (TC-1648) |
| FR-662-AC-3 | An `expected accumulate R until holds(B) <= 0.5` claim over an integer reward has the threshold `1/2`, equal to the claim written with `decimal(5, 1)` and with `rational(1, 2)`. | Test (TC-1648) |
| FR-662-AC-4 | `1e5` and `1.5E-3` as primaries refuse `invalid_syntax`/`invalid-token` located at the whole spelling. | Test (TC-1648) |

## Dependencies

- [Proposed shared grammar](../../../proposals/filament-v1/shared-grammar.md): `primary` and `decimal-literal`.
- [FR-140](../type-model/FR-140-evaluate-exact-decimals.md): exact decimal typing and evaluation.
