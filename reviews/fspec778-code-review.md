---
id: SR-2026112105041301
title: "Code review: PR 778 code-lane delta (tc_2090 walk-order rebuild, tc_2099 binder link)"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; git diff a34bb5cc..6313e9a5: proposals/checked-package-v2/checked-term-data-vectors.json, tests/checked_package_v2.rs (decide_body_locations, tc_2090_function_body_location_rows, parameter_name_and_level, let_binder_link_refusal, let_binder_containment_refusal, tc_2099_integer_containment)"
review_set: subset
---

## Summary

Ticket: QSL-2691 (PR agent-ix/filament-specification#778, branch draco/spec-batch-4, head 6313e9a5). Scope is the code-lane delta only, commits 8813374b and 6313e9a5; the spec text was reviewed in SR-2026112104184601. Code-review with the rust-review lane folded in. Ran only `tc_2090` and `tc_2099` (own CARGO_TARGET_DIR, flock /tmp/cargo-slot.lock, -j 4); no full gate.

`decide_body_locations` now rebuilds paths in walk order: the k-th walk visit at an application pairs with that application's row of rank k in (`occurrence`, `term_ordinal`) order, the previous path is the paired row of the preceding visit, and a row refuses at `.../{i}/location/up` when `up` exceeds the previous path or `down[0]` equals the entry `up` removed last. That is FR-322 "Pairing rows with visits", "One encoding" and "Reader" as written; the least-form test is exactly the condition for a smaller `up`. BL-R9 (row 8 `{up: 2}` after `[0]`) and BL-R10 (row 8 `{up: 1, down: [0]}` after `[0]`) refuse at that member only through the rebuild. BL-01/02/03 assert both the encoded `{up, down}` and the rebuilt paths; BL-S5/BL-S6 are validated against the `BodyLocation` def; counts 10 and 6 are asserted. Nothing was loosened: the old `== Ok(())` is now `is_ok()` because the decider returns the rebuilt paths, and all three positives then assert those paths.

In tc_2099 the binder-link and binder-containment helpers compute their verdict from the package and compare it with the vector's stated locus; neither returns a hard-coded answer. The five AC-512 mutations of IC-14 are built in the test and each isolates one rule (the `z` binder is added to `dependencies` so only the name rule can fire). The defects are two harness limits.

## Verdict

**CONDITIONAL**: no correctness defect in the delta; two low harness findings. The coverage gaps are in the gap-analysis artifact SR-2026112105041302.

## Assurance Context

No applicable AssuranceProfile was found under `spec/`.

## Methods and gates

code-review with the rust-review lane. Focused tests only: `cargo test -j 4 --test checked_package_v2 -- tc_2090 tc_2099`. Independent Python check of the regenerated vectors, recorded in SR-2026112105041303.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `let_binder_link_refusal` and `let_binder_containment_refusal` resolve node-held ids by byte equality (`node["node_id"] == *id`). That holds only for label-only packages such as the IC set. On an operand-slot package it is wrong (FR-322 "Node bytes are not identity"): in package `f`, let 18's `sqrt` reference holds `{"operand": 3}` where node 17's own id holds `{"operand": 4}`. So the helper cannot be reused for the f/h binder links, and its doc comment does not state the limit. | tests/checked_package_v2.rs:25867, 25925 |
| FND-002 | low | The third binder-link rule ("a reference in the `let` body to a `value`/`parameter` node with the binder's name and level") is checked only on `body.arguments[*].target` of expression nodes reached from the body root. A reference inside a nested binding term's `value` (an inner `let`'s `arguments[0].value.target`) or inside an aggregate member is never inspected, so a stray same-name/same-level read there is admitted. This is enough for IC-14's mutation but does not implement the rule as stated. | tests/checked_package_v2.rs:25886-25903 |

## Scope

- decide_body_locations walk-order rebuild and least-form check (examined, clean)
- tc_2090 BL-01/02/03 encoded and rebuilt-path assertions, BL-R9/R10, BL-S5/S6, counts 10/6 (examined, clean)
- let_binder_link_refusal, let_binder_containment_refusal, parameter_name_and_level (examined; FND-001, FND-002)
- tc_2099 IC-15/IC-21 loci and ineligible list, AC-512 IC-14 mutations (examined, clean)

## Dispositions

Round 1, against 5b70cf1a (delta 6313e9a5..5b70cf1a).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | rejected: not a defect in this harness | `parsed_fixture` runs `operand_slots::index_view` on every vector (tests/checked_package_v2.rs:117-119), so the helpers only ever see index segments `{"node": i}`, and byte equality of ids is identity there. The bytes this review cited came from the raw JSON, which no test passes to the helpers. 5b70cf1a states this precondition in the helper doc comment, and TC-2090 now runs `let_binder_link_refusal` over f and h and it returns `None`. |
| FND-002 | fixed | `references()` now collects every `reference` term anywhere in a reached node's body: call arguments, nested binding `value` and aggregate `members`. It reports the reference's own pointer. tc_2099 adds a nested-binding mutation (`.../arguments/2/value`) and an aggregate-member mutation (`.../arguments/2/members/0`); the old harness admitted both. |
