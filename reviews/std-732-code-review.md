---
id: SR-85911
title: "Code review: PR 848 stale conformance vectors and reader tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; git diff 53fe180b...42b156d7: tests/backend_provider_v1.rs, tests/checked_package_v2.rs, proposals/backend-provider-v1/refinement-failure-vectors.json, proposals/backend-provider-v1/vectors.json, proposals/checked-package-v2/integer-containment-vectors.json"
review_set: subset
---

## Summary

Ticket: STD-732 (with STD-733, STD-734, STD-735, STD-736; PR agent-ix/filament-specification#848, branch titan/fspec-red-main, head 42b156d7). The rust-review lane is folded in. No cargo was run; the author's logs `STD-fspec-red-*.log` in the 2026-11-24 handoff `gates-lead-conf` directory were read, and the vectors were checked with jq against the worktree.

Per fix, which side the requirement makes right:

- STD-732: FR-379-AC-14 says the `RefinementFailure` definition admits exactly twelve kinds. The old test asserted nine; the schema and the twelve positive cases were right. The new test now also reads the schema's `oneOf` branches and asserts the exact twelve-name set, with one branch per kind, and asserts the positive cases cover the same twelve. This makes the test stronger.
- STD-733: FR-331-AC-67 and TC-431 BP-68 say the fixed-N cause with `required: "921034100000000000000000000000000000000"` is admitted. On main it was an adverse vector with four patches, which broke the one-member rule. It is now a positive `outcome` vector, and the four adverse vectors (zero, numeric, leading-zero, signed) patch only `required` over it. The new document equals the old base plus patches, except that `accounting.limits` and `reached` now name `max_samples` instead of `work_units`. That change matches the cause's own limit (FR-408-AC-19, FR-461). Owners are `/results/0/cause` FR-408 and `/results/0/cause/required` FR-331.
- STD-734: FR-322 "body walk" says the producer and reader SHALL descend through `value`/`record_value` and `value`/`tuple_value` nodes. The test's decider stopped at every `value` node, so BL-06 failed. The decider is the stale side. The new assertions take their literal values from FR-322-AC-513 and the FR-322-AC-513 BL-06 paragraph: 18 rows, paired term ordinals, paths `[c]`, `[c,0]`, `[c,1]`, the deltas `(1,[1,0])` and `(1,[1])`, BL-R11 = BL-04 with row 16 `{up:2, down:[0,1]}` at `/16/location/up`, and BL-06 `(0,[0,0])`, `(2,[1,0])`.
- STD-735: FR-322-AC-392 requires one `operand_rows` row per node. IC-21 had 13 rows for 12 nodes, and the fix drops one trailing empty row. Rows 2 and 5 carry `[1]` for their `{"operand":0}` id segment. The other `[1]` rows hold body references, and the slot-view round trip in tc_233 passes for the file.
- STD-736: FR-379-AC-13 and AC-14 say every wire `position` is a canonical u64 decimal string and a numeric one is malformed. Product `expected` values were already strings, so the evaluator was stale. `wire_position` now stringifies the product's internal prefix count. The 11 replay `recorded` positions are now strings, and each is now validated against `RefinementFailure`. `TC318-9-wrong-position` still records `"0"` against product position 1, so it still checks replay-parity. The 15 product `position` members stay numeric because they are fixture inputs, not wire values.

Remaining red (STD-737): the final log on 42b156d7 shows `tc_233_no_published_node_id_nests_an_operand_node_id` failing only at the `nested.is_empty()` assertion. Every package passes the operand-refusal and slot-view checks first. All 313 listed paths are in `model-union-witness-vectors.json` and are `node_key`, `type_handle`, `settled_key` and `declaration` nests (positive/16, schema_refusals 10, 13 and 18). Those are FR-830 original node keys of the `{package,path,ordinal}` shape, so the cause is only the FR-322-AC-278 vs FR-830-AC-6 conflict that STD-737 records.

Clippy: the same 23 findings in checked_package_v2 before and after, at shifted line numbers only. The diff adds none.

## Verdict

**PASS (mergeable)**. No assertion was weakened to match a fixture. Each changed side is the one the requirement text makes stale, and the expected values are literals from the ACs. One low finding is a missing BL-06 structural assertion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The BL-06 and BL-05 assertions skip part of what FR-322-AC-513 states. They do not check that tuple node 6 references record nodes 3 and 4 in positions 0 and 1, that `f`'s (node 7) body references node 6, or that the subtraction's operands are the literals `0` and `1`. For BL-05 they do not check the tuple's position order. Only BL-04's field order `z`, `a` is asserted. The admitted rows and paths imply this structure, and the vector holds it (checked by jq), but a vector edit that changed the shape while keeping the row paths would go unnoticed. | tests/checked_package_v2.rs:25838-25922 |

## Scope

- tests/backend_provider_v1.rs tc_431_refinement_failure_wire_kinds_are_closed (examined)
- tests/backend_provider_v1.rs wire_position, relation_failure, replay_relation_failure, tc_318 replay validation (examined)
- tests/checked_package_v2.rs is_composite_value, decide_body_locations walk, tc_2090 BL-04/05/06/R11 block (examined; FND-001)
- refinement-failure-vectors.json replays recorded positions (examined)
- vectors.json limit-reached-fixed-sample-preflight positive and its four adverse dependants (examined)
- integer-containment-vectors.json IC-21 operand_rows (examined)
- tc_233_no_published_node_id_nests_an_operand_node_id final failure output (examined)
- author's baseline, fix and final logs, clippy logs (examined)

## Dispositions

Round 1, head 25fdcd07 (fix commits 6f9aec98, 6a988419, 75a430ef, 7d363619, 25fdcd07). The new scope was also reviewed in this round. STD-737 (6f9aec98): `is_original_node_key_position` excludes a nested key only when the file is `model-union-witness-vectors.json`, the segment has the key shape, and the holder sits at `model_tuples/*/node_key`, `model_unions/*/type_handle|settled_key`, `cases/*/member/declaration` or `cases/*/payload/original_type`, in a positive row or at a refusal patch's target. These are exactly the FR-830 original-key positions (FR-830 "original node key", line 71), and a jq listing of every key-shaped object in the file finds no other position that needs it. The injected checks show `structural_node`, payload `node` and a refusal patch retargeted to `structural_node` are still reported. STD-731 (6a988419): in `protocol-node-base.json` the 24 nodes with a `protocol_profile` law are 59 to 82. Under the FR-322 node order (package, then path, a string before an operand), node 62 `["Example","OrderFlow","channel","Charges"]` sorts first: `channel` comes before `compensate`, `finish`, `role`, `run` and `terminal`; `protocol` sorts after `Example`; and clause 82 ends in an operand. So `/semantic_graph/nodes/62` is right and the `reader_order` prose now matches. Logs r2 on 25fdcd07: the focused tests pass 38 + 4, the manifest/id tests pass, quire passes, clippy still has 23 findings, and contract-ir be92e1c3 `protocol_mutation_manifest::` fails only `earlier_protocol_law_refusal_precedes_missing_temporal_selection`, whose 82 is hardcoded (known, carried separately).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 75a430ef asserts BL-04/BL-05 positions share BL-01's `let` node and f's body is the composite, and BL-06's literal operands, record bindings `n`, tuple members `[ref 3, ref 4]` and node 7 body `ref 6`. |
