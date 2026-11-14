---
id: SR-202611141914004
title: "Gap analysis of the STD-661 operation-body amendment"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR #656 at ea7010a5: FR-322-AC-397, TC-987, tests/checked_package_v2.rs operation_body_carrier_vectors_match_the_closed_member_schema_and_reader"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-661. Plan completion: not assessed. One test is tagged TC-987 and FR-322-AC-397. It
reads every positive case and checks each refusal's exact code, cause and locus, and those
assertions can fail. AC-397 now carries clauses on the selection-key join, the graph-derived
has_body and the check order. The test exercises the join for redefined and inherited receivers
(OBC-07), a body swapped between candidates (OBR-22), and a `no_body` row (OBR-20).

## Verdict

CONDITIONAL on FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No vector backs the stated rule that the carrier leaves an out-of-family or unjoined candidate to the FR-322-AC-290 dispatch join. The reference reader does not implement that rule, and OBR-20 contradicts it (failure-domain FND-001). Fix: add a case with an out-of-family candidate that the carrier admits (for example, a reader-scope positive case). Make OBR-20 an in-family case. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4021-4023; tests/checked_package_v2.rs:22844-22882 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | OBD-01's test runs the carrier half: `candidate_row_key` returns None and the carrier admits. The dispatch-join code and locus are pinned in the vector, not executed, which is the repo's existing pattern for DT-R* cases. They match quire-contract-ir `dispatch_projection.rs`, which raises `invalid_package`/`invalid-value` at `candidates/{j}/selection_key` when `operation_in_family` fails, the same refusal and locus DT-R5 records. Adequate for this repo. |
