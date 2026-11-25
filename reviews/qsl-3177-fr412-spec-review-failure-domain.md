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
