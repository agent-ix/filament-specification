---
id: SR-2026112106460400000014
title: "spec-review/scope-boundary review of FR-830 sidecar vs native boundary"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-830-carry-selected-model-union-witness-rows.md; spec/objects/interfaces/FR-322-checked-package-artifact.md (Model unions paragraph); proposals/checked-package-v2/model-union-witness-vectors.json; QSL-2743; PR 789"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: references
---

## Summary

Ticket: QSL-2743. PR agent-ix/filament-specification#789 at fb65d9db. This review checks the allocation of responsibility between the FSL producer sidecar, the IR native typed-row join and the FSpec contract.

Clean:
- `checked_term_data` and the I04 schema are unchanged.
- `model_unions`/`model_tuples` are identity-sidecar members, not package members.
- FSL owns the JSON parse, the original-key and value-type codecs, the `declarations` rows and `SelectedTupleDeclarationRow`.
- FR-358 owns the budget amounts and the locus source, and FR-352 owns the Exact projection.
- No FSL version or pin is chosen.
- The carrier adds no digest, signature, pin or catalog (no hash/pin antipattern).
- Existing `source_unions`/`source_tuples` keep their order and are never reused.
- The vectors' `witnesses.declarations` shape matches FSL origin/main `bare.rs` `DeclarationJson`.
- The FR-322 addition only points to FR-830.

## Verdict

**CONDITIONAL.** The "no source-row fallback" boundary is stated but has no constructible input (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-15 requires an absent model-tuple row to be `unavailable_identity_witness` "whether ... the file's `source_tuples` member carries a row for the node", and AC-10 requires "no ... source-union fallback". No vector carries a `source_tuples` or `source_unions` row. TC-05 only names `source_tuples` in `file_members` prose, and the sidecar object of every case is `{model_tuples, model_unions}`. A reader that falls back to source rows passes TC-05 because there is nothing to fall back to, and the author's report states that the source-row encoding is undescribed. Supply concrete existing-codec `source_tuples` and `source_unions` rows for the affected cases, or narrow the AC clauses to what the vectors can check. | FR-830:49-60,119-122,239-241; FR-830-AC-10; FR-830-AC-15; model-union-witness-vectors.json:7443 (MUW-TC-05) |

## Scope

- FR-830 Carrier and scope, Encodings (examined, clean)
- FR-830 Side-input witnesses (examined, clean)
- FR-830 Work and storage ownership (examined, clean)
- FR-830 Authority boundary (examined, clean)
- FR-830-AC-10, AC-15 (examined; FND-001)
- FR-322 model union paragraph addition (examined, clean)
- FSL fsl-package/src/bare.rs DeclarationJson (context_only)

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The fix narrows the source-tuple guarantee by rewriting the normative identity-file contract to match a lagging codec. FR-830 Dependencies (344-351) now says the identity file's members are the seven `format`, `declarations`, `fields`, `source_records`, `source_unions`, `source_objects` and `source_operations`, "so there is no `source_tuples` file member". AC-15 says "a file that carries one is `Malformed` (MUW-TS-09)". MUW-TS-09 adds an empty `source_tuples: []` and expects `malformed` at `/source_tuples`, which is only an unknown-member failure. This contradicts four sources. (1) Normative FSL origin/main spec/functional/FR-531-evaluate-from-a-checked-package-read-from-its-bytes-alone.md:340-358 requires `filament.checked-handoff.identities/5` with "All eight members", including `source_tuples`, whose `SelectedSourceTupleRow` wire is `{structural_node, source_authority, source_identity, qualified_declaration, type_handle, positions[{ordinal, type_node, value_type, member_type, collections}]}`. Under that contract an absent `source_tuples` is the refusal, not a present one. (2) FR-830's own lines 55-56 ("The existing `source_unions` and `source_tuples` members and their order are unchanged") and 125-127 and 259. (3) The adopted arch ruling, which keeps `source_tuples` separate and unchanged. (4) The Dependencies paragraph's own "None is restated". That FSL `fsl-package/src/bare.rs` still writes and reads `identities/4` with seven members is implementation lag, like the IR `Held` gap: it cannot redefine the normative channel as malformed. Fix: remove the seven-member sentence and the Malformed clause from AC-15, and drop or replace MUW-TS-09. The file format version stays FSL's choice and is not preselected. | spec/objects/interfaces/FR-830-carry-selected-model-union-witness-rows.md:55-58,125-127,259,332,344-351; proposals/checked-package-v2/model-union-witness-vectors.json (MUW-TS-09); filament-spec-language spec/functional/FR-531-evaluate-from-a-checked-package-read-from-its-bytes-alone.md:340-358 |

## Dispositions

Round 1, against 63a7f10abfdf8e8876bb2fabfb039f443631e93f (fix delta fb65d9db..63a7f10a).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Union half fixed, tuple half not. Fixed for unions: fixture C adds the source union Route (`composite_type`/`union`, SourceOwner), and MUW-19 carries a valid `source_unions` row exactly in FSL origin/main `fsl-package/src/bare/carried.rs` `SourceUnionJson`/`SourceUnionMemberJson` form, with case names overlapping Link's. MUW-19 admits Link's model row beside it, and R-59 refuses `unavailable_identity_witness` at the Link node and `/model_unions` when the model row is missing; the strict read admits C with Held removed. Still open for tuples: no vector carries a valid normative `source_tuples` row (FR-531 `SelectedSourceTupleRow`) beside an absent model-tuple row. TC-05 still only empties `model_tuples` with nothing to fall back to, and TS-09 tests an unknown-member failure (FND-002), so a reader that lets a source-tuple row supply a model tuple still passes. Missing: a package with a source tuple composite and its valid `source_tuples` row (for example with a `type_handle` equal to the missing model tuple's `node_key`, or a qualified name overlapping it), with the model row absent and the first refusal at the model join: step 6 `unavailable_identity_witness` at the payload node, or step 7 at the node and `/model_tuples`, never `Malformed`. AC-15 should keep its source/model non-substitution clause. |
