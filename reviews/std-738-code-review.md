---
id: SR-202611210022442313144108572
title: "Code review: PR 854 protocol rows C10 and C7 owner operand"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; git diff origin/main...bfed8537: proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json (row C10), proposals/checked-package-v2/fixtures/protocol-node-layout-rows.json (row C7)"
review_set: subset
---

## Summary

Ticket: STD-738 (PR agent-ix/filament-specification#854, head bfed8537681f49dcb64230e046affb1440a7dbb8, branch base a06642a6). The diff is two JSON fixture rows; no Rust source, spec file, schema or test changed, so the rust-review lane has nothing to read beyond the tests that load the two files.

Owning requirements, read at the head:

- FR-322 "Protocol clause owner identity": a `protocol_clause` with `owner_model` has the path `declaration.qualified_name` followed by exactly one `{"operand": k}`. The ordinary first-occurrence rule assigns `k`: it reuses a row entry already holding the owner's key, or takes the next slot and appends the owner's graph index.
- FR-322 "Locality": a row is numbered by first occurrence when node `i` is read in RFC 8785 order (member names sorted), has one entry per distinct slot, and its entries are pairwise distinct.
- FR-322-AC-538: the final slot resolves to the `owner_model` target, otherwise `invalid_package`/`stale-node-key` at `/semantic_graph/nodes/{i}/owner_model`.
- FR-464 "Reader order" step 6: the owner-model edge is the first check of a clause, before its other references. A stale owner slot therefore hides the C10 `requirements` refusal.

Base (`protocol-node-base.json`, unchanged): node 82 path `["Example","OrderFlow",{"operand":8}]`, row `[2,1,17,9,21,19,57,0,14]`; node 14 is `model/object_type/ix://acme/orders/Order`, the `owner_model` value and dependency 0. In RFC 8785 order the body reads slots 0 to 7, `dependencies` then reuses slots it has already read, and `node_id` reads slot 8 last.

C10: the edit retargets `requirements` to a `state`/`state_clause` key whose segments use slots 5, 6, 7 and 8, with row entries 20, 47, 3 and 0. Slot 8 is therefore node 0, the Boolean scalar type, and node 14 was in no slot. Index 14 is not in the row, so by the slot rule the owner takes the next slot, 9, and the row gets entry 14. That is the head's `[2,1,17,9,21,20,47,3,0,14]` with path operand 9. The `owner_model` member and dependency 0 are unchanged, so the edge, the dependency and the slot all name node 14.

C7: the edit replaces `over` with a literal and renumbers the remaining slots 0 to 6 in the body. Dependency 15 (the `view` parameter key) introduces slot 7 (node 17). In RFC 8785 order `dependencies` comes before `node_id`, so the owner's path slot 8 is the next slot. The head appends 14 as slot 8, and the path already names `{"operand":8}`. The row on main had 8 entries, so slot 8 was outside it. The row's expectation is schema-stage, which a reader slot does not change.

Sibling check (jq, read-only, over the worktree files): each edit list was applied to the base, and every `protocol_clause` was checked for first-occurrence numbering in RFC 8785 order, row length equal to the slot count, distinct entries, and an owner slot that resolves to the `owner_model` node.

- Main: manifest 85 rows, defects in C10 (owner slot is node 0) and OM-03; layout 34 rows, defect in C7 (no row entry for slot 8).
- Head: manifest, only OM-03, which removes `owner_model` on purpose and is refused at the schema stage; layout, none.
- The 69 edit lists of `protocol-correlation-vectors.json` on the same base: none.
- The unedited `protocol_clause` nodes of the other checked-package fixtures (`positive-*.json`, `protocol-correlation-dependency.json`): none.
- The manifest's `later` and `pending` entries carry no edits.

No other row has the same staleness.

The test `appended_fence_event_and_await_nodes_are_schema_valid` reads only row L12, the base and the schema. It is unchanged by this diff, and its cause is outside this PR. Its `event` case binds `related` to `effect[1]`, which is the effect node's `binder` reference, and `constraint` to `effect[2]`, which is the `related` aggregate. Node 67's arguments are `of, binder, related, constraint` in every revision of the base. Since the change that made `related` of receive/attempt/effect/event a `ProtocolRecordsTerm` (an aggregate only), a reference there is schema-invalid. Main has the same bytes for every input the test reads, so it is red on main as well.

Tests run at the head, name-filtered: `protocol_node_mutation_manifest`, all 12 tests passed; `protocol_node_layout_rows`, all 4 ran, 3 passed and `appended_fence_event_and_await_nodes_are_schema_valid` failed ("event is schema-valid", cause above); `published_files` `published_file_returns_every_proposal_byte_and_lists_fixture_paths_in_order` passed; `checked_package_v2` `tc_233_no_published_node_id_nests_an_operand_node_id` passed. `layout_vectors_are_refused_by_the_schema_at_their_first_error_path`, which checks C7, passes.

## Verdict

**PASS (mergeable)**. Both rows now follow the FR-322 slot rule and FR-322-AC-538, the expected outcomes are unchanged, and no sibling row carries the same defect. The failing layout test is not caused by this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

- protocol-node-mutation-manifest.json row C10 edits, expect and resulting node 82 (examined)
- protocol-node-layout-rows.json row C7 edits, expect and resulting node 82 (examined)
- All other rows of both files, the correlation-vector edit lists, and the protocol clauses of the other checked-package fixtures, by the sibling check above (examined)
- FR-322 "Protocol clause owner identity", "Locality", FR-322-AC-538, FR-322-AC-130 (examined)
- FR-464 "Reader order" step 6 and "Owner-model edge phase" (context only)
- schema.json `event` layout and `ProtocolRecordsTerm`, tests/protocol_node_layout_rows.rs (context only)
