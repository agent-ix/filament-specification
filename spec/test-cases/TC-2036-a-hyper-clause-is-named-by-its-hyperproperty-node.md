---
id: TC-2036
title: "A hyper clause is named by its hyperproperty node"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-466
    type: verifies
---

# TC-2036: A hyper clause is named by its hyperproperty node

## Description

Verify that a hyper clause's name is carried by `claim_names` rows of its
`claim`/`hyperproperty` node, that its body wrapper has no
`temporal_clause_names` row, and that a name shared across the two tables is
refused even when both rows carry one region.

## Test Procedure

1. Read the package of a unit declaring the hyper clause `NonInterference`.
   List its `temporal_clause_names` and `claim_names` rows.
2. Add a `temporal_clause_names` row whose `declaration` is the body
   wrapper, and read the package.
3. Replay `claim-name-vectors.json` cases `CL-R14` and `CL-R18`.

## Expected Results

- Step 1: no `temporal_clause_names` row names the wrapper. One `claim_names`
  row per `claim` occurrence of the `hyperproperty` node carries
  `NonInterference`, `qualified_name` `["NonInterference"]`, and the whole
  hyper declaration's `declaration_region`. The package is admitted.
- Step 2: `invalid_package`/`invalid-value` at
  `/temporal_clause_names/{i}/declaration`.
- Step 3: both refuse `ambiguous_declaration`/`ambiguous-name` at
  `/claim_names/0/name`.
