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

Scope: FR-825-AC-1 to FR-825-AC-9 and FR-322-AC-473. The published
[requirement-record-vectors.json](../../proposals/checked-package-v2/requirement-record-vectors.json)
cases RRV-01 through RRV-06 are the reader-input recipes for AC-6 through
AC-9.

## Test Procedure

1. Emit and read FR-825-AC-1's `g`/`h`/`f` unit; compare the frame table, each record's terminal frame id in its body and identity path, and the typed path view with the AC (FR-825-AC-1).
2. Emit FR-825-AC-2's guarded unit; establish package order from the three application nodes' structural keys, compare the parent-linked frame chains and record terminal ids, then rewrite `n < 10` as `n < 11` and compare frame and record identities (FR-825-AC-2).
3. Emit FR-620's Counter unit with `inc` added; establish the package order from the rows' keys, occurrences and kinds and compare; take the typed read for `inc`; emit a unit with no requirement record (FR-825-AC-3).
4. Apply each mutation of FR-825-AC-4 to step 1's package.
5. Apply each mutation of FR-825-AC-5 to the packages of steps 1 to 3, and the budget one visit short to step 1's package; read a package whose only `claim` nodes are `value_validity` nodes (FR-825-AC-5).
6. Build FR-322-AC-473's package with both defects, then with the record defect and the reversed `probabilistic_claims` row; add a `claim_names` row naming a `value_validity` node; emit FR-825-AC-1's unit and compare it node by node with the golden package written without records, then with a producer double that rewrites the `+` node's path (FR-322-AC-473).
7. Build RRV-01 from its source and graph-role declarations. Establish that both scalar application occurrences use one shared Boolean guard occurrence `p`, with separate frame rows for holds true and false. Confirm both records cite only their respective frame ids and the reader independently derives those chains (FR-825-AC-6).
8. Apply RRV-02's bad guard mutation to the frame row: replace the Boolean guard occurrence with the owner-reachable integer parameter `x`; confirm `invalid_package`/`invalid-value` at that row's `guard` (FR-825-AC-7).
9. Starting from AC-8's nested package, apply RRV-03's parent self-reference and RRV-04's duplicate-frame mutation, then the remaining malformed-table mutations: wrong/empty table shape, missing/extra/reordered row members, wrong primitive type, forward/missing/cross-owner parent, and unknown guard node/occurrence. Confirm each first refusal's code and pointer (FR-825-AC-8).
10. Starting from the same package, apply RRV-05's body-inconsistent terminal chain and RRV-06's identity-path-only terminal mismatch, then test wrong frame order or polarity, a missing table, an out-of-range terminal id and an orphan frame. Confirm every refusal code and pointer, including `invalid-semantic-graph` for the node-id mismatch (FR-825-AC-9).

## Expected Results

Each step gives the rows, frames, record paths, shared typed reads, identities and refusals (code, cause and JSON pointer) its AC states. The local qualification test checks all published RRV-01 through RRV-06 recipes and their trace links; it does not execute the strict reader.
