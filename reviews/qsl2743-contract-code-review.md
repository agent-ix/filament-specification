---
id: SR-2026112106460400000010
title: "Code review (data/fixture lane, rust-review baseline) of FSpec PR 789 FR-830 vectors and fixtures"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; proposals/checked-package-v2/model-union-witness-vectors.json; proposals/checked-package-v2/domain-package-acme-shapes-witness.json; proposals/checked-package-v2/fixtures/witness-model-union-rows.json; proposals/checked-package-v2/README.md; QSL-2743; PR 789"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: references
  - target: ix://agent-ix/filament-specification/TC-2112
    type: references
---

## Summary

Ticket: QSL-2743. PR agent-ix/filament-specification#789, frozen head fb65d9db6d9b3d886914690d37250b6228b4c683 over base d1ebfc724 (`git diff origin/main...HEAD`). The diff has no `.rs` or `Cargo.toml` change. The rust-review baseline lane applies only as conventions for the verification below, and no Rust production code was changed, so gap-analysis does not apply. This lane reviews the published inputs and expected results of the new FR-830 carrier: 16 positives, 21 schema refusals, 68 reader refusals and 7 budget cases. It checks that each vector can be constructed, that its oracle isolates the intended defect, and that the complete FCD document/package inputs are admissible.

Verification run (Rust only, outside the branch, one cargo job via `flock /tmp/cargo-slot.lock`, `-j 4`, `CARGO_TARGET_DIR=<FSpec worktree>/target`). Driver: a scratch test module added to an `git archive` copy of filament-contract-ir origin/main 743b5e5f1 at /tmp/ir-qsl2743-verify. It calls only the existing production strict read (`read_and_account` + `CheckedPackageEvidence::insert_domain_package_document`, the same path as `conformance_fspec::model_tuple_union::read_selected_variant_package`) and adds no carrier or reader logic. Focused targets were `--test conformance_fspec qsl2743_scratch::qsl2743_scratch_strict_read_of_fixture_pairs --exact` and `qsl2743_scratch::qsl2743_scratch_fixture_b_probes --exact`. Logs: /tmp/ir-qsl2743-spec-reviews/verify-strict-read-run2.log (exit 0; the first run, verify-strict-read.log, ran 0 tests because the `--exact` filter lacked the module path) and verify-fixture-b-probes.log (exit 0). Observed:

- Fixture A unpatched: ADMITTED. MUW-08 document_patch: ADMITTED.
- MUW-07 document_patch: Refused `invalid_timed_model`/`clock-read-outside-constraint` at `/semantic_graph/nodes/34`.
- MUW-09 document_patch: Refused `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/33`.
- Fixture B unpatched: Refused `unsupported_construct`/`declaration-form`, document pointer `/types/12/variants/1/payloadType` (Mix `Span`).
- B with `Span` respelled `ix://filament/unit/acme/shapes/ms`: Refused at `/types/12/variants/6/payloadType` (Mix `Held`, an object payload).
- B respelled and without `Held`: ADMITTED. B without `Span` and `Held`: ADMITTED.

The `Held` refusal is an existing engine limitation, not a defect of this PR. FR-322:2817 admits an object-type payload as `Reference<O>`, but IR `model_members.rs` (variant payload validity, the `SelectedTypeRef::Declared` arm) does not list `object_types`. The FCD `semantic-ir` intake of document B was not run, because no focused existing entry point takes an arbitrary document. No FR-830 carrier/reader exists, so no vector was executed as a join.

Checked clean: the declarations-row JSON shape and form spellings (`object_type`, `record_value_type`, `declared_unit`, exact_identity `{kind, value}`, effective `{chain:[{package,node}]}`, declared unit `{kind:"declared", node}`) are exactly FSL origin/main 7bb12494 `fsl-package/src/bare.rs` `DeclarationJson`/`IdentityJson`/`EffectiveJson`/`UnitJson`/`form_of`. All reached `closure_edges` counts (4, 4, 3, 6, 7, 18, 6, 2, 4, 4) match a recomputation from document B. Case census, positions and `original_case` identities match the documents for every positive except MUW-09. Every refusal's first-failing step was traced against FR-830 "Checks" order. `jq -e` passes on all three JSON files, and `git diff --check` is clean.

