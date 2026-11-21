---
id: SR-2026112106460400000008
title: "spec-review/base review of FR-331 closed unchecked results"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-331-backend-provider-envelope.md; QSL-2128; PR 788"
review_set: subset
---

## Summary

Review of filament-specification#788 at c9d9ace2d0b440af764ada27f64e69b42792bc35. Ticket: QSL-2128.

## Verdict

**FAIL** — The unchecked condition conflicts with the existing rejected-certificate outcome.

## Scope

- FR-331 (examined): An exhaustive result whose explored graph closed, with no node left open, but whose closure no accepted certificate has certified SHALL settle `incomplete` with cause `{"type": "bounded-search-incomplete", "reason": "certificate-unchecked"}`.
- FR-331 (context_only): A certificate a core checker rejects settles the item `inconclusive` with cause `certificate-rejected{rule, at}`: `rule` the check rule that failed and `at` where it failed, for every core checker except the hybrid certificate checker.
- FR-331-AC-58 (examined): The schema admits an `incomplete` result entry whose `cause` is `{"type": "bounded-search-incomplete", "reason": "certificate-unchecked"}` with no `task_search` member, and refuses the same entry with `task_search` added; an entry whose `cause` is `{"type": "bounded-search-incomplete", "reason": "unresolved-dynamics"}` with no `task_search` is still refused, and one whose reason is `closed` is refused.

## Validation and method limits

No applicable AssuranceProfile was found under spec/. Direct review selected code-review, base and EARS because a requirement-bearing SHALL statement changes. No object fields, relationships or structure change; dependency, object and integrity analyses were not selected. No production change, so language review and implementation gap analysis do not apply. No Cargo, Python or full gate ran. Scoped FR-331 validation fails identically on modified head and unmodified origin/main with value_object required id missing despite id frontmatter, plus existing ProtocolClaimKind and verifies vocabulary warnings. Both have 1/1 grammar-clean, zero grammar findings. This is a baseline module/tool limitation, not a new change defect. git diff --check passes. Embedded tests/backend_provider_v1.rs:28 reads FR331 only through the unchanged Certificate member inventory row at line 2547; the existing AC-58 wire test at line 2788 preserves cause shape. No schema, vectors or pins change. The existing AC-58 test is relevant to unchanged wire shape; no full matrix completion is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The broadened closed-result rule includes an HP-1 certificate checked and rejected (no accepted certificate certified its closure), requiring incomplete/certificate-unchecked while FR-331 lines 173-175 require inconclusive/certificate-rejected. Restrict the unchecked condition to absent/not-performed checks and preserve rejection and checker-limit precedence explicitly. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:744-756 |

## Disposition pass 1

**PASS; mergeable** at 342cd6c62678c6f3f4d8791524e1f6d24fd8bb78. Rechecked the bounded wording fix and surrounding rejection, stop and existing cause rules. Absent or unperformed checks now use CertificateUnchecked; rejected certificates remain inconclusive/certificate-rejected, and limit-stopped checks retain their incomplete limit cause. No new finding. Scoped FR-331 validation still reports the same baseline module issue; grammar remains clean. No Cargo or branch edits.

## Dispositions

Round 1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The condition now requires an absent certificate or an unperformed core check; explicit rejected and limit-stopped precedence is retained. |

Fixed excerpt:

> When an exhaustive result's explored graph is closed, with no node left open,
> and its certificate is absent or no core certificate check has been performed,
> the producer SHALL settle the item `incomplete` with cause
