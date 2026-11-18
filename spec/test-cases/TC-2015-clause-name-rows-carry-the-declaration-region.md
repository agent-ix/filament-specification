---
id: TC-2015
title: "Clause name rows carry the whole declaration region"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2015: Clause name rows carry the whole declaration region

## Description

Verify that each `temporal_clause_names` and `claim_names` row carries the
whole source declaration's region, that the region is provenance only, and
that the reader checks it against the occurrence's source-map region. Scope:
FR-322-AC-460 and FR-322-AC-461.

## Test Procedure

1. Compile a unit declaring the temporal clause `Bounded` and the invariant
   `NotAboveThree`, and read both rows' `declaration_region`.
2. Compile the unit with `Bounded` declared last and whitespace added right
   before `Bounded`'s closing `}`.
3. Read the package of step 1 with a row's `declaration_region` removed,
   shrunk to a byte outside the name token, or naming another source.
4. Replay `claim-name-vectors.json` cases `CL-P1`, `CL-P6`, `CL-S4` to
   `CL-S6` and `CL-R10` to `CL-R13`.

## Expected Results

- Step 1: each region's bytes are the whole declaration, keyword to closing
  `}`, and contain the name token's region.
- Step 2: no `node_id` and no node changes.
- Step 3: `malformed_wire` at the row for the removed member; `invalid_package`/
  `invalid-value` at the row's `declaration_region` for the other two.
