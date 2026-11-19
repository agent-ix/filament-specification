---
id: TC-2035
title: "Rows of one claim declaration share its name"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-465
    type: verifies
---

# TC-2035: Rows of one claim declaration share its name

## Description

Verify that `claim_names` rows with one `declaration_region` are one
declaration. They may repeat its name, they must not carry two names, and
the lookup returns the row on the declaration's own node. Rows of distinct
declarations still may not share a `qualified_name`.

## Test Procedure

1. Take FR-322-AC-457's package. Add a second `claim` occurrence to the
   `AtMostTwo` node, with a row carrying `NotAboveThree`'s name,
   `qualified_name` and `declaration_region`. Read the package and look up
   `NotAboveThree` and `AtMostTwo`.
2. Instead, add a second `claim` occurrence to the `NotAboveThree` node,
   with a row of the same name and `declaration_region`. Read the package
   and look up `NotAboveThree`.
3. Rename step 2's second row `Other` and read the package.
4. To step 1's package, add a second `claim` occurrence to the
   `NotAboveThree` node for a declaration `B` with its own region, and read
   the package.
5. Replay `claim-name-vectors.json` cases `CL-P7`, `CL-P8`, `CL-R5`, `CL-R16`
   and `CL-R17`.

## Expected Results

- Step 1: the package is admitted. `NotAboveThree` resolves to the row on
  its own one-row node, never to the row on the two-row node, and
  `AtMostTwo` resolves to its own row.
- Step 2: the package is admitted, and `NotAboveThree` resolves to its least
  `claim` row.
- Step 3: `invalid_package`/`invalid-value` at that row's
  `/claim_names/{i}/name`.
- Step 4: `invalid_package`/`invalid-value` at `/claim_names/4/declaration`.
- Step 5: every case gives its `expected` outcome, and `CL-R5` is
  `ambiguous_declaration`/`ambiguous-name`.
