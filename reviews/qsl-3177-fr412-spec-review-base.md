---
id: SR-202611210022442313144108562
title: "QSL-3177 FR-412 base review"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-412-probability-certificate.md, proposals/backend-provider-v1/schema.json"
review_set: subset
---

## Summary

Reviewed the root-index reset amendments against the existing certificate row shape, paired FSL FR-196-AC-18, and the schema transport description. The new requirements preserve root identity across convergence, define per-root reset/value/policy/ranking checks, and retain the legacy key bytes outside the multi-root reset case.

## Verdict

**PASS** — no defects found in the requested behavior. The new ACs remain to be bound to implementation tests by the code lane; this spec-only review does not treat the not-yet-landed implementation as a defect in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Round 1 Re-review

Re-reviewed PR head `8f24ceb2d5cab784d14ab3c194f793d99b840b96` against paired FSL FR-196-AC-18 at PR #3842 head `7c6e028bc923f0a700bf323b46b8b928a0bd333d`. FR-411 and FR-331 specify the same conditional ninth `root` component and preserve the original eight-member bytes for single-root and no-reset cases. TC-363 EC-08 now scopes its eight-member fixture to a bounded no-reset product. No prior findings existed and no new defect was found.
