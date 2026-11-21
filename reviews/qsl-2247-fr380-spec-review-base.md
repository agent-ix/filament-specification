---
id: SR-2026112106460400000003
title: "base review of FR-380 terminal-stutter correction"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; QSL-2247; FR-380-AC-9 and TC-1042; PR 782 at 8c352bff9e4060f68348e4bd5a488ac78a5807e0"
review_set: subset
---

## Summary

Reviewed the exact authorized FR-380-AC-9 and TC-1042 correction. No defects found in this bounded specification change.

## Verdict

**PASS — MERGEABLE for the specification stage.** Runtime implementation and exhaustive repository assurance are not claimed.

## Scope

- **FR-380-AC-9** (examined), spec/functional/analysis/FR-380-transfer-claims-through-refinement.md: a proved abstract `always (holds(c.value = 1) implies once[1,1] holds(c.value = 0))` and a proved abstract `always (holds(c.value = 1) implies eventually[0,2] holds(c.value = 2))` each transfer `proved`, basis `decisive-witness`, with no exploration of the concrete subject
- **TC-1042** (examined), spec/test-cases/TC-1042-step-for-step-refinement-transfers-previous.md: The abstract clause is `refuted`: on the counter behaviour with values `0, 1, 2, 2, ...`, the antecedent holds at the repeated terminal state 2 but `once[1,1] holds(c.value = 1)` is false because the preceding state also has value 2.
- **FR-380-AC-5** (context_only), spec/functional/analysis/FR-380-transfer-claims-through-refinement.md: A premise that is refuted, inconclusive, unsupported or failed settles the transfer item `inconclusive`, cause `premise-unproved`, naming the premise; no transfer item settles `refuted`.
- **FR-380** (context_only), spec/functional/analysis/FR-380-transfer-claims-through-refinement.md: A transfer item is never `refuted`: an abstract counterexample is not a concrete behaviour.
- **FR-127** (context_only), /Users/peter/dev/filament-spec-language/spec/functional/FR-127-settle-a-model-check-verdict-as-a-terminal-record.md: `Holds{Exhaustive}` whose certificate the core checker accepts

## Review Notes

FR and TC identity, unchanged frontmatter/relationships, direct TC-1042 verification linkage, positive/negative boundary, expected labels/bases/causes and premise slot are coherent. No hash/pin/tracking ceremony is introduced. The repository-local computed matrix reports FR-380-AC-9 untagged; this pre-existing implementation-evidence backlog is disclosed, not claimed resolved by a spec-only change.

No applicable AssuranceProfile exists under the reviewed spec tree. Selected methods are code-review, base and integrity. EARS is not selected because no requirement-bearing SHALL statement changes; only one acceptance criterion and the TC description/procedure/oracle change. Dependency analysis is not selected because relationship edges are unchanged. No production/Rust files change, so language review and implementation gap analysis do not apply. No builds or tests were run, as directed. Scoped Quire validation and git diff checks pass. Matrix diagnostics are retained at /private/tmp/fr380-matrix-diagnostics.log; no repository-wide coverage claim is made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
