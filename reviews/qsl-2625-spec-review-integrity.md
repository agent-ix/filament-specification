---
id: SR-2026112005294500000003
title: "spec-review of QSL-2625 FSpec body_locations vectors"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #765 at 2c28b3de8bd3493a1a90fc676ce377baf4f421c4; proposals/checked-package-v2/checked-term-data-vectors.json (body_locations), tests/checked_package_v2.rs (tc_2090_function_body_location_rows, tc_367 hunk); context: FR-322 body_locations paragraph and FR-322-AC-490, TC-2090, schema.json BodyLocationOrigin"
review_set: subset
---

## Summary

Ticket: QSL-2625. Reviewed the BL vectors and `tc_2090_function_body_location_rows` against FR-322-AC-490 and TC-2090 (merged #760). Read only; no builds.

## Examined scope

- BL-01, BL-02 (examined, correct). Nine rows over package `f`, origin `{function_body, f, f, 0}`; the composition parts at `[]`, the sqrt rows at `[0]` and `[1]`; BL-02 changes only the last path to `[0]`.
- BL-03 (examined, correct). Package `h`; the one `+` node has rows at occurrences 0 and 1, ordinal 0 each, paths `[0]` and `[1]`.
- BL-R1 to BL-R6 (examined, loci match AC-490). BL-R7 (a row naming expression occurrence 1 of a node that has one) is an extra shared-rule case.
- Schema-negative cases (examined): a carrier without `body_locations`, and a `type_declaration` origin kind. Carrier-less package (examined): refuses at `/checked_term_data/body_locations`.
- TC-2090 traces FR-322-AC-490 (examined, correct).
- Overlap (examined). #765 does not edit FR-322 or any file that #767, #766, #695 or #640 edit. It adds no AC or TC id.

## Verdict

Not mergeable until FND-001 is fixed. FND-002 should be fixed in the same round.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-490 and TC-2090 step 4 require BL-03 with its occurrence-1 `+` row removed to refuse at `/checked_term_data/body_locations`. No vector has this case, and the test does not check it. It is the case that catches a reader that counts a node reached twice only once. BL-R7-no-occurrence-1 is a different case: package `f`, an undeclared occurrence, refused at `/0/occurrence`. Add a `reader_refusals` entry over package `h` (BL-03's rows without the occurrence-1 `+` row, locus `/checked_term_data/body_locations`), and add its id to the asserted list | proposals/checked-package-v2/checked-term-data-vectors.json:15844 |
| FND-002 | medium | The positive vectors are validated only as a carrier (`CheckedTermData`) and as rows (`BodySemanticLocation`). The whole packages `f` and `h` are never validated against the checked-package schema, so a package that is not valid wire would pass the test. Validate `package_with(case)` against the root package validator for every positive vector | tests/checked_package_v2.rs:25382 |
| FND-003 | low | An unrelated `tc_367` change reformats the failure message (`instance_path` and `kind` instead of `to_string()`). It is outside QSL-2625. Drop it, or state why in the PR | tests/checked_package_v2.rs:16506 |
