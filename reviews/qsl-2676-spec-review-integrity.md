---
id: SR-2026112005294500000006
title: "Integrity analysis — FR-331 certificate inventory"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-331-backend-provider-envelope.md (Certificate member row, FR-331-AC-46); spec/objects/interfaces/FR-412-probability-certificate.md (FR-412-AC-10); spec/objects/interfaces/FR-418-zone-certificate.md (FR-418-AC-15, FR-418-AC-17)"
review_set: subset
---

## Summary

Checked completeness, consistency, atomicity, verifiability, and the failure-domain prompts that apply to the changed closed certificate-member inventory. The newly documented names have merged-requirement sources, and the schema/member set remains closed and reconciled.

## Verdict

**PASS** — no integrity defect found in the reviewed scope.

## Analysis

FR-412 specifies the six conditional-certificate names (`conditional`, `mode`, `denominator`, `numerator`, `post`, `reset`); FR-418-AC-17 specifies `state_graph`, `form`, and `ef_nodes`. The certificate-rejected rule `non-zeno-mismatch` is already scoped to `non_zeno` by FR-418-AC-15. The rule and member criteria remain distinct and externally verifiable. For the failure-domain lens, this diff changes no provider execution, identity calculation, user-supplied evaluation, or graph traversal behavior; the edited contract continues to reject members outside the closed set.

## Scope

- `spec/objects/interfaces/FR-331-backend-provider-envelope.md:231` — examined the closed 61-name member inventory and requirement attribution.
- `spec/objects/interfaces/FR-331-backend-provider-envelope.md:1036` — examined AC-46's set equality and derived-count obligations.
- `spec/objects/interfaces/FR-412-probability-certificate.md:67, 248` — context for conditional certificate forms and names.
- `spec/objects/interfaces/FR-418-zone-certificate.md:331-332, 370, 372` — context for `non-zeno-mismatch` and FR-418-AC-17.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low | No findings (placeholder) | - |
