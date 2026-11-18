---
id: TC-2012
title: "Claim and temporal clause names share one namespace"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-459
    type: verifies
---

# TC-2012: Claim and temporal clause names share one namespace

## Description

Verify that the strict package reader refuses a `claim_names` row whose `name` equals a `temporal_clause_names` row's `name`, and admits distinct names that the lookup resolves to one row each.

## Test Procedure

1. Take FR-322-AC-457's package with a temporal clause named `Bounded`, rename the `ReachThree` `claim_names` row to `Bounded` (`qualified_name` `["Bounded"]`) and read it.
2. Repeat with the row's `qualified_name` `["Scope", "Bounded"]`.
3. Read the package with the claim row `ReachThree` and the temporal row `Bounded`, and look up both names.
4. Replay `claim-name-vectors.json` cases `CL-P5`, `CL-R14` and `CL-R15`.

## Expected Results

- Steps 1 and 2: `ambiguous_declaration`/`ambiguous-name` at `/claim_names/{i}/name` of the renamed row, naming both rows.
- Step 3: the package is admitted; `Bounded` resolves to the temporal row and `ReachThree` to the claim row.
