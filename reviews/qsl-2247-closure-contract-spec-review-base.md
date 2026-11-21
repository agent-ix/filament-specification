---
id: SR-2026112115155546054652122
title: "spec-review/base of the FR-364 closure-stutter replay contract"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md (Shapes, model-arm step, Closure stutter, Named limit, Replay steps 2/4/5, agreement, FR-364-AC-25, Dependencies); QSL-2247; branch at 7537ca96, PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
  - target: ix://agent-ix/filament-specification/FR-368
    type: references
---

## Summary

Ticket: QSL-2247. I checked the change against the approved contract. It names one closed `ModelStep.transition` `{type: closure-stutter}`, valid only as the sole loop step of a `kind: formula` lasso and selected by the item's obligation identity (FR-368 `stutter_closure`). The step is non-terminal and its `post_state` is the entry key. There is no prefix use, no mixing with other step kinds, and no new request or envelope member. Replay recomputes enabledness, so any enabled constraint makes the lasso unfair. An evaluator `true` gives `replay-parity` and a `false` gives `reproduced-with-evaluated-witness`, and the core gates `refuted` on replay. Wrong flag, terminal state, prefix use, mixing and a post-state tamper are refused with typed refusals. Plain `terminal-stutter` is unchanged. The text names the mixed-loop edge-drop limit.

Clean:
- Every element of that contract is present and worded as approved.
- The flag binds through the item identity, never through counterexample metadata ("the counterexample, the envelope and the request carry no flag of their own").
- The fairness rule matches FR-362's lasso rules at a one-state loop: weak means enabled at every loop state and never taken; strong means enabled at some state and never taken.
- The timed, task and protocol alphabets and the continuation rules are explicitly not widened.
- The named limit is accurate. A projection that drops the closure stutters is a fair lasso of the subject itself, so a `false` there is a sound refutation and a `true` is parity. The limit claims no preservation outside that case.

## Verdict

**CONDITIONAL.** The contract is sound. Two normative-precision gaps (FND-001, FND-002) need fixing before the FSL FR-128/FR-1030 work builds on it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Inputs (FR-364:42-46) still lists only "the item's clause and resolved fairness set" from the item. The new alphabet rule makes the item's `stutter_closure` member (its obligation identity) a replay input, and the Closure stutter paragraph says replay "already resolves" it. The normative input list does not say so. An implementer building from Inputs (FSL `ModelTraceSubject`) can omit the flag, or read it from free counterexample metadata. Add the item's `stutter_closure` membership, bound through the FR-331 counterexample entry's item, to Inputs. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:42-46,297-301 |
| FND-002 | medium | Replay step 5 says a `closure-stutter` loop "denotes the trace that repeats the loop's state and its letter, as a terminal stutter does", and that "A letter undefined there is the undefined-evaluation rule of step 6, not a formula value". There are three problems. (a) "letter" is defined neither here nor in FR-361. FR-361 defines the terminal stutter position as the terminal state's observation, "anchored by the stutter step", and says it "enables no operation". So "as a terminal stutter does" can import "enables no operation" at a non-terminal state, contradicting step 4, which uses that state's enabled identities. (b) The anchor of a closure-stutter position is not stated. (c) Step 6 applies only to the undefined-evaluation envelope kind. For a `kind: formula` counterexample, "any other result disagrees" already gives `replay-parity`. The new sentence lets one implementer refuse, another settle parity, and a third treat the counterexample as an envelope. Fix: say that each closure-stutter position reads the loop state's observation, anchored by the closure-stutter step, with enabledness that of the model state; and that an undefined value at `trace_position` disagrees (`replay-parity`). | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:514-523; spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:161-167 |
| FND-003 | low | The Shapes row allows the closure lasso for an item "whose only violating loop is the stutter closure's own stutter". "only" is a condition on the engine's choice among counterexamples that replay cannot check, and it conflicts with "Validity, not identity" (FR-364:547-551). Replay accepts any valid closure lasso. Drop "only", or say "a violating loop that is". | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:64 |
| FND-004 | low | FR-364-AC-25 is one compound criterion with about 13 outcomes: 2 positive, 7 refusal kinds, terminal control, mixed-loop parity and a schema clause. It also uses an elided pointer, `/…/transition/type`. A failure cannot be attributed to a clause, and the pointer does not say whether the prefix case refuses at `/prefix/<i>` or at `/loop/<j>`. Write the full pointers. Consider splitting the schema clause out (see SR-2026112115155546054652121 FND-001). | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:585 |

## Scope

- FR-364 Shapes Liveness row (examined; FND-003)
- FR-364 model-arm step paragraph (examined, clean)
- FR-364 Closure stutter paragraph (examined; FND-001 context)
- FR-364 Named limit paragraph (examined, clean for accuracy; writer completeness is SR-2026112115155546054652125 FND-001)
- FR-364 Replay step 2 closure refusals (examined, clean except the delay rule, SR-2026112115155546054652123 FND-001)
- FR-364 Replay step 4 fairness addition (examined, clean)
- FR-364 Replay step 5 (examined; FND-002)
- FR-364 agreement paragraph (examined, clean)
- FR-364 Inputs (examined; FND-001)
- FR-364-AC-25 (examined; FND-004)
- FR-368 Request members `stutter_closure`, obligation identity (context_only)
- FR-380 Safety and liveness premises (context_only)
- FR-362 Enabled and taken; Weak and strong fairness (context_only)
- FR-361 Terminal states (context_only)

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inputs now names the item's `stutter_closure` member, part of the obligation identity and never read from the counterexample. |
| FND-002 | fixed | Step 5 defines the observation, the anchor (the closure-stutter step, as FR-361 anchors a terminal stutter) and the model state's own enabledness. An undefined value gives `replay-parity`, and step 6 reads only the envelope. The 'letter' wording and the 'as a terminal stutter does' import are gone. |
| FND-003 | fixed | 'only violating loop' becomes 'may be refuted by a violating loop that is…'. |
| FND-004 | fixed | Split into AC-25 (positive, parity, terminal control), AC-26 (schema), AC-27 (refusals with full pointers), AC-28 (fairness) and AC-29 (mixed-loop limit). |
