---
id: SR-2026112115155546054652121
title: "gap-analysis of FR-364-AC-25 trace coverage for the closure-stutter contract"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-specification; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md (FR-364-AC-25); spec/test-cases/TC-295-temporal-counterexample-replay-refusals.md; tests/backend_provider_v1.rs; QSL-2247; branch at 7537ca96, PR agent-ix/filament-specification#795"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
---

## Summary

Ticket: QSL-2247. Planless gap audit, scoped to the change. Plan completion: not assessed.

- **Matrix.** I ran `quire matrix --scope .` (quire 0.52.0, engine 0.62.2) at 7537ca96. It reports FR-364-AC-25 as `untagged` with no binder. AC-4 and AC-5 (TC-295, replay semantics owned by the FSL consumer) are also untagged here, which is the existing pattern. AC-10 and AC-14 are tagged through `tests/backend_provider_v1.rs:533`/`:769`/`:634`.
- **Reverse gap.** The new schema alternative is owned by FR-364 (its description and the vector owners), so there is no ownerless code.
- **Stubs.** No source or test was edited, so there is no stub or coverage inflation.
- **Production consumer.** The replay checker is in FSL (`fsl-replay`, `ModelTraceSubject`), not in this repo. This repo's only executable consumer of the change is the schema qualification test.

## Verdict

**CONDITIONAL.** FR-364-AC-25 has an executable, schema-only clause, and this repo has a test that exercises it. That test is not traced to AC-25, and AC-25's Verification column omits TC-431. The replay clauses stay with the FSL TC-295 consumer, as AC-4 and AC-5 do.

Focused qualification the root must run (one batched build):

```
cargo test --test backend_provider_v1 -- --exact tc_431_positive_vectors_are_admitted_with_owned_members tc_431_adverse_vectors_are_refused
```

Do not treat the author's Python replica as this qualification. It could not compose the package bytes, so its six request-stage positives failed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-364-AC-25's last sentence ("The schema admits the `closure-stutter` step with exactly the `type` member in `ModelStep.transition` and refuses an extra member") is checked in this repo only by `tc_431_positive_vectors_are_admitted_with_owned_members` and `tc_431_adverse_vectors_are_refused`. Neither carries `#[trace("TC-431", "FR-364-AC-25")]`, and AC-25's Verification column names only TC-295, so `quire matrix` shows AC-25 untagged. The schema clause's evidence is then untraceable, and the FSL TC-295 consumer cannot bind it. Fix: split the schema sentence into its own AC verified by TC-431, tag both tests and add a TC-431 BP row for the four vectors. Or add TC-431 to AC-25's Verification column with the two trace tags. Adding the tags is a test-source edit in this PR. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:585; tests/backend_provider_v1.rs:517-533,750-769; spec/test-cases/TC-431-backend-provider-wire-vectors.md |

## Scope

- FR-364-AC-25 (examined; FND-001)
- FR-364-AC-4, FR-364-AC-5, FR-364-AC-10, FR-364-AC-14 matrix rows (context_only)
- tests/backend_provider_v1.rs tc_431 positive/adverse tests and their trace tags (examined)
- TC-295 steps 15-25 (context_only; grounding is SR-2026112115155546054652125)

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-364-AC-26 asserts that the schema "also admits the step in a `prefix`, in the `refinement_failure` prefix, with a `delay` and for an item without `stutter_closure`". The two TC-431 tests tagged to AC-26 check only the one positive (a sole-loop step) and its three adverse vectors, and BP-67 says "no schema check of them is claimed". The admission half of AC-26 is therefore asserted by the AC but not checked by the tests that claim it, so the AC-26 tag inflates coverage. Either add positive vectors for those placements, or reword AC-26 so it does not assert them: for example, "the schema does not see the item, subject, position or delay; refusals for those are AC-27's". | FR-364:607; TC-431:106; tests/backend_provider_v1.rs:527,762 |

## Dispositions

Round 1, against c70d088951937623a763d8dd492d0980d1bba7a6 (fix delta 7537ca96..c70d0889, one commit). Only the fix delta was verified; there was no fresh full review. No Cargo or runtime run.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Schema-only FR-364-AC-26 (Test TC-431), TC-431 BP-67, and the two #[trace] attributes. `quire matrix` at c70d0889 shows AC-26 as `tagged` (backend_provider_v1.rs:534, :771). Semantic AC-25/27/28/29 stay on TC-295, the FSL consumer, and untagged here, which is correct. |

Round 2, against ba5ad863daf7b6e3a30c84c267cf8f1ca83e6145 (fix delta c70d0889..ba5ad863, one commit; base 9f5b30a5 unchanged). Only the delta was verified; static checks only (`quire validate --strict` on FR-364, TC-295 and TC-431, exit 0). No Cargo or runtime run. The 15 findings fixed in round 1 were re-checked against the delta, and none regressed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | AC-26 now asserts only what the 1 positive and 3 adverse vectors check (exact `type` member admitted; extra member, misspelt type and missing `post_state` refused) and says "no vector asserts what it admits there". BP-67 and TC-295 step and expected 26 match. The `#[trace("TC-431","FR-364-AC-26")]` on the two generic tests is now honest. No semantics were lost: placement, delay, item and subject refusals are all in AC-27. |
