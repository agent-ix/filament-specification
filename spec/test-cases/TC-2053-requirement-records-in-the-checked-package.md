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

Verify emission, reading, identity and reader refusals of the `requirement_items` rows and their `claim`/`value_validity` and `claim`/`operation_contract` record nodes, and the place of the requirement-record step in the reader's check order.

Scope: FR-825-AC-1 to FR-825-AC-11, FR-322-AC-473 and FR-322-AC-522. The published
[requirement-record-vectors.json](../../proposals/checked-package-v2/requirement-record-vectors.json)
cases RRV-01 through RRV-06 are the reader-input recipes for AC-6 through
AC-9.

## Test Procedure

1. Emit and read FR-825-AC-1's `g`/`h`/`f` unit; compare the frame table, each record's terminal frame id in its body and identity path, and the typed path view with the AC (FR-825-AC-1).
2. Emit FR-825-AC-2's guarded unit; establish package order from the three application nodes' structural keys, compare the parent-linked frame chains and record terminal ids, confirm that the false branch with no scalar operation has no frame and is admitted, then rewrite `n < 10` as `n < 11` and compare frame and record identities (FR-825-AC-2).
3. Emit FR-620's Counter unit with `inc` added; establish the package order from the rows' keys, occurrences and kinds and compare; take the typed read for `inc`; emit a unit with no requirement record (FR-825-AC-3).
4. Apply each mutation of FR-825-AC-4 to step 1's package. For the added `declaration` member, confirm the strict schema/owner-presence check refuses first as `malformed_wire` at `/semantic_graph/nodes/{n}`, before the FR-825 record step. Confirm the other record mutations reach the FR-825 rule and match its stated code and pointer; the non-Boolean `holds` mutation fails the strict-wire check at `/path_condition_frames/{f}`.
5. Apply each mutation of FR-825-AC-5 to the packages of steps 1 to 3, and the budget one visit short to step 1's package; read a package whose only `claim` nodes are `value_validity` nodes (FR-825-AC-5).
6. Build FR-322-AC-473's package with both defects, then with the record defect and the reversed `probabilistic_claims` row; add a `claim_names` row naming a `value_validity` node; emit FR-825-AC-1's unit and compare it node by node with the golden package written without records, then with a producer double that rewrites the `+` node's path (FR-322-AC-473).
7. Build RRV-01 from its source and graph-role declarations. Establish that both scalar application occurrences use one shared Boolean guard occurrence `p`, with separate frame rows for holds true and false. Confirm both records cite only their respective frame ids and the reader independently derives those chains (FR-825-AC-6).
8. Apply RRV-02's bad guard mutation to the frame row: replace the Boolean guard occurrence with the owner-reachable integer parameter `x`; confirm `invalid_package`/`invalid-value` at that row's `guard` (FR-825-AC-7).
9. Starting from AC-8's nested package, apply RRV-03's parent self-reference and RRV-04's duplicate-frame mutation, then the remaining malformed-table mutations: wrong/empty table shape, missing/extra/reordered row members, wrong primitive type, forward/missing/cross-owner parent, and unknown guard node/occurrence. Confirm each first refusal's code and pointer (FR-825-AC-8).
10. Starting from the same package, apply RRV-05's body-inconsistent terminal chain and RRV-06's identity-path-only terminal mismatch, then test wrong frame order or polarity, a missing table, an out-of-range terminal id and an orphan frame. Confirm every refusal code and pointer, including `invalid-semantic-graph` for the node-id mismatch (FR-825-AC-9).
11. Emit FR-825-AC-3's `Bounded` operation-contract item. Inspect the `claim`/`operation_contract` node's seven graph bindings, population Group terms, dependencies and operand row; compare its occurrence, extent identity segments, operation anchor, invariant marker and model refs to the source `RequirementRecord`. Then independently mutate each binding and identity segment, remove its row, substitute a value-validity record, reverse population order, alter a population maximum, mismatch its row extent, and add an orphan record node. Verify the first strict-wire/record/row-join refusal and exact pointer, including precedence when the row shape and record body are both malformed (FR-825-AC-3, AC-10; FR-322-AC-473).
12. Emit a package with one structurally shared scalar application reached once in each of two owner bodies, with owner `a`'s declaration source-map position before owner `b`'s. Confirm that the package-wide owner walk assigns the expression occurrences 0 and 1, and each value-validity row names its owner's occurrence. Omit owner `a`'s requirement record while retaining the shared application and owner `b`'s record; confirm the second record remains keyed to occurrence 1 and the reader admits the package. (FR-322-AC-522; FR-825-AC-11).
13. Emit a package where a Boolean parameter is read as an expression before it is used as an `if` guard around a scalar operation. Confirm the guard frame names expression occurrence 1, matching the checker's package-wide owner walk; confirm the reader's package-wide counter derives the same ordinal and guard chain. (FR-322-AC-522; FR-825-AC-11).

## Expected Results

Each step gives the rows, frames, record paths, shared typed reads, identities and refusals (code, cause and JSON pointer) its AC states. The local qualification test checks all published RRV-01 through RRV-06 recipes and their trace links; the contract-record cases additionally exercise the strict reader, package-wide expression ordinals and the new ordered Group encoding.