## Verdict

**CHANGES REQUIRED.** Several published positives cannot be admitted. Fixture B's unit payload is misspelled (FND-001). MUW-09's own row is out of canonical order (FND-002). Two document patches make fixture A's unchanged package refuse at the strict read (FND-003, FND-004). The key-depth vectors do not exercise the 32-key rule (FND-005). Every refusal inheriting one of these bases inherits the defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Fixture B names the model-owned unit as `ix://filament/unit/ms`. FR-322 names a selected package's unit `ix://filament/unit/<package>/<name>` (`ms` of `acme/rpc` is `ix://filament/unit/acme/rpc/ms`), and `ix://filament/unit/ms` names a unit-of-compilation unit. The production strict read refuses fixture B `unsupported_construct`/`declaration-form` at `/types/12/variants/1/payloadType`. So MUW-11 and MUW-15 and every refusal based on MUW-11 (R-39..R-43, R-49..R-55) have no admissible input. With the spelling `ix://filament/unit/acme/shapes/ms`, the read gets past `Span`. | proposals/checked-package-v2/domain-package-acme-shapes-witness.json:762; spec/objects/interfaces/FR-322-checked-package-artifact.md:2550; model-union-witness-vectors.json:2833,4099 |
| FND-002 | high | The MUW-09 positive lists Shape's cases as `Both, Tail, Empty, Flag` with ranks 0..3. That is not ascending UTF-8 order (`Tail` sorts last), so under FR-830 step 4 the positive itself refuses `mismatched_identity_witness` at `/model_unions/1/cases/2/name`. MUW-R-32, based on MUW-09, expects a step 6 refusal at `cases/1/payload/original_type`, which step 4 pre-empts, so its oracle does not isolate the intended defect. | model-union-witness-vectors.json:2087-2400,7053; FR-830:86-93,207-213 |
| FND-003 | high | MUW-07 changes only the document (`Shape.Flag` payload becomes Clock) over package A. Package A still holds a `union_value` `Flag` with a Boolean payload term (FR-322-AC-144) and a `structural.eq` over Shape. The production strict read refuses `invalid_timed_model`/`clock-read-outside-constraint` at `/semantic_graph/nodes/34`, and TC-2112 step 1 never reaches the join. The AC-4 Clock positive and R-23, R-24, R-26 and S-09 have no admissible input. | model-union-witness-vectors.json:1366; TC-2112 step 1; FR-830-AC-4 |
| FND-004 | high | MUW-09 adds `Shape.Tail(List)` to the document only. Package A's `case` over Shape still has exactly the arms `Both, Empty, Flag`, and FR-322-AC-144 says an arm missing refuses. The production strict read refuses `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/33`. The AC-1 `Tail(List)` clause and R-32 cannot run until the package (or a separate fixture) is consistent with the patched document. | model-union-witness-vectors.json:2087; FR-830-AC-1; FR-322-AC-144 |
| FND-005 | medium | The depth vectors do not exercise the "nested beyond 32 keys" rule. MUW-S-11 replaces `original_type` with 41 nested `option` types around `boolean` and contains no original node key at all. FR-830 states no value-type depth bound, so its `malformed` expectation is not derivable from the contract. MUW-TS-04 replaces `node_key` with that value-type chain, which is not a NodeKey shape, so it refuses as "not a key" exactly like TS-03. A reader with no 32-key depth check passes both. A key with 33 genuinely nested keys (and a 32-deep admitted control) is needed. | model-union-witness-vectors.json:5395,5856; FR-830:66-68,178-181; FR-830-AC-6; FR-830-AC-14 |

## Scope

- FR-830-AC-1, AC-4, AC-6, AC-14 vectors (examined; FND-001..FND-005)
- FR-830-AC-2, AC-3 reached positives and closure_edges (examined, clean)
- FR-830-AC-5 MUW-08 (examined, clean; strict read admits)
- FR-830-AC-7, AC-8 refusal vectors R-01..R-18 except R-32 (examined, clean)
- FR-830-AC-9, AC-10, AC-13, AC-15 refusal vectors (examined; clean apart from inherited bases)
- witnesses.declarations encoding vs FSL bare.rs (examined, clean)
- README.md entry (examined, clean)
