---
id: SR-85913
title: "Gap analysis: PR 848 criteria of the changed tests and vectors"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-specification; diff-limited, 53fe180b...42b156d7: criteria FR-379-AC-13, FR-379-AC-14, FR-331-AC-67, FR-322-AC-392, FR-322-AC-278, FR-322-AC-513"
review_set: subset
---

## Summary

Ticket: STD-732 (PR agent-ix/filament-specification#848). The caller limited this to the diff. `quire trace --scope . --id <AC>` was run for each criterion the diff touches. There is no production code in this repository; the tests' deciders are the executable statement. There are no stubs in the diff: every new helper (`is_composite_value`, `wire_position`) is used and asserts real values.

Trace results on 42b156d7:

- FR-379-AC-14: tc_431_refinement_failure_wire_kinds_are_closed (verifies).
- FR-379-AC-13: tc_318_protocol_relation_failure_vectors (verifies).
- FR-322-AC-513: tc_2090_function_body_location_rows (verifies). The tag is new in this PR and matches the assertions.
- FR-322-AC-392 and FR-322-AC-278: tc_233_no_published_node_id_nests_an_operand_node_id and tc_233_node_index_vectors_refuse_at_their_loci. The first stays red for STD-737 only.
- FR-331-AC-67: no claims (`resolved: false`). The BP-68 vectors that STD-733 fixes are asserted by tc_431_positive_vectors_are_admitted_with_owned_members and tc_431_adverse_vectors_are_refused. Those are tagged FR-331-AC-66, not AC-67.

## Verdict

**FAIL** under the skill's rule (an untagged criterion in scope). The gap predates this PR, and closing it takes a tag-only commit in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-331-AC-67 has no trace-tagged test. The PR cites it as the criterion of the STD-733 fix, and BP-68 is asserted by tc_431_positive_vectors_are_admitted_with_owned_members and tc_431_adverse_vectors_are_refused, but neither carries `#[trace("TC-431", "FR-331-AC-67")]`. The gap predates the PR; add the tag to both in this PR. | tests/backend_provider_v1.rs:533, tests/backend_provider_v1.rs:771 |

## Coverage

- Criteria in scope: 6. Tagged: 5. Untagged: 1 (FR-331-AC-67).
- Red after PR: 1 test (tc_233_no_published_node_id_nests_an_operand_node_id), owned by STD-737.
- Semantic review: skipped as a separate pass. The code-review (SR-85911) checked intent against test for each changed assertion.
- Plan completion: not assessed

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The new FR-322-AC-544 says Test (TC-2090), and TC-2090 step 10 names only FSL `a_source_operation_declares_its_effective_precondition_function` "for the function's declared name". That test asserts only that some node is declared `["Order","measure","effective_precondition"]`. It checks neither the candidate's `effective_precondition` handle nor that the node differs from the authored precondition function, and no test anywhere carries a trace tag for FR-322-AC-544. The handle part is asserted by the FSL twin test in open PR #3963 under FR-569-AC-6. Name that test in TC-2090's Test line, or add an FSpec-side check. | spec/test-cases/TC-2090-function-body-location-rows.md:44-49 |

## Dispositions

Round 1, head 25fdcd07.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 7d363619 adds `tc_431_fixed_sample_preflight_required_is_an_exact_positive_string`, tagged `#[trace("TC-431", "FR-331-AC-67")]`. It asserts the literal count, the ordinary cause with no `required`, and each of the four spellings refused at `/results/0/cause/required`. A dedicated test replaces the requested tags on the generic tests. |

Round 2, head a3a02703. a3a02703 corrects TC-2090's Test line. Step 10 now names FSL `the_source_and_model_twins_link_equal_tables_and_check_equal_types`, which on the checked graph compares `Order::size`'s source candidate's effective-precondition function by its declared name and finds it differs from the one contributing clause function. It also names `a_source_operation_declares_its_effective_precondition_function` for the declared name. The Test line states that neither test reads the emitted candidate's handle against the emitted declarations. The twin test's new form is only in open FSL PR #3963 (head 25173079); on FSL main it is the old, red version. What remains: no test checks AC-544's wire claim (the emitted `dispatch_tables` candidate's `effective_precondition` NodeRef resolves to the node declared `[<type>, <operation>, "effective_precondition"]`, distinct from the `precondition` clause node), and no test carries an FR-322-AC-544 tag. The contract-ir reader refuses the reuse case at admission (`dispatch_projection.rs`, `Route::Source`), but that is a reader check, not producer evidence. A note alone does not close this. The fix is one assertion in FSL `a_source_operation_declares_its_effective_precondition_function`: read the candidate's `effective_precondition` from the emitted `dispatch_tables`, resolve it, check it is the node declared `[..., "effective_precondition"]` and not the `precondition` clause node, and tag it `FR-322-AC-544`. That belongs in FSL #3963 under QSL-3398 and does not block this FSpec PR.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | deferred | QSL-3398 (FSL PR #3963): add the emitted-handle assertion tagged FR-322-AC-544 to a_source_operation_declares_its_effective_precondition_function; TC-2090 now states the evidence honestly and the evidence lives in FSL. |
