---
id: SR-202611210022442313144108565
title: "QSL-3177 FR-412 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-412-probability-certificate.md, proposals/backend-provider-v1/schema.json"
review_set: subset
---

## Summary

Reviewed the new root-indexed identity for convergence aliasing, reset target selection, cross-root row mixing, incomplete row coverage, and legacy-key compatibility. The root index remains in the existing complete state-key encoding and is independently checked against each root's reset process.

## Verdict

**PASS** — no failure-domain defects found in the amendment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Round 1 Re-review

Re-reviewed PR head `8f24ceb2d5cab784d14ab3c194f793d99b840b96` against paired FSL FR-196-AC-18 at PR #3842 head `7c6e028bc923f0a700bf323b46b8b928a0bd333d`. FR-411 and FR-331 preserve root identity in complete product-key octets; TC-363 no longer applies the legacy key fixture to reset products. No prior findings existed and no new failure-domain defect was found.
