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

## Dispositions

Round 1, against 63a7f10abfdf8e8876bb2fabfb039f443631e93f (fix delta fb65d9db..63a7f10a). Strict-read evidence: verify-disposition1-strict-read.log (rerun of the existing-reader scratch driver, exit 0); the reader is filament-contract-ir origin/main 743b5e5f1. Every remaining fixture B/C refusal is the known `Mix.Held` object-payload engine gap (`/types/12/variants/6/payloadType`), not an FSpec defect; with Held removed every B/C positive is ADMITTED. This is an existing-reader observation, not FR-830 qualification.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | MUW-18 (reached from Mix, fixture B) gives every union row distinct full original keys: `type_handle` `{fsl,[type,<id>],0}` and `settled_key` `{fsl,[settled,<id>],1}`. Every `member.declaration` equals the row's `settled_key` (jq-checked), and `Mix.Next`'s composite names Link's `type_handle`. Wrong-role controls: R-56 (`member.declaration` is the `type_handle`, step 5), R-57 (the payload composite is the `settled_key`, step 6) and R-58 (roles swapped, first refusal step 5 at `cases/0/member/declaration`). AC-7 states the distinction. With Held removed, the strict read admits MUW-18. |
