---
id: SR-2026112114454020075413603
title: "spec-review/integrity: PR 792 fixed-point unsupported coherence and TC coverage"
type: SpecReview
analysis: integrity
date: 2026-11-21
scope: "agent-ix/filament-specification; PR 792 git diff 9f5b30a5...35b3b0e0; spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md, spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md, spec/objects/interfaces/FR-492-planner-settlement-request.md, spec/test-cases/TC-983-composition-carries-liveness-and-fairness-premises.md, spec/test-cases/TC-985-component-composition-declaration.md, spec/test-cases/TC-986-planner-settlement-request.md; QSL-2739"
review_set: subset
---

## Summary

Ticket: QSL-2739. Method: spec-review/integrity. Reviewer run 4b0eb3f0-7e32-4a5e-ad42-0e821771b4c3, model claude-opus-5-5, quire 0.52.0 (engine 0.62.2), quoin 0.36.8. Frozen head 35b3b0e0a133f8288b5d252f371c5e43b7437788 over base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f.

Consistency between the new fixed-point `unsupported` rule (ruling 3) and the existing FR-176/FR-489/FR-491 fixed-point normatives, and AC-to-TC coverage of the new ACs. FR-176-AC-4 ("a supported cyclic composition records every iteration") is not contradicted, because an unsupported cycle is not a supported one. The contradictions are inside FR-489 and its TC.

## Verdict

**CONDITIONAL**: three medium findings and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-489 "Cycles" now says both that a fixed-point-deriving composition item "settles unsupported ... and no provider fills the slot" and, two sentences later and unchanged, that "an accepted cycle's fixed-point slot (FR-491) is filled by the FR-331 composition result of FR-176's item ... run by its provider; the cycle's premises are discharged only when that result is proved". The paragraph also keeps "accepted only under ... with every iteration recorded". Mark the provider-fill sentences as the deferred future route, or remove them, so the paragraph states one current behaviour. | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:118-139 |
| FND-002 | medium | FR-489-AC-5 requires a fixture "under a profile that declares FR-176's monotone lattice" while stating no requirement defines that declaration, so the fixture cannot be built, and it ends with "is open", which is not a pass/fail outcome. TC-983 expected result 4 still reads "with the lattice accepted and every iteration recorded", contradicting new expected result 8 ("unsupported ... no iteration is recorded"). FR-491-AC-4's "Under a profile declaring the lattice, the pair derives one fixed-point slot" and TC-985 step 4 have the same unconstructible precondition. Per ruling 3, state the observable outcome only (`unsupported` naming `composition`) and fix TC-983 expected result 4. | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:167; TC-983:53 |
| FND-003 | medium | FR-492-AC-13 and AC-14 are verified by Test (TC-986), and TC-986's scope now names FR-492-AC-14. But its new steps 14 to 16 and expected results cover only FR-331-AC-61/62. No TC-986 step builds the absent/weaker mismatch, the both-present ordering, or the declared-but-unsettled premise cases. Those exist only in TC-983 step 7, which traces FR-489-AC-12. Add the TC-986 steps, or point the two ACs at TC-983. | spec/objects/interfaces/FR-492-planner-settlement-request.md:294-295; TC-986:16,51-86 |
| FND-004 | low | The fixed-point `unsupported` reuses the negotiation diagnostic `unsupported_projection`/`unsupported-requested-capability` with free-text `detail` ("that a fixed-point composition is not available"). That code means no registered backend advertises the capability, but here a composition-capable provider may be registered. A reader cannot tell the fixed-point case from the no-provider case except through prose detail. Confirm that this is the typed form ruling 3 intends. | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:128-132 |

## Scope

- FR-489-cycles (examined; FND-001, FND-004)
- FR-489-AC-5 (examined; FND-002)
- FR-491-AC-4 (examined; clean)
- FR-492-AC-13 (examined; FND-003)
- FR-492-AC-14 (examined; clean)
- TC-983 (examined; clean)
- TC-985 (examined; clean)
- TC-986 (examined; clean)
- FR-489-AC-12 (context only)
- FR-331-AC-61 (context only)

## Dispositions

