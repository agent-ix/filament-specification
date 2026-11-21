---
id: SR-2026112114454020075413604
title: "spec-review/object-review: PR 792 assumptions, mismatch and core-guard objects"
type: SpecReview
analysis: base
date: 2026-11-21
scope: "agent-ix/filament-specification; PR 792 git diff 9f5b30a5...35b3b0e0; spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md, spec/objects/interfaces/FR-492-planner-settlement-request.md, proposals/backend-provider-v1/schema.json; QSL-2739"
review_set: subset
---

## Summary

Ticket: QSL-2739. Method: spec-review/object. Reviewer run 4b0eb3f0-7e32-4a5e-ad42-0e821771b4c3, model claude-opus-5-5, quire 0.52.0 (engine 0.62.2), quoin 0.36.8. Frozen head 35b3b0e0a133f8288b5d252f371c5e43b7437788 over base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f.

Domain-object audit of the new members and records: the `assumptions` item member (FR-331, FR-491, schema), the planner result's `mismatch` (FR-492), the FR-331 `interface-mismatch` counterexample and the `provider-answer-unconfirmed{premise}` cause. The prose and schema shapes agree. `InterfaceMismatch` matches the FR-331 bullet and the FR-492 `mismatch` member as `{type, component, interface, premise, reason}`, with `component` a name, `interface` a NodeId, `premise` a Count and `reason` absent/weaker. FR-492 slot indices equal the FR-331 `premises` indices, because premises are spelled as the slots in derived order. The `assumptions` indexing (per component-guarantee entry, inner indices into the full `premises` array, strictly ascending, no self index) is determinate. The open object question is where the component-guarantee slot key gets its assumption set.

## Verdict

**CONDITIONAL**: one medium finding and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-492's slot-key enumeration ("... correspondence, premises and extent classification") was not extended with `assumptions`, although FR-331 now puts `assumptions` in request identity. On the planner route the component-guarantee slot key has no carrier for its assumption set. That slot is backed by G's own temporal item, which the schema forbids from carrying `assumptions`, and the FR-492 request has no FR-331 composition item. So FR-491's "a result for G checked under other assumptions, or under none, is another key" and "a slot key reads the assumption set from the item that carries it" have no defined source. That is the PR1 local-only key gap, still open on that route. Name `assumptions` in the slot key and state where the component-guarantee slot reads its assumption set. | spec/objects/interfaces/FR-492-planner-settlement-request.md:83-87 |
| FND-002 | low | The mismatch result's "slot outcomes are those read so far" is undefined. "Decision order" runs Interface mismatch first, but slot outcomes are settled in the earlier "Slot outcome" section, so whether `slots` holds every slot, none, or those before the mismatching slot is unstated, as is its interaction with a `max_work` stop. State which slots a refuted result lists. | spec/objects/interfaces/FR-492-planner-settlement-request.md:257-260 |

## Scope

- FR-331-assumptions (examined; clean)
- FR-331-identity (examined; clean)
- FR-331-counterexample (examined; clean)
- FR-492-slot-key (examined; FND-001)
- FR-492-interface-mismatch (examined; FND-002)
- FR-492-decision-order (examined; clean)
- FR-491-assumption-binding (examined; clean)
- schema-InterfaceMismatch (examined; clean)
- schema-provider-answer-unconfirmed (examined; clean)
- FR-331-core-guard (context only)

## Dispositions

Round 1, against bec3059e731621e5cfb63144c37a3eed3da851cc (fix diff 35b3b0e0..bec3059e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-492 slot key carries `assumptions` for component-guarantee slots; admission step 5 computes keys with derived assumptions; FR-491 states both routes. |
| FND-002 | fixed | A refuted result lists the complete slot outcome list in slot order. |

Round 2, against b3c66f3554a142998bd211782bd6626b71fa7784 (fix diff bec3059e..b3c66f35).

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | The guard states the match: the held item's own identity members equal the key's item members, and the key's `assumptions` equal the k-th derived array (the FR-492 step 5 computation). The context is never taken from the held item and never matched by clause node alone. This is not a relabelling: the held item carries no assumptions and so proves `G` unconditionally (stronger than under assumptions, hence sound), and the key-vs-derived check rejects a forged entry context. AC-62 and TC-986 step 15 cover the same-clause-node other-member item, the altered key context and the matching case. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The fixed guard matches "the request item whose slot key (FR-492 Request) equals each component-guarantee entry's key". After this round, a component-guarantee slot key includes `assumptions`, but a component item in an FR-331 request carries none, and the guard does not say the item's key is computed with the declaration-derived `assumptions`, as FR-492 admission step 5 now does. Read literally, no held item ever matches, so every AG `proved` is replaced by `provider-answer-unconfirmed` (fail-closed, but AG can never prove). Read loosely, an implementer drops `assumptions` from the compare. State the same "computed with the declaration-derived `assumptions` of that entry" rule in the guard, and add the matching case to AC-62 and TC-986 step 15. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:897-903 |
