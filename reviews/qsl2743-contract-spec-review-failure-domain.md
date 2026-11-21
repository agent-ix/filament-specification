---
id: SR-2026112106460400000013
title: "spec-review/failure-domain review of FR-830 identity roles and refusal precedence"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-830-carry-selected-model-union-witness-rows.md; proposals/checked-package-v2/model-union-witness-vectors.json; QSL-2743; PR 789"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: references
---

## Summary

Ticket: QSL-2743. PR agent-ix/filament-specification#789 at fb65d9db. This review looks for unstated failure modes, identity confusion and topological edge cases.

Clean:
- Each node reference (CheckedNodeId) is kept apart from each original NodeKey.
- The sealed `UnionMember{declaration, identifier}` is used, never an enum `VariantId` or a fabricated case node.
- Canonical `rank` and authored `position` are kept apart.
- `original_case` is the structured `VariantRecord` `DeclarationKey`.
- The `none`/`data`/`clock` trichotomy holds: Clock is data-arity 0 and not nullary.
- Two same-spelled cases are distinguished by declaration (MUW-08, admitted by the strict read).
- Cycle termination works by worklist and visited set (List↔Cell, Node↔Link).
- Unreached declarations are not required (MUW-14).
- First-refusal precedence: every reader refusal's first failing step was traced, and only the bases named in SR-2026112106460400000010 pre-empt.
- No test assumes provenance proof. MUW-10 admits forged-but-consistent keys.

## Verdict

**CONDITIONAL.** The type_handle/settled_key role distinction, which the arch ruling requires, has no isolating vector (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | In every positive, including the forged MUW-10, each union row's `type_handle` equals its `settled_key` byte for byte, and no refusal gives them different values. FR-830 says the two "are different roles ... and no relation between them is required". Step 5 checks `member.declaration` against `settled_key`, and a union payload's `composite` is checked against the payload union's `type_handle`. A reader that swaps the two roles, or that refuses or collapses `type_handle != settled_key`, therefore passes every vector. Add a positive (MUW-10 is the natural place) whose `type_handle` and `settled_key` differ, with a union payload (`Tail(List)` or `Next(Link)`) that names the `type_handle`. Add refusals where `member.declaration` equals the `type_handle` and where a payload composite names the `settled_key`. | FR-830:79-85,154,217-218; FR-830-AC-9; FR-830-AC-12; model-union-witness-vectors.json (MUW-10) |

## Scope

- FR-830 model-union row and case definitions (examined; FND-001)
- FR-830 Side-input witnesses table (examined; FND-001)
- FR-830 Checks steps 3, 5, 6 (examined; FND-001)
- FR-830-AC-4, AC-5, AC-8, AC-10, AC-12 (examined, clean for failure-domain)
- FR-830-AC-9 (examined; FND-001)
- FR-830 reached closure and cycle rule (examined, clean)
- FR-441 UnionMember pair (context_only)
