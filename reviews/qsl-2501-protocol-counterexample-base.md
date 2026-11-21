---
id: SR-1795275094665040523
title: "Base review of the protocol-analysis canonical counterexample contract"
type: SpecReview
org: agent-ix
analysis: base
review_set: base
scope: "filament-specification: changed FR-331 protocol-analysis counterexample body and FR-331-AC-61 through FR-331-AC-63; proposals/backend-provider-v1/schema.json and vectors.json"
---

## Summary

PASS for the bounded contract change. The new closed canonical alternative preserves the current native realizability and projection evidence, retains the existing result-record/body boundary, and explicitly assigns structural checks and native truth checks to their respective owners. No correction is required within this change.

This is one independent base checklist review before implementation. It does not accept an implementation, establish runtime behavior, close QSL-2501, assess general assurance readiness, or discharge code-review or gap-analysis. The existing source handover still needs the separately owned implementation and runtime acceptance work.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No defect found in the scoped contract delta. | FR-331 protocol-analysis counterexample body; FR-331-AC-61; FR-331-AC-62; FR-331-AC-63 |

## Review scope and authority

Reviewed the three-file delta in `/home/peter/dev/worktrees/protocol-batch-2-qsl-2501-counterexample-contract`, frozen at `550d97404abd85ad3be22e1b449e2a83b8aceb2b`, relative to `bca4311d`. Git status was clean. These revision names identify review inputs only; the contract adds no file revision tracking requirement. Unchanged historical requirements in the large FR-331 document are outside this bounded delta review.

Read the installed quoin spec-review checklist, fetched the SpecReview authoring pack once with `quoin write . --types SpecReview --json`, and read both its skeleton and input schema. The selected review set is the user's base selection. No seven-lens analysis or subagent was used.

The installed catalog resolves AssuranceProfile to the engineering-assurance module. Repository Markdown/YAML inspection found no AssuranceProfile document or review_policy applying to this scope. There is consequently no applicable profile requiring an enforced spec-review run; no human gate or accepted enforced run is invented here.

Read the current source authority in the hidden-choice worktree: `fsl-replay/src/protocol_analysis/strategy.rs`, `artifact.rs`, `realizability_replay.rs`, `protocol_analysis.rs`, `result/wire.rs`, `spine/native_result/protocol.rs`, and the existing protocol canonical writer/identity definitions. Compared the parallel FSL FR-286 contract at `005cfb391230d39d175dc173096e473f25cec2c9`; its body spelling and association requirements agree with FR-331.

## Base checklist assessment

The delta extends the existing FR-331 rather than allocating another FR or US. AC-61 through AC-63 are distinct, sequential within the existing table, and use canonical parent-prefixed IDs. Their Verification cells name Test without adding TC identifiers. No new US, TC, option or constraint ID is introduced. The explicit local dependency links for FR-352, FR-426 and FR-433 resolve to existing files. Quire reports no new structural or link validation error.

Inputs, outputs and adverse behavior are concrete: exactly three body kinds; two projection decisions; closed new objects; complete key and witness preservation; canonical unsigned decimal strings for new indices and instance numbers; existing exact JSON u64 key members unchanged; and request/backend/index association checks. Unknown, missing, duplicate, wrong-null and noncanonical inputs refuse. Configured frame and replay limits apply, and exhausted validation cannot establish a decisive result. No new performance target or security subsystem is needed for this bounded body contract.

The typed-field comparison confirms environment-defeat has top-level obligation and separating_witness and each branch contains only play. Projection preserves Choice's two cases, paths and nullable actions, and ActingInstance's event, environment owner, two instance ordinals and maximum. No projected graph or realizability strategy is substituted. Strategy sends, receives and ends preserve the actual native action fields.

Plays preserve initial state, successor order and complete canonical transition identity. A send carries its separate canonical send record; a non-send omits that member and cannot carry null. The legacy open TransitionIdentity subformat is unchanged; native canonical validation remains responsible for its step-specific members. Send records match the existing scalar and interval writer forms. The witness matches ProtocolWitnessWire's three fields, object assessed root, empty path steps and decimal position; quantifier/index are excluded. FR-352 remains unchanged and excludes the body.

The hash/digest/pin checklist was applied to the scoped delta. Its structural-key preservation and explicit rejection of digest substitution add no hash, manifest, inventory, toolchain pin, provenance or second file-tracking requirement. Existing native analyzer version fields identify the analyzer as part of the preexisting key rather than prescribing a toolchain version.

## Criterion coverage assessment

Coverage is assessed here as contract verifiability before implementation. No matrix was run, and executable trace-tag coverage for new AC-61 through AC-63 is not claimed complete.

1. Coverage: AC-61 covers typed writer/reader preservation; AC-62 covers structural envelope vectors; AC-63 covers native association, replay and semantic refusals. Implementation evidence remains required.
2. Option permutation: all three body alternatives, both projection decisions, action alternatives, nullable branch ends and send versus non-send shapes are stated. The four positive vectors cover the four main body/decision combinations.
3. Constraint boundary: the contract defines empty plays, nonempty branch defeats, exact two-element projection pairs, distinct cases/instances, zero-based u64 indices, range and horizon checks, live-instance bounds and resource exhaustion. Adverse vectors exercise overflow, negative/numeric index spellings, empty branches, short pairs and duplicates; runtime boundaries belong to native qualification.
4. Error path: AC-62 defines structural refusals and AC-63 defines context/semantic refusals. Structural schema admission is explicitly insufficient for refutation truth.
5. State transition: AC-61 preserves ordered plays and canonical transitions/send records; AC-63 requires replay equality to the selected successor and original key. This contract introduces no new protocol transition kind.
6. Edge case: stated-empty versus absent premises, explicit key nulls, branch-end nulls, zero-position empty-prefix convention, wrong originating item/backend, branch ordering/completeness, projection nondivergence and ambiguous acting instances are explicitly covered.

## Validation and limits

Independent existing Python Draft202012 static validation exited 0: schema valid, four protocol-analysis positives admitted and 54 adverse vectors refused. This establishes the authored structural fixtures only. No helper was added and no Python semantic implementation was introduced.

`quire validate --scope /home/peter/dev/worktrees/protocol-batch-2-qsl-2501-counterexample-contract 'spec/**/*.md'` exited 0. Fresh stdout and stderr are retained beside this review. The whole-tree output includes unrelated advisory grammar and semantic warnings; it is not a clean-warning claim. Targeted FR-331 validation exited 0 with the existing unresolved ProtocolClaimKind warning and two verifies-edge warnings. Validating the base FR-331 text via stdin under the same explicit repository scope reproduced those same three warnings.

The standalone SpecReview schema query exited 0 while emitting existing UnknownEdgeType and UnknownRole module-composition warnings on stderr. No fallback module or binary was substituted. Actual executables are `/home/peter/.npm-global/bin/quire` (0.52.0, engine 0.62.2) and `/home/peter/.npm-global/bin/quoin` (0.36.8), newer than the environment's historical release notice.

The author's earlier prohibited Cargo invocation is recorded in `/tmp/protocol-ce-fspec-cargo-positive.log`: one positive-vector Rust qualification test passed. That preexisting record is disclosed, not treated as authorized qualification or runtime acceptance. This review ran no Cargo command, runtime acceptance, full implementation gate or matrix. Official Rust qualification remains with the root test runner on draco.

The review leaves source and repository artifacts unchanged. It authorizes no tracker publication, new issue, PR mutation or merge, and does not waive the separate FSL review or subsequent implementation checks.
