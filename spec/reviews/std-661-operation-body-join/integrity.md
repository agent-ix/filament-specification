---
id: SR-202611141914002
title: "Integrity review of the STD-661 operation-body amendment"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #656 at ea7010a5: FR-322 Operation-body carrier, FR-322-AC-397, TC-987, proposals/checked-package-v2/README.md, operation-body-vectors.json note and ids"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-987, type: references }
---

## Summary

Ticket: STD-661. This review checks cross-references and counts across FR-322 prose, AC-397,
TC-987, the README and the vectors. The vectors hold 7 positive cases, 13 schema refusals and 21
reader refusals (OBR-01 to OBR-20, then OBR-22). OBR-21 is retired and its id is not reused. TC-987's
code and locus groups match each vector's `expected_code`, `expected_cause` and `locus`. No
`has_body` remains in any vector document. The only other files that mention `has_body`,
`foreign-owner` or OBR-21 are historical STD-660 review records, and they stay unchanged. `quire
validate` passes on FR-322 and TC-987. Its line-104 EARS warnings were already on main.

## Verdict

PASS for structure. The semantic defects in OBC-07, OBR-20 and OBR-22 are in the failure-domain
SpecReview (SR-202611141914001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The row records no finding. Counts after the fix round: 7 positive cases, 13 schema refusals, 21 reader refusals (OBR-01 to OBR-20 and OBR-22, OBR-21 retired and not reused) and 1 dispatch-join refusal (OBD-01), consistent across FR-322 AC-397, TC-987, the README and the vectors. |
