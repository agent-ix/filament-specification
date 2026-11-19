---
id: TC-2038
title: "The synthesis request selects one declaration and writes its outcome"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-823
    type: verifies
---

# TC-2038: The synthesis request selects one declaration and writes its outcome

## Description

Verify the synthesis request's members, its declaration selection and the
`filament.synthesis-outcome/v1` document for each status. Scope:
FR-823-AC-1 to FR-823-AC-5.

## Test Procedure

1. Request `Pick` with `Pick * Pick = 49`, `= 50`, and `minimize Pick` with
   `Pick >= 3`.
2. Request `= 50` with `max_candidates` 3, and with `limits` `{"work": 1}`
   and `{"max_work": 0}`.
3. Omit `declaration` over a package holding only `Pick`, then over one
   holding `Pick` and `Reach`; name `Nope`, the invariant `Positive`, a bare
   `Pick` declared as `A::Pick` and `B::Pick`, and `A::Pick`.
4. Select `Pick` with `initial`, `policy` and `dwell_grid`; select `Reach`
   without `initial`; send `engines`.
5. Inject a checker fault through the library test seam.

## Expected Results

- Step 1: `candidate` 7 `validated`, `visited` 8, exit 0; `no-candidate`,
  `visited` 10, exit 10; candidate 3 `optimal`, `visited` 4; every member
  present in each.
- Step 2: `incomplete` naming `synthesis.max_candidates`, exit 22;
  `unknown-member` at `/limits/work`; `invalid-value` at `/limits/max_work`.
- Step 3: `Pick` runs; `missing-member` at `/declaration` naming both;
  `missing-name` at `/declaration` (naming `state_clause` for `Positive`);
  `ambiguous-name` at `/declaration`; `A::Pick` runs.
- Step 4: `unknown-member` at `/initial`, `/policy`, `/dwell_grid` and
  `/engines`; `missing-member` at `/initial`.
- Step 5: `check` `rejected` with counterexample and lineage, exit 30.
