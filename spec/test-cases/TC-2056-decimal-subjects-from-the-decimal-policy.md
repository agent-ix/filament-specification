---
id: TC-2056
title: "Decimal subjects are typed from the decimal policy"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2056: Decimal subjects are typed from the decimal policy

## Description

Verify that the FR-322 step 4 decimal subject row types a member, a tuple
position and a variant payload whose filament-core-data resolution walk
passes a decimal policy. The type is `Decimal[lo, hi; s, s; exact]`, its
bounds narrowed by every bound on the walk, and it names the same
`decimal_range` node as the equal source form. Also verify that a
`target`-bound Decimal value type, which carries no policy, keeps no type.
Scope: FR-322-AC-478, and the admitted Decimal controls of FR-322-AC-181,
FR-322-AC-182 and FR-322-AC-183.

## Golden fixture

Domain package `test/decimal` (semantic IR 2.0.0) with these types and fields:

- Value types `Soc`, `Price` and `Margin`, each with `scalar` `decimal`:
  - `Soc`: policy `(5, 2)`, `min` `"0"`, `max` `"1"`.
  - `Price`: policy `(10, 2)`.
  - `Margin`: policy `(4, 1)`, `exclusiveMin` `"-1"`, `exclusiveMax` `"1"`.
- Aliases:
  - `Rate`: kind `alias`, target `ix://filament/native/Decimal`, policy `(7, 4)`, `min` `"0.0001"`, `max` `"1"`.
  - `Half`: kind `alias`, target `Soc`, `max` `"0.5"`.
- Value type `Amount`, with `target` `ix://filament/native/Decimal` and no policy.
- Object type `Meter` with one field typed by each of `Soc`, `Price`,
  `Margin`, `Rate`, `Half` and `Amount`, and a field `fee` typed
  `ix://filament/native/Decimal` with the field policy `(6, 2)`.
- Operation `adjust(by)`: parameter `by` typed `ix://filament/native/Decimal`
  with policy `(5, 2)`, result typed `Soc`.
- Tuple type `P` with position `p0` typed `Soc`.
- Variant type `V` with case `c` whose payload is `Rate`.

FSL keeps the matching byte fixture at
`tests/fixtures/decimal-value-types.semantic-ir.json` (FSL FR-056 "Decimal
value types").

## Test Procedure

1. Read the package and build the checked package of a unit that selects it
   and holds the invariant `self.soc >= 0.20` on `Meter`, plus a function whose
   parameter is typed `Decimal[0, 100; 2, 2; exact]`.
2. Read each member's, position's and payload's type node.
3. Read the `Amount` field, and apply `self.amount = self.amount`.

## Expected Results

- Step 2: `Soc` members, the result of `adjust` and `p0` name one
  `bounded_domain`/`decimal_range` node. Its bounds are `0` and `100`, it has
  `scale_min` `2`, `scale_max` `2` and rounding `exact`, and it is the node the
  function parameter's annotation names.
- Step 2: `Price` is `Decimal[-9999999999, 9999999999; 2, 2; exact]`, and
  `Margin` is `Decimal[-9, 9; 1, 1; exact]`.
- Step 2: `Rate`, including the payload of `c`, is
  `Decimal[1, 10000; 4, 4; exact]`, and `Half` is `Decimal[0, 50; 2, 2; exact]`
  (a different node).
- Step 2: `fee` is `Decimal[-999999, 999999; 2, 2; exact]`, and `by` is
  `Decimal[-99999, 99999; 2, 2; exact]`.
- Step 3: `amount` has no type, and the clause refuses
  `ill_typed`/`operator-ineligible`.
