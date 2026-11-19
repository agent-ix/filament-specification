---
id: TC-2048
title: "A value type bound by target is its native"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-154
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2048: A value type bound by target is its native

## Description

Verify that intake admits a value type bound to a native by `target`, with
the clock and constraint rules of FR-154, and that FR-322 step 4 gives it its
native's row. Scope: FR-154-AC-51 and FR-322-AC-469.

## Test Procedure

1. Admit a domain package with value types bound by `target` to `Boolean`,
   `Integer`, `String`, `Float64`, `Float32`, `Text`, `Rational`, `Decimal`
   and `Clock`, each typing a field; add the constraints and option value
   type of FR-154-AC-51; add a `target` outside the native set and a value
   type with both `target` and `scalar`.
2. Read the selected document's member types; read `memo`; type a tuple
   position by `Ratio`; read the `Watch` field and a reference to `Watch`.

## Expected Results

- Step 1: admitted with the clock members and the `max` constraint;
  `malformed-declaration` at `Constraint.keyword` for `maxLength` on `Watch`
  and `min` on `Count`; the option of `Watch` refused as the option of
  `Clock`; `malformed-declaration` at `TypeDefinition.target` and at
  `TypeDefinition.scalar`.
- Step 2: the natives' element types and nodes; `memo` refuses
  `ill_typed`/`operator-ineligible`; the `Ratio` position refuses
  `malformed-declaration`; `Watch` is a `Clock` member and the reference is
  `Reference<Clock>`.
