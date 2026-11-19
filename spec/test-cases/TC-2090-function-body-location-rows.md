---
id: TC-2090
title: "Function body location rows in checked term data"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2090: Function body location rows in checked term data

## Description

Verify that `checked_term_data.body_locations` carries the checker's
location of every term of a function body walk, that the strict reader
admits a complete set of rows and refuses each defect at its own locus, and
that the rows change neither node ids nor the package `name`.
Scope: FR-322-AC-490.

## Test Procedure

1. Validate the BL-01 vector of `checked-term-data-vectors.json`
   (`function f(p: Float64[nearest-even], q: Float64[nearest-even]): Boolean pure { sqrt(p) <= sqrt(q) }`)
   against the schema and admit it with the strict reader; read the rows of
   the two `sqrt` applications.
2. Admit each reader-refusal vector BL-R1 to BL-R5.
3. Validate a `checked_term_data` object with the four other arrays and no
   `body_locations`.
4. Compare the node ids and `name` of BL-01 with the same package whose
   `body_locations` is emptied.

Test: `tests/checked_package_v2.rs` `tc_2090_function_body_location_rows`.

## Expected Results

- Step 1: BL-01 is schema-valid and admitted; every row's origin is
  `{kind: "function_body", function: <f>, name: "f", index: 0}`; the first
  `sqrt` row's path is `[0]` and the second's is `[1]`, not `[1, 0]`.
- Step 2: BL-R1 refuses `invalid_package`/`invalid-value` at
  `/checked_term_data/body_locations`; BL-R2 at the row's `application`;
  BL-R3 at the row's `name`; BL-R4 at the row's `index`; BL-R5 at the row's
  `function`.
- Step 3: `malformed_wire`.
- Step 4: equal node ids and equal `name`.
