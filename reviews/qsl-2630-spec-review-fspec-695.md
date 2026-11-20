---
id: SR-20261120051604
title: "Base review of FR-331 open-history and FR-785 replay binding (PR #695)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #695 at c0cf00715c, diff origin/main...origin/qsl-1887-replay-binding: FR-059, FR-331, FR-520, FR-785, TC-1911, backend-provider-v1 schema.json and vectors.json"
review_set: base
---

## Summary

Ticket: QSL-2630 (this PR carries QSL-2343). It adds the `open-history` inconclusive cause for a `global-conformance` entry (FR-331, FR-059, schema rule, one positive and two adverse vectors) and an FR-785 capacity binding for finite replay and recording assessment (FR-785-AC-7). It branched 69 commits behind main and conflicts with it, and its `open-history` condition contradicts FSL FR-990's `incomplete-closure-scope` row.

## Verdict

FAIL on FND-001 and FND-002. No file is shared with #767, #766, #765 or #640. FR-785-AC-7 is free. quire validate exits 0 on the branch files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Conflicts with origin/main in FR-331 (main added `no-evaluated-case` to the inconclusive row) and FR-520 (main #696 moved `capacities` into the `subject.protocol` binding); the FR-520 hunk and FR-785's "the `subject` member `capacities`" contradict main | spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md:62; spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:81 |
| FND-002 | high | `open-history` is required whenever an obligation is pending or closure is not closed, but FSL FR-990 settles the unknown-population case (closure closed, member obligations pending) `incomplete-closure-scope`, which FR-331 does not admit, so a conforming reader refuses FSL's result | spec/objects/interfaces/FR-331-backend-provider-envelope.md:748-755; spec/functional/protocol/FR-059-assess-finite-global-conformance.md:44-48 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | FSpec FR-059-AC-6 and FR-331 (like FSL FR-990's text) require `incomplete-closure-scope` for a fan-out recording with no stated population whatever the closure state, but fsl-analyze on filament-spec-language origin/main emits it only when `closure.state` is `closed` and gives `open-history` for an open record | spec/functional/protocol/FR-059-assess-finite-global-conformance.md:73; filament-spec-language fsl-analyze/src/protocol_analysis/recording.rs:543 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Rebased at 70500128c7: merge-tree with origin/main is clean, FR-331 keeps `no-evaluated-case`, the FR-520 hunk is gone, FR-785 names the `capacities` member of the subject's `protocol` binding |
| FND-002 | fixed | `incomplete-closure-scope` added to the FR-331 inconclusive row, cause list, exception clause, new paragraph and kind sentence, FR-059 text and AC-6, the schema enum and the global-conformance-only rule, with one positive and one adverse vector |
| FND-003 | still-open: FSL code divergence at recording.rs:543 (open record gives open-history); needs an FSL code ticket, not a #695 change | - |
