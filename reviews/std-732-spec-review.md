---
id: SR-85912
title: "Spec review: PR 848 FR-322 clause-function sentence, TC-2090 and changed vectors"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; git diff 53fe180b...42b156d7: spec/objects/interfaces/FR-322-checked-package-artifact.md (Clause functions item 5), spec/test-cases/TC-2090-function-body-location-rows.md, proposals/backend-provider-v1/{vectors,refinement-failure-vectors}.json, proposals/checked-package-v2/integer-containment-vectors.json"
review_set: subset
---

## Summary

Tickets: STD-732 (primary; also STD-733 to STD-736) and QSL-3398. This is a diff-limited spec review. It covers the one-sentence FR-322 change, the TC-2090 `Test:` line and the published vectors, each checked against the schema and its criterion (see SR-85911 for the vector checks).

The FR-322 sentence matches arch ruling c9e092a7 item 3 on QSL-3398. It also matches what the contract-ir reader enforces: `dispatch_projection.rs` requires, on `Route::Source`, that a candidate's effective precondition be a source clause function declared `[<type>, <operation>, "effective_precondition"]`. That is filament-contract-ir origin/main be92e1c3. Nothing else in FR-322 contradicts it. The model route, which reuses the authored function, is left unstated, which is consistent with the ruling.

The TC-2090 `Test:` line now says steps 7 to 9, which matches the assertions added. The changed vectors match the schema and their criteria (FR-379-AC-13 and AC-14, FR-331-AC-67, FR-322-AC-392).

## Verdict

**CONDITIONAL**. The sentence is right. The findings are wording, an unverifiable normative clause, and a ticket-state risk. None blocks the content merge, but FND-003 needs action before or at merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new sentence says "even where a single term (one authored clause) contributes". "Term" here is the FSL checker's internal sense (the contributing precondition terms of `checked_dispatch.rs`). FR-322 defines "term" for the body walk (a visited expression node). Suggest "even where only one authored precondition clause contributes". | spec/objects/interfaces/FR-322-checked-package-artifact.md:2531-2535 |
| FND-002 | low | The new normative statement has no acceptance criterion: no FR-322 AC names `effective_precondition` or the single-clause case. The only checks are FSL `a_source_operation_declares_its_effective_precondition_function` and the contract-ir reader, and neither traces to an FR-322 AC. | spec/objects/interfaces/FR-322-checked-package-artifact.md:2523-2538 |
| FND-003 | medium | QSL-3398 is attached to PR #848 and named in its title. A Linear on-merge transition would move it to Done, but this PR delivers only ruling item 3. Items 1, 2 and 4 remain: FSL FR-569-AC-5/AC-6 restated, the twin test `the_source_and_model_twins_link_equal_tables_and_check_equal_types` updated, and the verdict and work-charge comparison between S and D run and reported. FSL main stays red until they land. | QSL-3398; PR #848 title |

## Scope

- FR-322 "Clause functions" item 5, the new sentence (examined; FND-001, FND-002, FND-003)
- FR-322 dispatch_tables paragraph (context_only)
- FR-322 body walk rule and FR-322-AC-513 including the BL-06 paragraph (examined)
- TC-2090 Test line and steps 8 and 9 (examined)
- FR-379-AC-13, FR-379-AC-14 against refinement-failure vectors (examined)
- FR-331-AC-67, TC-431 BP-68, FR-408-AC-19 against limit-reached-fixed-sample-preflight (examined)
- FR-322-AC-392 against IC-21 (examined)
- FR-322-AC-278 and FR-830-AC-6 (STD-737 conflict; context_only)

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The PR title now names STD-731, and a Linear on-merge transition would close it. The lead says STD-731 stays open until the one-line contract-ir change to `earlier_protocol_law_refusal_precedes_missing_temporal_selection`, which hardcodes node 82. contract-ir be92e1c3 `conformance_fspec protocol_mutation_manifest::` still fails that test against 25fdcd07 (log r2-731-green). Use a non-closing reference, such as `Refs STD-731` in the body with STD-731 removed from the title, or reopen the ticket after merge. | PR #848 title |

## Dispositions

Round 1, head 25fdcd07. AC-278's new exclusion lists the FR-830 original-key members (`node_key`, `type_handle`, `settled_key`, `member.declaration`, keys inside `original_type`) and keeps `structural_node` and payload `node` covered. That matches the FR-830 definition and arch ruling 4e8a79c9 on STD-737, and the TC-233 scan sentence states the same exclusion. FR-322-AC-544 does not clash with open PR #847, which uses AC-539 to AC-543; FR-322-AC-544 appears on no other origin branch.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 25fdcd07 rewords to "even where only one authored precondition clause contributes". |
| FND-002 | fixed | 25fdcd07 adds FR-322-AC-544. Its test evidence is FND-002 of SR-85913. |
| FND-003 | fixed | QSL-3398 is out of the PR title; the body says "Refs QSL-3398 (FSpec sentence only; FSL work open)", a non-closing reference; FSL PR #3963 is now attached to QSL-3398 for the FSL work. The #848 link attachment remains on QSL-3398; verify its state after merge. |

Round 2, head a3a02703. The PR title no longer names STD-731, and the body reads "Refs STD-731 (stays open for the contract-ir change)", which is a non-closing reference. Since the #848 attachment stays on STD-731, check its state after merge.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | Title: "STD-732, STD-733, STD-734, STD-735, STD-736, STD-737: fix stale conformance vectors, reader tests and the nested-key rule"; body: "Refs STD-731 (stays open for the contract-ir change)." |
