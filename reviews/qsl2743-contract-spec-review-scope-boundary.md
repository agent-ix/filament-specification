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
