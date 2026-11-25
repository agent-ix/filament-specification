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
