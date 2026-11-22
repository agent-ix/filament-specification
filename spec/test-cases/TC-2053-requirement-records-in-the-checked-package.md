---
id: TC-2053
title: "Requirement records in the checked package"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-825
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2053: Requirement records in the checked package

## Description

Verify emission, reading, identity and reader refusals of the `requirement_items` rows and the `claim`/`value_validity` record nodes, and the place of the requirement-record step in the reader's check order.

Scope: FR-825-AC-1 to FR-825-AC-7 and FR-322-AC-473. The published
[requirement-record-vectors.json](../../proposals/checked-package-v2/requirement-record-vectors.json)
cases RRV-01 and RRV-02 are the reader-input recipes for AC-6 and AC-7.

## Test Procedure

1. Emit and read FR-825-AC-1's `g`/`h`/`f` unit; compare each record node's body and path with the AC (FR-825-AC-1).
2. Emit FR-825-AC-2's guarded unit; establish the package order from the three application nodes' structural keys and compare the rows; rewrite `n < 10` as `n < 11` and compare the three record `node_id`s (FR-825-AC-2).
3. Emit FR-620's Counter unit with `inc` added; establish the package order from the rows' keys, occurrences and kinds and compare; take the typed read for `inc`; emit a unit with no requirement record (FR-825-AC-3).
4. Apply each mutation of FR-825-AC-4 to step 1's package.
5. Apply each mutation of FR-825-AC-5 to the packages of steps 1 to 3, and the budget one visit short to step 1's package; read a package whose only `claim` nodes are `value_validity` nodes (FR-825-AC-5).
6. Build FR-322-AC-473's package with both defects, then with the record defect and the reversed `probabilistic_claims` row; add a `claim_names` row naming a `value_validity` node; emit FR-825-AC-1's unit and compare it node by node with the golden package written without records, then with a producer double that rewrites the `+` node's path (FR-322-AC-473).
7. Build RRV-01 from its source and graph-role declarations. Establish that both scalar application occurrences use the declared Boolean `value`/`parameter` guard `p`, with one condition-read occurrence and `holds` true and false, respectively. Confirm both records admit and their identity paths include the guard node, read occurrence and branch outcome (FR-825-AC-6).
8. Apply RRV-02's mutation to RRV-01: replace one record's guard with the owner-reachable integer `value`/`parameter` node `x`, then rebuild the record identity and every reference to that record. Confirm the reader refuses at the changed guard with `invalid_package`/`invalid-value` (FR-825-AC-7).

## Expected Results

Each step gives the rows, nodes, paths, typed reads, identities and refusals (code, cause and JSON pointer) its AC states. The local qualification test checks the published RRV-01/RRV-02 recipes and their trace links; it does not execute the strict reader.
