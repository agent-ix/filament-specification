---
id: TC-1280
title: "Unit header integer-division law selection"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-570
    type: verifies
---

# TC-1280: Unit header integer-division law selection

## Description

Select, refuse and scope the integer-division law a unit header names.

Scope: FR-570-AC-1 to FR-570-AC-5.

## Test Procedure

1. For each law, compile a complete-value unit whose header names the law
   and that declares `a div b` and `a rem b` over `a: Int[-7, 7]`,
   `b: Int[1, 3]`; read its package selection; evaluate at `(-7, 3)`.
2. Remove the law header; then replace the body with `a mod b` only.
3. Use two law headers naming floor and truncating; then two naming floor;
   then one floor header with a supplied truncating alternative role.
4. Declare a function `using` the law header's alias.
5. Compile a floor-header unit with no `div` or `rem` and the same unit
   without the header; compile a floor unit importing a truncating library
   and evaluate each unit's own `-7 div 3`.

## Expected Results

- Step 1: admitted; exactly that law's role; truncating `(-2, -1)`, floor
  `(-3, 2)`, Euclidean `(-3, 2)`.
- Step 2: `missing_declaration`/`missing-selection` at each `div` and `rem`;
  the `mod`-only unit is admitted with no law.
- Step 3: `invalid_package`/`conflicting-definition` at the second identity
  literal; floor selected once; `invalid_package`/`conflicting-definition` at
  the header's identity literal.
- Step 4: `unknown_profile`/`wrong-selection-role` at the alias.
- Step 5: no integer-division role and equal package identities; `-3` for
  the importing unit and `-2` for the library.