Round 1, against bec3059e731621e5cfb63144c37a3eed3da851cc (fix diff 35b3b0e0..bec3059e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-489 Cycles no longer says a provider fills a fixed-point slot or records iterations; FR-491 fixed-point slot and FR-492 table mention removed. |
| FND-002 | fixed | FR-489-AC-5 Withdrawn (quire matrix: retired); TC-983 expected 4 and step 8 fixed; FR-491-AC-4 lattice clause and TC-985 step 4 profile removed. The residual fixed-point clause is new finding FND-005. |
| FND-003 | fixed | TC-986 steps and expected results 17 and 18 cover FR-492-AC-13 and AC-14. |
| FND-004 | fixed | The diagnostic is grounded in FR-483's `unsupported` row form (detail names kind, form `fixed-point` and reason), with or without a registered provider. |

Round 2, against b3c66f3554a142998bd211782bd6626b71fa7784 (fix diff bec3059e..b3c66f35).

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | The input is now named: a well-formed `assume-guarantee` declaration whose dependency graph holds a cycle, for example TC-983 and TC-985 step 4's mutual `depends`. It settles typed `unsupported`, form `fixed-point`, before any slot, planner or provider step (FR-492 pre-slot step). FR-489-AC-4, FR-491-AC-4, the settlement row and TC-983/985 step and expected 4 are aligned. AC-7 is back to the plain unsupported rule. The constructibility gap and the 'every cycle refuses' contradiction are gone. |

Round 3, against 2f400c8b0c106768c2875b315e9a180d38227cfb (contained completion 430945dd, routing 9d89b90b, integration 20032191 and 2f400c8b).

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | FR-489 "Cycles" states that the typed `unsupported` is FR-176's refusal of an unsupported circular dependency for a composition item, never self-justifying, with no guarantee available as an assumption (FR-176-AC-3). The library normative stays coherent, with no new category or wire code. |
| FND-007 | fixed | The same early typed `unsupported` covers a well-formed cycle under all four rules, before any slot, planner or provider call (before writing the item to a provider for the three routed rules). The AG-only and "no other route" wording is removed. FR-489-AC-4, FR-491-AC-4 and TC-983/TC-985 step and expected 4 are parameterised over the four rules. |

Round 4, against 58f4a4df0c0e718af61468b5bec7482709936dc9 (routing completion and #798 integration).

| FND | outcome | reason |
| --- | --- | --- |
| FND-008 | fixed | FR-489 Inputs is conditional: the FR-492 library settlement stays planner-local, and the same declaration selected by name on an analyze request is an FR-331 `composition` request item of the existing `protocol` family (citing FR-491 "Derived item and slots"). This matches FR-290, FR-368, TC-271 and TC-302. No new family, verb or semantics. |
| FND-009 | fixed | The FR-290 note now says a cycle derives no `fixed-point` item and that its composition item settles `unsupported`, consistent with FR-489 and FR-491. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The fix makes every cycle refuse `invalid_model_binding`/`malformed-declaration` (FR-489 "Cycles", FR-491, FR-489-AC-4), and FR-491 derives no `fixed-point` slot. The rule spellings are still only the four, none of them fixed-point. Yet the same FR-489 paragraph, FR-489-AC-7 and TC-983 step 5 ("select a fixed-point composition") require a "fixed-point composition" to settle `unsupported`. No input is defined that is a fixed-point composition and not a refused cycle. So AC-7's new clause cannot be built, and read against a cycle it contradicts "every cycle is refused". Per ruling 3, either name the exact input that settles typed `unsupported` (for example, if a cycle is meant to, say that it settles `unsupported` instead of refusing) or drop the clause from AC-7 and TC-983 until the planner ticket defines one. | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:126-135, 166 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The library normative FR-176 "Behavior" still says "A dependency cycle is accepted only with an explicit sound fixed-point rule supported by the selected profile; otherwise it refuses as circular reasoning" (FR-176-AC-3: "refuses"). FR-489 "Cycles" and FR-491 now settle that same cycle with an FR-360 `unsupported` result, `unsupported-requested-capability`. That is a settled result, not a refusal. The text drops the earlier "refused exactly as FR-176-AC-3 refuses it" link and states no reconciliation. Two implementers of FR-176 and FR-489 would emit different outcomes for the same mutual-`depends` unit. Ruling 3 (typed unsupported) stands. Add one sentence, in FR-489 "Cycles" or as an FR-176 note, saying the typed `unsupported` is FR-176's refusal of an unsupported circular dependency for composition items (never self-justifying, no guarantee available as an assumption), so the library rule stays coherent. | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:119-138 |
| FND-007 | medium | At base 9f5b30a5, FR-491's cycle rule applied under every rule: without a lattice the composition item refused `malformed-declaration`. Now FR-491 says only that "for an `assume-guarantee` rule its composition item settles `unsupported`", and FR-489 "Cycles" names only an `assume-guarantee` declaration as the input, then says "this requirement defines no other route for a cycle". A well-formed `protocol-refinement`, `refinement-transfer` or `interface` declaration whose `rely`/`depends` members form a cycle now checks and is routed to the provider as an FR-331 `composition` item with no stated cycle outcome. The core guard checks only component settlements, so a provider's circular `proved` is not excluded. State the cycle outcome for every rule: the same typed `unsupported`, or keep the base refusal for non-AG rules. | spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md:111-117 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The approved routing exception (ruling A) was applied to FR-290, FR-368, TC-271 and TC-302 but not to FR-489 "Inputs". FR-489 still says "Under the rule `assume-guarantee` the composition item is planner-local, and the planner reads ... from one FR-492 settlement request; under any other rule it is FR-348's `composition` request item". FR-368 now says the same AG declaration selected by name on an analyze request "is instead an FR-331 `composition` request item" with family `protocol`, and FR-290 has a row for it. A reader of FR-489 alone would treat an analyze-selected AG composition as planner-local only. Add the analyze-selected FR-331 route, citing FR-491 "Derived item and slots". | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:56-61 |
| FND-009 | low | FR-290's new note "A cycle in a composition declaration derives no item (FR-489 Cycles)" can be read as "a cycle-bearing declaration derives no request item". That contradicts FR-489/FR-491, where the declaration's composition item exists and settles typed `unsupported`, form `fixed-point`. It means that no `fixed-point` item or slot is derived. Say "derives no `fixed-point` item; its composition item settles `unsupported` (FR-489 Cycles)". | spec/objects/protocol/FR-290-protocol-claim-kind.md:258 |

## Routing and integration (disposition pass 3)

Routing and integration verification (disposition pass 3, against 2f400c8b0c106768c2875b315e9a180d38227cfb):

- **Ruling A delta (9d89b90b vs b3c66f35):**
  - FR-290's claim-form table splits AG into an FR-492 planner settlement (family none) and an analyze-selected `composition` item (family `protocol`).
  - The node-family table has one `component_composition` row "of any rule" with family `protocol`; the obsolete accepted-cycle fixed-point row is removed.
  - FR-368 "Item kinds" is conditional: FR-492 stays planner-local, while an analyze-selected item is an FR-331 `composition` item with no `item_kind`.
  - TC-271 NG-23 adds the AG item under `protocol` (`unsupported-requested-capability` with no provider, never `inapplicable-kind`). TC-302 adds the selected item with its own index and identity.
  - No new family, verb, schema or code. The FR-492 library alternative remains.
- **Integration:** this PR's own patch is preserved.
  - The `-U0` added and removed lines of `9f5b30a5..9d89b90b` and `73e81dce..2f400c8b` differ only in: the neighbouring oneOf entry (main's `ProtocolAnalysisCounterexample` precedes our `InterfaceMismatch`; both are present); JSON trailing commas next to main's appended vectors; the renumberings FR-491-AC-7 to AC-9 and FR-331-AC-61/62/63 to 64/65/66, with their references (TC-985 and TC-986 scope, FR-491-AC-9 citing AC-64, AC-66 citing AC-64/65, BP-66 description, the two `#[trace]` attributes); and one missed cell (gap FND-002).
  - Main's FR-331-AC-61/62/63 and FR-491-AC-7/8 are present and unchanged, and the TC-985 scope excludes main's AC-7/8 exactly as on main.
  - No live reference to the old IDs remains in spec/ or tests/.
- **Mergeability:** GitHub reports PR 792 `isDraft: true`, `mergeable: CONFLICTING`, `mergeStateStatus: DIRTY`. Yet `baseRefOid` is 73e81dce and `git merge-base HEAD 73e81dce` is 73e81dce: the head strictly descends from the base, so no content conflict is possible. The state looks stale and must be refreshed, and the PR undrafted, before an admin merge.

## Integration (disposition pass 4)

Against 58f4a4df0c0e718af61468b5bec7482709936dc9, rebased onto main 9add8a0f (#798); base = merge-base = PR baseRefOid 9add8a0f.

- The `-U0` added and removed lines of `9add8a0f..58f4a4df` equal those of `73e81dce..2f400c8b`, except for the disposition-3 SR copies and the three fixed cells. No other owned hunk changed.
- Main's #798 FR-491-AC-7/8 rows are byte-identical to 9add8a0f, and our FR-491-AC-9 is retained, citing FR-331-AC-64.
- `git diff 2f400c8b..58f4a4df -- tests proposals src build.rs` is empty, and main's `73e81dce..9add8a0f` touches only FR-491-AC-8 and four historical SR files.
- GitHub now reports `mergeable: MERGEABLE`, `mergeStateStatus: BLOCKED`, `isDraft: true`. The #798 conflict was real and is resolved. BLOCKED/draft is left for the root (undraft) at merge.
