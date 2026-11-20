---
id: SR-2026112005294500000005
title: "Base specification review — FR-331 certificate inventory"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-331-backend-provider-envelope.md (Certificate member row, FR-331-AC-25, FR-331-AC-46); spec/objects/interfaces/FR-412-probability-certificate.md (FR-412-AC-10); spec/objects/interfaces/FR-418-zone-certificate.md (FR-418-AC-15, FR-418-AC-17)"
review_set: subset
---

## Summary

Reviewed the changed FR-331 certificate-member row and AC-46, with FR-331-AC-25 and the cited FR-412/FR-418 criteria as context. The inventory and the named source requirements agree; the revised criterion is specific and testable by set reconciliation.

## Verdict

**PASS** — no specification-text defect found in the reviewed scope.

## Review notes

No AssuranceProfile applies to the reviewed scope. The criterion-strength analysis skill was not run: its installed instructions state that it is blocked until the Jev client exists. The `spec-correctness` skill generates property tests and does not serve as a review method for this change.

The changed row lists 61 names. The six FR-412 conditional-certificate names and the three FR-418-AC-17 names are explicitly attributed, and the existing `non-zeno-mismatch` rule is restricted to `{member: non_zeno}` as FR-418-AC-15 requires. The ticket's arch ruling says unsupported names removed: none; the PR body agrees.

## Scope

| ID | Role | Excerpt |
| --- | --- | --- |
| Certificate member row | examined | `Its closed 61-name set is ... conditional ... denominator ... ef_nodes, form ... mode ... numerator ... post ... reset ... state_graph ...` |
| FR-331-AC-25 | context_only | `An independent finite inventory enumerates all 25 owned rule spellings and equals both the schema's rule set and the positive-vector rule set; removing a rule from both fails that comparison.` |
| FR-331-AC-46 | examined | `The member enum's set equals that row and the set of property names of the two certificates and their certificate-owned definitions computed from the schema; adding or removing a name on any side fails the equality. The count is derived from that reconciled set` |
| FR-412-AC-10 | context_only | `a conditional certificate without conditional is rejected malformed at conditional, a ratio one without denominator ... malformed at denominator, ... mode ... malformed at mode` |
| FR-418-AC-15 | context_only | `each with its non_zeno swapped, or removed, settles inconclusive, certificate-rejected{rule: non-zeno-mismatch, at: {member: non_zeno}}` |
| FR-418-AC-17 | context_only | `the zone certificate admits the state_graph part {form: always-possible, ef_nodes: [0, 2]} ... and refuses ... with form possible, with an extra member, or without ef_nodes` |

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low | No findings (placeholder) | - |
