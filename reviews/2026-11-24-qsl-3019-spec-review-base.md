---
id: SR-202611210022442313144108561
title: "base review of QSL-3019 FSpec criteria"
type: SpecReview
analysis: base
scope: "spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md; spec/test-cases/TC-386-explicit-protocol-step-rows.md; supporting FR-375, FR-322, TC-233, TC-380 and FR-736"
review_set: subset
---

## Summary

Reviewed the new FR-432 compensation-row criterion, its TC-386 vector, and the companion FR-375 grammar and FR-322 identity amendments. The QSL-3019 example does not isolate one missing row: it supplies only the three compensation rows even though the Pay fixture has other concrete step classes that also require rows.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The FR-432-AC-25 examples omit required non-compensation Pay rows, so restoring one compensation row cannot produce the claimed admitted refinement or isolate the named missing-row refusal. | FR-432-AC-25; TC-386 SR-16; TC-380 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-432-AC-25 now supplies a complete baseline with the attempt, effect, event, finish, cattempt, cend and activate rows, then removes exactly one compensation row in each case. TC-386 SR-16 repeats that isolation explicitly. |
