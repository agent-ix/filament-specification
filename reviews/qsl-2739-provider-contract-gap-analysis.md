---
id: SR-2026112114454020075413601
title: "Gap analysis: PR 792 schema rules and new acceptance criteria"
type: SpecReview
analysis: gap-analysis
date: 2026-11-21
scope: "agent-ix/filament-specification; PR 792 git diff 9f5b30a5...35b3b0e0; proposals/backend-provider-v1/schema.json, tests/backend_provider_v1.rs, proposals/backend-provider-v1/vectors.json, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md, spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md, spec/objects/interfaces/FR-492-planner-settlement-request.md; QSL-2739"
review_set: subset
---

## Summary

Ticket: QSL-2739. Method: gap-analysis. Reviewer run 4b0eb3f0-7e32-4a5e-ad42-0e821771b4c3, model claude-opus-5-5, quire 0.52.0 (engine 0.62.2), quoin 0.36.8. Frozen head 35b3b0e0a133f8288b5d252f371c5e43b7437788 over base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f.

Planless, PR-scoped. `quire matrix --scope . --format json` (quire 0.52.0) reports every new or changed AC (FR-331-AC-61, FR-331-AC-62, FR-489-AC-5, FR-489-AC-12, FR-491-AC-7, FR-492-AC-10, FR-492-AC-13, FR-492-AC-14) as `untagged`. Their siblings FR-489-AC-11, FR-491-AC-6 and FR-492-AC-12 are untagged too, because TC-983/985/986 run at FSL's local Rust boundary, not in this repo. The FR-331 schema-level siblings FR-331-AC-58 and AC-60 are tagged in `tests/backend_provider_v1.rs` (TC-431). `vectors.json` holds no item with `premises` (count 0), so no existing vector breaks, and no vector exercises the new schema rules. No reverse gap (every schema change has an owning FR-331/FR-491/FR-492 clause) and no stubs. Semantic review: not requested, skipped. Plan completion: not assessed.

## Verdict

**FAIL** under this skill's rule (any untagged criterion). The untagged FR-489/491/492 ACs follow the existing cross-repo TC pattern and are consumer work for FSL PR2 (see report). The gap actionable in this repo is FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new schema rules have no TC-431 row, vector or tagged test. They are: `premises` requires `assumptions`, `assumptions` only on a `composition` item with `premises`, the closed `InterfaceMismatch` counterexample, and the closed `provider-answer-unconfirmed` cause. FR-331-AC-61/62 route everything to TC-986 (FSL), although AC-61's non-composition and without-premises clauses are schema-decidable here, as AC-58 to AC-60 are. Needed: an FR-331 schema AC (Test TC-431) plus a BP row, with positive vectors (a composition item with premises and assumptions; a refuted composition entry with an `interface-mismatch` counterexample; an inconclusive `provider-answer-unconfirmed` entry) and adverse vectors (premises without assumptions; assumptions on another kind or without premises; a non-integer inner index; InterfaceMismatch missing a member, with `reason` other than absent/weaker, or with an extra member; the cause without `premise` or with an extra member). Run only `cargo test --test backend_provider_v1 tc_431`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:1144-1145; schema.json 314, 321-322, 908-920, 1126 |

## Scope

- FR-331-AC-61 (examined; FND-001)
- FR-331-AC-62 (examined; clean)
- FR-489-AC-12 (examined; clean)
- FR-491-AC-7 (examined; clean)
- FR-492-AC-13 (examined; clean)
- FR-492-AC-14 (examined; clean)
- schema-allOf-assumptions (examined; clean)
- schema-InterfaceMismatch (examined; clean)
- schema-provider-answer-unconfirmed (examined; clean)
- FR-489-AC-5 (context only)
- FR-492-AC-10 (context only)

## Dispositions

Round 1, against bec3059e731621e5cfb63144c37a3eed3da851cc (fix diff 35b3b0e0..bec3059e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-331-AC-63 (Test TC-431), BP-66, 3 positive and 9 adverse vectors in vectors.json, and `#[trace("TC-431","FR-331-AC-63")]` on the two generic TC-431 loops. `quire matrix` now reports FR-331-AC-63 `tagged` by both tests. Runtime result is pending (Draco). |
