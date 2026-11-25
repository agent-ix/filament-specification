---
id: SR-2026112515500001
title: "Code review: PR 846 owner-clause source_map and derived-item ids"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; git diff 7de9a209...500439ee: proposals/backend-provider-v1/vectors.json, proposals/checked-package-v2/fixtures/{positive-all-families,positive-clause-operations,positive-tick-timed-model-nodes,positive-timed-clause-nodes,positive-timed-model-nodes,positive-timed-slot-nodes,protocol-correlation-dependency,protocol-node-base}.json, tests/backend_provider_v1.rs"
review_set: subset
---

## Summary

Ticket: STD-730 (PR agent-ix/filament-specification#846, branch titan/std-730-fixture-source-map, head 500439ee). FSpec #843 appended the owner-model operand segment to every `protocol_clause` node id but left the document-held copies of those ids (the clause's `source_map` entry in eight fixtures, and the claim and infinite-trace domain nodes of `derived-items-request` items 5 and 6) on the old two-segment path. This PR appends the index segment `{"node": i}` to each copy. The rust-review lane is folded in: the only Rust change is one expected literal in `tc_431_a_derived_item_is_keyed_by_its_anchor_node_and_kind`.

Independent checks, all by jq against the worktree, with no cargo run:

- For each of the eight fixtures, the `protocol_clause` node's own path ends in `{"operand": k}`, `operand_rows[i][k]` equals the graph index of the node its `owner_model` names, and the changed `source_map` entry's `{"node": n}` equals that index: all-families 44, clause-operations 21, tick-timed-model 47, timed-clause/timed-model/timed-slot 53, correlation-dependency and node-base 14 (FR-322 "Operand index segments").
- Every `source_map` entry of every embedded package in all 27 JSON files that carry one resolves to a node id after the operand-to-index view. The same check on origin/main's positive-all-families reports the stale entry, so it is not vacuous.
- The `derived-items-request` request carries no package: the test resolves it against `positive-all-families`, whose clause is node 44. Items 5 and 6 claim and domain nodes now read `["Example","Handshake",{"node":44}]`.
- No fixture digest or manifest hashes these fixtures; the mutation manifest patches no changed `source_map` entry.
- The test edit replaces only the expected literal `handshake` with the new id. Every assertion stays, and `anchor_of` still resolves the claim against the package's nodes and checks tag, form and occurrence, so the test is not weakened.
- Logs: baseline (main 7de9a209 + this PR's test filter) has 9 red tests, fixed has 4. The 5 that went green are the ones that read these ids. The 4 still red fail with byte-identical messages before and after: `tc_431_refinement_failure_wire_kinds_are_closed` (12 kinds vs 9), `tc_431_adverse_vectors_are_refused` (`limit-reached-fixed-sample-preflight` changes 4 members), `tc_2090_function_body_location_rows` (BL-06), `tc_233_no_published_node_id_nests_an_operand_node_id` (integer-containment IC-21 operand rows). None of their inputs (refinement-failure vectors, vectors.json adverse limit-reached, checked-term-data body_locations, integer-containment) is touched by #843 or this PR, and none holds a `protocol_clause` node. They are pre-existing and not caused by either PR.

## Verdict

**PASS (mergeable)**. Every changed id equals its node's real id, and no id made stale by #843 remains in the tree. One pre-existing defect outside the diff, from #781 and not caused by #843, is recorded below. It should get its own ticket and does not block this PR.

## Assurance Context

No applicable AssuranceProfile was found under `spec/`.

## Methods and gates

code-review with the rust-review lane. No cargo: the author's logs `STD-730-baseline.log`, `STD-730-fixed.log`, `STD-730-fixture-proof.log` and `STD-730-quire-validate.log` were read. jq resolution checks over the worktree.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Pre-existing, outside the diff: `loop-proof-vectors.json` `positive[0]` (`loop-proof-items-request`) item 1's `claim.node` and infinite-trace domain node are `{package: "example", path: ["Example","Handshake"], ordinal: 0}`. The vector's description builds it on protocol-node-base, which has no Handshake node at all; its clause is `["Example","OrderFlow",{"node":14}]`. The vector was introduced in #781 (2026-11-20), before #843, so it is a different cause. `tc_431_loop_items_follow_their_proof_bearing_repeat_anchors` passes over it because no test resolves item 1's claim. File it as its own ticket with a failing test; do not fold it into this PR. | proposals/backend-provider-v1/loop-proof-vectors.json positive[0].document.request.items[1] |

## Scope

- 8 fixture `source_map` clause entries vs node/operand_rows/owner_model (examined, clean)
- vectors.json derived-items-request items 5/6 claim and domain nodes (examined, clean)
- tests/backend_provider_v1.rs:2413 `handshake` literal and surrounding assertions (examined, clean)
- tree-wide sweep for two-segment Handshake/OrderFlow ids and unresolved source_map ids (examined; FND-001)
- fixture digests and manifests (examined, clean)
- baseline vs fixed logs (examined, clean)
