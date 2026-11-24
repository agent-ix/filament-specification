---
id: SR-2026112402322100000001
title: "spec-review/integrity-analysis of FSpec PR 828 missing ACs (QSL-2774/2778/2779/2799/2801/2991/3001/3016)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR 828 at c46160c5 (ade8f999 plus FR-375 fix); spec/functional/analysis/FR-375,FR-377,FR-408,FR-411,FR-420,FR-712,FR-740; spec/functional/temporal/FR-540; spec/objects/interfaces/FR-322,FR-412,FR-464,FR-760,FR-825,FR-829; proposals/backend-provider-v1/schema.json; TC-310,TC-362,TC-374,TC-433,TC-946,TC-1160,TC-1742,TC-1787,TC-1830,TC-2053,TC-2084"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-412
    type: references
  - target: ix://agent-ix/filament-specification/FR-420
    type: references
  - target: ix://agent-ix/filament-specification/FR-740
    type: references
  - target: ix://agent-ix/filament-specification/FR-825
    type: references
---

## Summary

Ticket: QSL-2778 (also QSL-2774, QSL-2779, QSL-2799, QSL-2801, QSL-2991, QSL-3001, QSL-3016). This review covers PR agent-ix/filament-specification#828. It was started at ade8f999 and finished at head c46160c5, which changes only FR-375 and TC-310 step 33. The review checks whether each new AC can fail, whether the new text contradicts neighbouring FRs and ACs, AC ID allocation, consistency between the schema and the text, and EARS form. It also cross-checks the FR-322/FR-825 ordinal and pruning text against the filament-contract-ir reader at 729a6641.

