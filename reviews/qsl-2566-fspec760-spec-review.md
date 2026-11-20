---
id: SR-2026111922154500000001
title: "spec-review of QSL-2566 FR-322 checked_term_data.body_locations"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #760 at 54cecc9a; spec/objects/interfaces/FR-322-checked-package-artifact.md (checked_term_data carrier, body_locations paragraph, FR-322-AC-288, FR-322-AC-490), spec/test-cases/TC-2090-function-body-location-rows.md, proposals/checked-package-v2/schema.json, tests/checked_package_v2.rs; context: filament-contract-ir origin/main 2ec296a1 (checked_term_data.rs, FR-038, FR-019, FR-051), filament-contract-codegen origin/main a11a66ab, filament-driver origin/main 18d6fb0, FSL origin/main 8db8981d (emit.rs, protocol_locations.rs, reader_protocol_projection.rs, check.rs float_ordering)"
review_set: subset
---

## Summary

Spec review of FSpec PR #760. Ticket: QSL-2566 (implementation: QSL-2625). The PR adds a fifth required array, `checked_term_data.body_locations`, to the checked package. It holds one row per term of each function body walk, carrying the checker's `Body{function,index}` location and path. The strict reader must refuse a package that lacks any row. The PR adds FR-322-AC-490, TC-2090 and the schema definitions, and updates one carrier fixture line. Merge order: this PR first, then FSL #3114.

## Examined scope

- The lowering claim (examined, holds). In `check.rs` `float_ordering`, an operand that is not a `Local` or `Literal` is bound by a `let`, with the left operand outermost. So for `sqrt(p) <= sqrt(q)` the lowered tree holds the second `sqrt` at `[1, 0]`, while the checker located it at `[1]` when it checked the right operand. `reader_protocol_projection.rs` `locate_functions`/`locate_from` derive the location from the child index of the lowered tree, so they give `[1, 0]`. The rows are needed.
- Identity (examined, holds). The package `name` is a value supplied by the producer: `EmittedPackage::new(name, encode)` and contract-ir `package_name_is_stable_while_complete_admitted_value_changes`. No FR-322 identity is a digest over bytes, and TC-2050 / FR-322-AC-470 say a digest a consumer holds over the bytes is no identity. So "bytes change, `name` and `NodeRef` do not" is consistent. Contract-codegen's `select_checked_callable` compares re-encoded bytes. It encodes from the struct, so it stays equal once the struct has the field.
- Schema (examined, valid). The JSON parses. `BodySemanticLocation` copies `ProtocolSemanticLocation` member for member, and `NodeRef`, `Occurrence`, `KeyOrdinal` and `Nonempty` exist. `body_locations` is required and `additionalProperties` stays false.
- Version story (examined). There is no format version bump. Precedent: #567 added `probabilistic_claims` as a required fourth array in the same way, and the format is pre-release. Contract-ir `CheckedTermDataV2` is `deny_unknown_fields` with required fields. So every reader at today's pin refuses the new bytes as malformed, and the new reader refuses old bytes. The PR does not say this (FND-003).
- Readers of `checked_term_data` (examined). contract-ir: the model struct, encoder and validator (`checked_term_data.rs`, `dispatch_projection.rs`, `intake.rs`). contract-codegen: re-encode in `oracle/equality/mod.rs:790,4028` and `oracle/function/owner_path.rs:283`, which goes through the struct and needs only a re-pin, plus a test that drops the sidecar (FND-003). filament-driver and filament-contract-runtime: no reader. FSL: `fsl-package` emitter and readers, plus tests.
- Cargo claim (examined, not run, no builds per brief). `tests/checked_package_v2.rs` has 187 `#[test]` and 5 `#[ignore]`, so 182 run. No published fixture outside that file carries `checked_term_data`. The changed `tc_233` carrier is the only schema check of the carrier, so the claim is plausible.
- Ids (examined). FR-322-AC-490 and TC-2090 are on neither main nor open FSpec PRs #695 and #640.
- `quire validate` on both changed spec files exits 0. Its EARS warnings at line 108 are outside the diff.

## Verdict

Mergeable: yes. The rule is correct and needed: it is the only way the package route can return the checker's locations. The identity claim is consistent with FR-322. FND-001 and FND-002 should be fixed in this PR's fix round. FND-003 needs tickets (see the comment) and a sentence in the PR or ticket; it does not block the spec text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The completeness refusal ("a body-walk term with no row") defines the walk as FSL FR-383's. That walk runs over FSL's in-memory checked tree in `NodeKind` operand order and explicitly not over the node's `dependencies`. The FSpec reader (contract-ir) has only the wire. FR-322 must state the walk over the admitted body: which references are followed into application nodes, that a node reached twice is walked twice, that `let`/conditional are walked but are not terms, and that `function.call` is not entered. Otherwise two reader implementations can count terms differently | spec/objects/interfaces/FR-322-checked-package-artifact.md:1388-1404 |
| FND-002 | medium | The last clause of FR-322-AC-490 cannot fail. It says "node ids and `name` of BL-01 equal those of the same package with its `body_locations` emptied". That compares two hand-authored JSON documents that differ only in the emptied array. Also, the emptied package is refused by BL-R1's rule, so it cannot be read. Drop the clause, or move the identity check to the producer (FSL emits the same unit and compares `name`/ids with the rows stripped before encoding) | spec/objects/interfaces/FR-322-checked-package-artifact.md:5999 |
| FND-003 | medium | The format change and its consumers are not stated or ticketed. There is no version bump (same as #567, pre-release). Readers at today's pin (`deny_unknown_fields`) refuse the new bytes, and the new reader refuses old bytes and any sidecar-less package that has a function body with terms. Not ticketed: (1) the contract-ir spec says "exactly four required arrays" in FR-038-AC-390 and FR-038 l.5531-5590, and FR-019 (`CheckedTermDataV2` interface) and FR-051-AC-5 (encoder inputs) need updating; (2) contract-codegen's `source_context_comparison_distinguishes_checked_term_sidecar_presence` (FR-018-AC-39/42, TC-029) drops the whole sidecar of UNION_SOURCE, which has `a = b` function bodies, and panics unless that package is admitted. QSL-2625 covers only contract-ir code | spec/objects/interfaces/FR-322-checked-package-artifact.md:1212-1222 |
| FND-004 | low | BL-R4 ("gives one row of `f` index `1` and refuses at that row's `index`") is ambiguous. The rule refuses at the later row's `index`, so if the changed row is not the last of `f`'s rows, the locus is another row. Name the row: for example the last row, or the second `sqrt`'s row | spec/objects/interfaces/FR-322-checked-package-artifact.md:5999 |
| FND-005 | low | FR-322 now depends normatively on FSL FR-383 "Protocol term locations" for the body walk, but its frontmatter has no relationship to FSL FR-383 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1393 |
