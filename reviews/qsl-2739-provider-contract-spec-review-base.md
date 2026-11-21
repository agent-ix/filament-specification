---
id: SR-2026112114454020075413602
title: "spec-review/base: PR 792 AG interface refutation, assumptions binding and provider-answer-unconfirmed"
type: SpecReview
analysis: base
date: 2026-11-21
scope: "agent-ix/filament-specification; PR 792 git diff 9f5b30a5...35b3b0e0; spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/functional/analysis/FR-348-admit-compositional-verification.md, spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md, spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md, spec/objects/interfaces/FR-492-planner-settlement-request.md, spec/test-cases/TC-983-composition-carries-liveness-and-fairness-premises.md, spec/test-cases/TC-985-component-composition-declaration.md, spec/test-cases/TC-986-planner-settlement-request.md; QSL-2739"
review_set: subset
---

## Summary

Ticket: QSL-2739. Method: spec-review/base. Reviewer run 4b0eb3f0-7e32-4a5e-ad42-0e821771b4c3, model claude-opus-5-5, quire 0.52.0 (engine 0.62.2), quoin 0.36.8. Frozen head 35b3b0e0a133f8288b5d252f371c5e43b7437788 over base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f.

Base checklist over the 8 changed Markdown artifacts, checked against the four binding rulings recorded in the brief. Ruling 1 (refutation, one FR-492 exception, unsettled premise stays premise-unproved): implemented, FR-348-AC-2 unchanged. Ruling 2 (ordered `assumptions` in request identity): implemented on the composition item. Ruling 3 (fixed point typed unsupported, no grammar or schema): implemented; its coherence is covered under integrity. Ruling 4 (new closed cause, ReplayRefused strict): implemented as a new core-guard category in FR-331 "Result entries", refused `unknown-cause` in any provider entry, with `replay-refused` untouched. Author choice "weaker" is the FR-489 Implication fairness clause only: a faithful, deliberately narrow operationalization of FR-348's "weaker", decidable from the declaration and introducing no formula entailment. Accepted. ID formats, AC numbering and the TC scope lines are valid. `quire validate --strict` on the 8 files exits 3, all on three FR-331 warnings (line 102 ProtocolClaimKind unresolved-type; `verifies` edges to TC-372 and TC-431). Base 9f5b30a5 has the same line 102 and the same frontmatter edges, and the diff changes no relationships, so the three warnings are inherited. The other 7 files are clean. No hash, digest or pin antipattern.

## Verdict

**FAIL**: one high finding (FND-001); the rest are medium or low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The core-guard SHALL is scoped to "a request holding a `composition` item that also holds the `component-guarantee` items the item's `premises` name". That makes its own "whose item the request does not hold" branch unreachable. A request lacking those items is outside the guard, so a provider's `proved` composition answer stands on the provider's word, which is the forgery the guard exists to stop. It also contradicts TC-986 step 15's third case (item absent, expected `provider-answer-unconfirmed`). The guard must apply to every `proved` answer of a `composition` item with `premises`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:890-909 |
| FND-002 | medium | The interface-mismatch order is ambiguous. "In this order over the item's rely and liveness-dependency slots in slot order", followed by a numbered Absent/Weaker list and AC-13's "the first in slot order decides, absent before weaker", reads either slot-major or reason-major. Absent and weaker cannot coincide on one slot, so "absent before weaker" is vacuous unless the order is reason-major. With slot 3 weaker and slot 4 absent, one reading reports slot 3 `weaker` and the other slot 4 `absent`, a different `mismatch` and counterexample. State one order. | spec/objects/interfaces/FR-492-planner-settlement-request.md:238-245, 294 |
| FND-003 | medium | FR-491-AC-7 says an `assumptions` array "that differs from what the declaration's rely and depends members derive" is refused before consumption (FR-331-AC-61). FR-331's reader obligations and AC-61 cover only shape (length, range, role, self index, ascending) and never compare against the declaration. Nothing owns refusing a well-formed array that adds an undeclared peer index. Such an array would let a provider prove `G` under an extra assumption, and the core guard checks only component settlements, never assumptions. State the derivation check in FR-331 (the reader holds the checked package) and in AC-61. | spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md:222 |
| FND-004 | medium | The refutation is given basis `decisive-counterexample`. FR-243 defines that basis as an open scope settled `violated` because the profile proves "the retained counterexample preserves that truth under every admitted continuation". An interface mismatch is decided from the declaration alone and retains no behavioural counterexample, so the FR-243 definition does not cover it. Either amend FR-243's `decisive-counterexample` row to admit a declaration-decided `interface-mismatch`, or state the basis this refutation uses. | spec/objects/interfaces/FR-492-planner-settlement-request.md:199-204, 236-260 |
| FND-005 | low | AC-61 asserts that items differing in "the order of an entry's indices" have different request identities. The same AC refuses a not-strictly-ascending entry before consumption, so a reordered entry never gets an identity to compare. FR-491 "Assumption binding" repeats the claim ("in its order"). Drop the order clause, or say the reordered item is refused. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:1144 |

## Scope

- FR-331-core-guard (examined; FND-001)
- FR-331-assumptions (examined; clean)
- FR-331-identity (examined; clean)
- FR-331-unknown-cause (examined; clean)
- FR-331-AC-61 (examined; FND-005)
- FR-331-AC-62 (examined; clean)
- FR-492-value (examined; clean)
- FR-492-interface-mismatch (examined; FND-004)
- FR-492-AC-10 (examined; clean)
- FR-492-AC-13 (examined; FND-002)
- FR-492-AC-14 (examined; clean)
- FR-491-assumption-binding (examined; clean)
- FR-491-AC-7 (examined; FND-003)
- FR-489-declared-interface (examined; clean)
- FR-489-settlement-refuted-row (examined; clean)
- FR-489-AC-12 (examined; clean)
- FR-348-assume-guarantee-para (examined; clean)
- FR-348-AC-2 (context only)

## Dispositions

Round 1, against bec3059e731621e5cfb63144c37a3eed3da851cc (fix diff 35b3b0e0..bec3059e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-331 core guard is unconditional over every proved composition answer with premises; AC-62 and TC-986 step 15 add the request-holding-none case. |
| FND-002 | fixed | One reason-major order; FR-492-AC-13 and TC-986 step 17 fix absent@4 over weaker@3. |
| FND-003 | fixed | FR-331 reader/producer derives from the checked package and refuses any non-derived array; FR-491-AC-7 and FR-331-AC-61 updated. |
| FND-004 | fixed | FR-243 decisive_counterexample row clarifies declaration-decided violation; FR-492 links it; no new basis. |
| FND-005 | fixed | The reordered-entry identity claim was removed from AC-61 and FR-491. |