Clean:
- Every new AC ID is the next free number in its FR, and each occurs exactly once.
- The `components[].policy` schema addition matches the FR-412 text: it is optional and has the same row shape as the top-level `policy`.
- FR-322-AC-522 and FR-825-AC-11 (the termination measure is walked after its owner's body) match the 729a6641 reader `derive_package_path_conditions`, which walks the body and then `function_measure_root`.
- FR-375-AC-36 agrees with the FR-375 `HistoryInitial` wire row.
- FR-375-AC-37 and FR-375-AC-38 at c46160c5 agree with FSL FR-140-AC-17 and FR-140-AC-18. The ade8f999 `conflicting-binding` contradiction is resolved.
- FR-540-AC-4, FR-760-AC-4, FR-760-AC-5 and FR-829-AC-8 and FR-829-AC-9 are testable and consistent with their neighbours.
- `quire validate` passes on every changed file; only warnings are reported.

## Verdict

**NOT MERGEABLE.** Two high findings: FR-412-AC-28 contradicts FR-412-AC-24/25, and FR-420-AC-10 cannot be satisfied. There are also eleven medium findings and seven low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-412-AC-28 and the "Replay of an initial-state verdict" text refuse "a part whose proof needs a ranking over the cyclic graph". This contradicts FR-412-AC-24, FR-412-AC-25 and the cyclic `lower` role text, under which `lower` certificates with progress ranks over cyclic induced chains are accepted. No refusal code or rule is named either. Limit the refusal to the profiles that require an acyclic order (the fair-minimum profile) and name the code. | FR-412:462-471; FR-412-AC-28; FR-412-AC-24; FR-412-AC-25 |
| FND-002 | medium | FR-412-AC-28 asserts the initial verdict is "decided before any graph ordering", an internal order that no test can observe. It also does not say which evidence replays: FR-412-AC-25 and TC-433 do not accept a path set for cyclic claims. Name the certificate kind and the observable outcome instead. | FR-412-AC-28; TC-433 step 6 |
| FND-003 | high | FR-420-AC-10 cannot be satisfied with FR-420-AC-9's `Up`/`Down` model (Up 1 ms, Down 3 ms). If the step out of Down stops resetting the clock, the run returns to Up with x = 3 ms. If the step into Down stops resetting, the run returns with x = 0. A return "with x = 1 ms" therefore never occurs. In the first variant x never returns to 0, so no "cut at the position where the key equals the initial one" exists either. TC-374 ST-09 repeats this. | FR-420-AC-10; TC-374 ST-09 |
| FND-004 | medium | Under "Delay-weighted cycles", a cycle that ends in a terminal state "contributes that limit as a completed run". However, FR-408's regenerative method samples one behaviour and estimates sum(Y_k)/sum(L_k) with a Chow-Robbins half-width. The text gives no rule for combining a 0/1 run limit with earlier cycle ratios or with the half-width, so FR-420-AC-11 has no defined estimate. | FR-420:162-176; FR-420-AC-11; FR-408:176-198 |
| FND-005 | medium | FR-740 requires an accepted undelayed failing edge to carry `taken` `null`. FR-418 "Refinement part" defines `taken` `null` only for an edge into or inside the locked region, and FR-418 is not amended. The checker therefore cannot tell the two cases apart, and FR-740-AC-6 tests a wire meaning that FR-418 contradicts. | FR-740:91-104; FR-740-AC-6; FR-418:274 |
| FND-006 | medium | Per-candidate clock blocks are "part of the candidate's state key", and FR-740-AC-4 requires a certificate position to list two candidates of one abstract state. FR-418 `positions[].abstract_states` is a sorted list of abstract StateKeys with no clock-block member, so the two entries are indistinguishable duplicates. The wire shape is undefined. | FR-740:45-66; FR-740-AC-4; FR-418 Refinement part |
| FND-007 | medium | FR-740-AC-5 accepts "settles `proved` or `refuted` ... and never `unsupported`". A wrong verdict passes. State the verdict the vector must settle. | FR-740-AC-5 |
| FND-008 | medium | The FR-412 body scopes the deterministic stationary witness to "no fairness set". FR-412-AC-27 extends "the same witness shape" to a fairness set, where a deterministic stationary scheduler need not be fair. The text has no fairness check for the witness. | FR-412:446-460; FR-412-AC-27 |
| FND-009 | medium | FR-411-AC-18's vector is "`LongRun` with an added action" choosing between components valued 1/2 and 3/4. LongRun's own component has the value 1800/1801. If that component stays reachable, the maximum is 1800/1801 and `<= 3/4` under `Every` is refuted, not proved. TC-362 LR-06 uses a different, unnamed model. The criterion also calls component values "gains", whereas FR-412 fixes gain = 0 at the accepted value. | FR-411-AC-18; TC-362 LR-06; FR-412:304 |
| FND-010 | medium | The new "Branches with no scalar operation" paragraph decides frame existence by the scalar-operation application. The existing producer rule ("no scalar operation occurrence that produces a record") and the 729a6641 reader (`derived_frame_id` returns `PRUNED_FRAME` when no row exists) decide it by record. When a branch holds a scalar application whose record is omitted (the FR-322-AC-522 and FR-825-AC-11 case), the new text requires a frame that the orphan rule then refuses. Key the paragraph on records, or drop it as a duplicate of the producer bullet. | FR-825:165-180; FR-825:382-386; FR-825-AC-12 |
| FND-011 | low | FR-825-AC-12 says "yields an empty `path_condition_frames` member that is absent", which is garbled. The member is absent only when every record's chain is empty (FR-825 "Shared path-condition frames"), not for each guard. | FR-825-AC-12; FR-825:118-121 |
| FND-012 | medium | FR-464-AC-27 and the new body text refuse "a plain `reference` from `Loop`'s control to `Loop`'s own `node_id`". FR-464-AC-15 and FR-464:106 say that in-component references are ordinary `reference`s to member `node_id`s on the checked wire, with `group_reference` used only in the written form. As written, the refused form is the admitted wire form. Say that the term must go through the carrier. | FR-464:744-763; FR-464-AC-27; FR-464-AC-15 |
| FND-013 | low | FR-464-AC-26 says "refuses `invalid_semantic_graph` at ... `/dependencies` or the typed refusal of FR-322-AC-66". Map each mutation to its single outcome: removal to `/dependencies`, and retarget to `ill_typed`/`operator-ineligible`. | FR-464-AC-26 |
| FND-014 | medium | FR-829 "State key and wire form" and FR-829-AC-10 cite "the FR-181 typed value of kind `clock_reference`". FR-181 defines no such kind, and no runtime-value schema carries `{"clock_reference": ...}`. FSL FR-1040-AC-12 inherits this. | FR-829:197-205; FR-829-AC-10; FR-181 |
| FND-015 | low | At c46160c5, FR-375 says "the step fault that FR-141 states". In this repo FR-141 is "evaluate text and enumerations". The intended target is FSL FR-141, so use an ix:// link or an "FSL" prefix. | FR-375:374-379 |
| FND-016 | low | FR-377-AC-18 cites TC-312, but TC-312 does not scope it; the coverage is in TC-310 step 33. | FR-377-AC-18; TC-312; TC-310 step 33 |
| FND-017 | low | The TC-946 K26 and K27 rows have 4 cells in a 3-column table. | TC-946:128-129 |
| FND-018 | low | The new FR-740 sentence ("the checker SHALL accept it when ... SHALL NOT reject ...") raises quire `ears:missing-subject` and `ears:unclassifiable`. | FR-740:87 |
| FND-019 | low | The FR-412 member tables (lines 36, 51, 66) still describe `components` as value plus gain-bias pair, with no `policy`. No rule covers a missing or extra component entry. | FR-412:36-66; FR-412:408-443 |
| FND-020 | low | FR-712 normative text holds a vector-specific sentence ("a `Replied` claim over a Zeno-only subject settles with `zone-certified`"). FR-712-AC-5 refutes `TimedOut`, while the companion FSL FR-852-AC-7 refutes `Late` for the same vector. | FR-712:100-102; FR-712-AC-5 |

## Scope

- FR-375 receiver and most-specific typing text, FR-375-AC-36..38 (examined)
- FR-377-AC-18, FR-408-AC-20, FR-411 one-solver text, FR-411-AC-18 (examined)
- FR-412 "Long-run certificate wire shape and witnesses", FR-412-AC-26..28 (examined); FR-412-AC-24, FR-412-AC-25, FR-412 cyclic `lower` role (context)
- FR-420 regeneration and delay-weighted text, FR-420-AC-9..11 (examined); FR-408 Regenerative method (context)
- FR-540 read-back text, FR-540-AC-4 (examined, clean)
- FR-712 Zeno-only text, FR-712-AC-5 (examined); FR-418-AC-16 (context)
- FR-740 clock blocks, tick source, undelayed failing edges, FR-740-AC-4..6 (examined); FR-418 Refinement part (context)
- FR-760 text, FR-760-AC-4, FR-760-AC-5 (examined, clean)
- FR-322-AC-522, FR-825 pruned-branch text, FR-825-AC-11, FR-825-AC-12 (examined); contract-ir 729a6641 requirement_records.rs (context)
- FR-829 runtime value text, FR-829-AC-8..10 (examined)
- FR-464 protocol references text, FR-464-AC-26, FR-464-AC-27 (examined); FR-464-AC-15 (context)
- proposals/backend-provider-v1/schema.json `components[].policy` (examined, clean)
- TC-310, TC-362, TC-374, TC-433, TC-946, TC-1160, TC-1742, TC-1787, TC-1830, TC-2053, TC-2084 additions (examined)
