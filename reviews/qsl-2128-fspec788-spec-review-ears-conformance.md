---
id: SR-2026112106460400000009
title: "spec-review/ears-conformance review of FR-331 closed unchecked results"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-331-backend-provider-envelope.md; QSL-2128; PR 788"
review_set: subset
---

## Summary

Review of filament-specification#788 at c9d9ace2d0b440af764ada27f64e69b42792bc35. Ticket: QSL-2128.

## Verdict

**PASS** — No additional defect found by this method.

## Scope

- FR-331 (examined): An exhaustive result whose explored graph closed, with no node left open, but whose closure no accepted certificate has certified SHALL settle `incomplete` with cause `{"type": "bounded-search-incomplete", "reason": "certificate-unchecked"}`.
- FR-331 (context_only): A certificate a core checker rejects settles the item `inconclusive` with cause `certificate-rejected{rule, at}`: `rule` the check rule that failed and `at` where it failed, for every core checker except the hybrid certificate checker.
- FR-331-AC-58 (examined): The schema admits an `incomplete` result entry whose `cause` is `{"type": "bounded-search-incomplete", "reason": "certificate-unchecked"}` with no `task_search` member, and refuses the same entry with `task_search` added; an entry whose `cause` is `{"type": "bounded-search-incomplete", "reason": "unresolved-dynamics"}` with no `task_search` is still refused, and one whose reason is `closed` is refused.

## Validation and method limits

No applicable AssuranceProfile was found under spec/. Direct review selected code-review, base and EARS because a requirement-bearing SHALL statement changes. No object fields, relationships or structure change; dependency, object and integrity analyses were not selected. No production change, so language review and implementation gap analysis do not apply. No Cargo, Python or full gate ran. Scoped FR-331 validation fails identically on modified head and unmodified origin/main with value_object required id missing despite id frontmatter, plus existing ProtocolClaimKind and verifies vocabulary warnings. Both have 1/1 grammar-clean, zero grammar findings. This is a baseline module/tool limitation, not a new change defect. git diff --check passes. Embedded tests/backend_provider_v1.rs:28 reads FR331 only through the unchanged Certificate member inventory row at line 2547; the existing AC-58 wire test at line 2788 preserves cause shape. No schema, vectors or pins change. The changed SHALL is singular, names the result/settlement subject and gives a concrete terminal outcome. Rejection precedence is a semantic base finding, not a grammar defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
