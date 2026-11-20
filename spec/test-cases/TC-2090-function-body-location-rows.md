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
location of every term of a function's body walk, that the strict reader
admits a complete set of rows and keeps each captured path, and that it
refuses each defect at its own locus, and that a source-model clause
function's rows carry no check-time package in their `name`. Scope:
FR-322-AC-490, FR-322-AC-494.

## Test Procedure

1. Validate the BL-01 vector of `checked-term-data-vectors.json`
   (`function f(p: Float64[nearest-even], q: Float64[nearest-even]): Boolean pure { sqrt(p) <= sqrt(q) }`)
   against the schema, admit it with the strict reader, and read the rows of
   the two `sqrt` applications through the typed read.
2. Admit BL-02 (BL-01 with the second `sqrt`'s `path` set to `[0]`) and
   compare its node ids, `name` and that row's typed path with BL-01's.
3. Admit each reader-refusal vector BL-R1 to BL-R6.
4. Admit BL-03, then BL-03 with its occurrence-1 `+` row removed.
5. Validate a `checked_term_data` object with the four other arrays and no
   `body_locations`, and admit BL-01 with the whole carrier removed.
6. Emit FSL FR-531-AC-40's unit (a precondition clause function of an
   operation of a source object type, `SourceOwner` identity `I`) and read
   every `body_locations` row of that clause function.

Test: `tests/checked_package_v2.rs` `tc_2090_function_body_location_rows`
(steps 1 to 5); FSL `tests/it/dispatch_emission.rs` (step 6).

## Expected Results

- Step 1: BL-01 is schema-valid and admitted; every row's origin is
  `{kind: "function_body", function: <f>, name: "f", index: 0}`; the first
  `sqrt` row's path is `[0]` and the second's is `[1]`, not `[1, 0]`.
- Step 2: admitted; equal node ids and `name`; the changed row reads `[0]`.
- Step 3: each refuses `invalid_package`/`invalid-value`: BL-R1 at
  `/checked_term_data/body_locations`; BL-R2 at the row's `application`;
  BL-R3 at the last row's `name`; BL-R4 at the last row's `index`; BL-R5 at
  the row's `function`; BL-R6 at the added row's `application`.
- Step 4: BL-03 is admitted with two rows for its one `+` node, at its `expression` occurrences 0 and 1, each `term_ordinal` 0, paths `[0]` and `[1]`; without the occurrence-1 row it refuses at `/checked_term_data/body_locations`.
- Step 5: the object is `malformed_wire`; the carrier-less package refuses
  `invalid_package`/`invalid-value` at `/checked_term_data/body_locations`.
- Step 6: every row's `name` begins `ix://I/`; none begins `ix://source/`.
