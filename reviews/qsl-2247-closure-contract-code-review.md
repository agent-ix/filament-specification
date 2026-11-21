---
id: SR-2026112115155546054652120
title: "code-review of the FR-364 closure-stutter schema alternative and vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; proposals/backend-provider-v1/schema.json ($defs.ModelStep.transition); proposals/backend-provider-v1/vectors.json (temporal-outcome-closure-stutter + 3 adverse); QSL-2247; branch terra/missing-engines-forms/qsl-2247-closure-contract at 7537ca96 (base 9f5b30a5); PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
---

## Summary

Ticket: QSL-2247. The PR has not been opened yet; this review is against the frozen head 7537ca96317a2715da23614ecea6932d8524251f on base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f (4 files, +225/-6, clean tree). This is a JSON schema and vector change only; no Rust is touched, so the rust-review lane does not apply.

Clean:
- The new `oneOf` alternative `{type: const closure-stutter}`, `required [type]`, `additionalProperties: false` is closed. It cannot overlap `TransitionIdentity`, because every `TransitionIdentity` branch pins `type` to `transition`, `protocol-transition` or `task-transition`, nor the `terminal-stutter` alternative. So `oneOf` admits exactly one branch.
- `post_state` stays required at `ModelStep` level. The `else` branch of the arms coupling also requires `transition` and `post_state` on every model-arm loop step.
- The positive vector mirrors the existing `temporal-outcome` positive: the same subject, the same state key and the same owner map, with the loop transition changed only.
- The three adverse vectors are single patches. Under `located_at_patch` each is refused at or above the patched member: the operation member, the underscore spelling, and the missing `post_state`, which is reported at its parent `/loop/0`.
- The `terminal-stutter` positive is untouched.
- Reviewer's own check with the Python `jsonschema` library: a `ModelStep` sub-schema admits `{closure-stutter}`+`post_state` and `terminal-stutter`, and refuses the `operation`, `closure_stutter` and no-`post_state` forms. This is not the Rust qualification.
- The schema still admits a `delay` member on any `ModelStep`, so the delay refusal is replay's, not the schema's. See SR-2026112115155546054652123 FND-001.

## Verdict

**APPROVE WITH LOWS.** The schema shape is correct and closed. The two lows are documentation and scope notes. The Rust vector check has not been run; see SR-2026112115155546054652121 for the exact target.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `$defs.ModelStep` is also the item type of `RefinementFailure` `mapping-undetermined.prefix` (FR-379). The new alternative therefore also admits `{type: closure-stutter}` in a refinement-failure prefix. FR-364 forbids the step in any `prefix`, and a `refinement` item never carries `stutter_closure` (FR-368). Neither the FR-364 text, the schema description nor TC-295 step 25 names this second admission point, and no case states that FR-379 replay refuses it through the FR-364 alphabet. State it in the description or the TC-295 step 25 list of schema-admitted, replay-refused placements. | proposals/backend-provider-v1/schema.json:602,1107 |
| FND-002 | low | The alternative's `description` lists the replay refusals: no `stutter_closure`, outside a sole formula loop, terminal state, another entry key. It omits two that the FR text also makes replay's: a timed, task-derived or protocol subject, and a `delay` member. A reader taking the list as exhaustive would miss them. Complete the list or say "for example". | proposals/backend-provider-v1/schema.json:602 |

## Scope

- schema.json `$defs.ModelStep.transition` new alternative (examined; FND-002)
- schema.json `$defs.TransitionIdentity`, `TraceStep`, `TemporalCounterexample` arms coupling (context_only)
- schema.json `$defs.RefinementFailure` `mapping-undetermined.prefix` (examined; FND-001)
- vectors.json `temporal-outcome-closure-stutter` positive (examined, clean)
- vectors.json `closure-stutter-with-an-operation-member`, `closure-stutter-spelled-with-an-underscore`, `closure-stutter-without-a-post-state` (examined, clean)
- tests/backend_provider_v1.rs `tc_431_positive_vectors_are_admitted_with_owned_members`, `tc_431_adverse_vectors_are_refused`, `located_at_patch` (context_only)

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

Rust-review lane (attribute-only change in tests/backend_provider_v1.rs): `#[trace("TC-431", "FR-364-AC-26")]` was added to `tc_431_positive_vectors_are_admitted_with_owned_members` (line 527) and to `tc_431_adverse_vectors_are_refused` (line 762), inside each function's existing `#[trace]` stack and above `#[test]`. It uses the same macro form as its neighbours, with no logic change, so the lane is clean. `quire matrix` binds FR-364-AC-26 to tests/backend_provider_v1.rs:534 and :771. Not compiled; the root runs the two tests.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR text, AC-26, TC-431 BP-67 and TC-295 step 26 now name the RefinementFailure prefix admission and the replay alphabet refusal there. |
| FND-002 | fixed | The description now lists the item, subject classes (including hybrid), prefix, RefinementFailure prefix, delay, terminal and entry-key refusals as 'for example', and cites AC-27. |
