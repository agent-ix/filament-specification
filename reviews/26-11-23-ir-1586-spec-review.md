---
id: SR-2026112305352200000001
title: "spec-review of FR-322 finite-hyper temporal_profile member refusals (TCB-17..24)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-2028-a-finite-domain-hyper-clause-carries-one-row-per-trace.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/temporal-clock-binding-vectors.json; IR-1586; PR 815"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
---

## Summary

Ticket: IR-1586. PR agent-ix/filament-specification#815 at 746855ab. Scope is `git diff origin/main...HEAD`: the FR-322 finite-domain paragraph, FR-322-AC-464, the TC-2028 procedure, the HyperpropertyBody schema description, and finite_hyper_profile_cases TCB-17..TCB-24. The codes are compared with the reader in agent-ix/filament-contract-ir PR 1136 at e7b81bbb (`finite_profile` and the behaviours check in temporal_binding.rs, `validate_body` in v2/mod.rs, and its tests). Sub-analyses covered: integrity (cross-artifact consistency), executability of the vectors, and AC/TC traceability. `quire validate` passes on both changed Markdown files; its two EARS warnings at FR-322 line 116 predate this diff. Both JSON files parse.

Clean: TCB-17 (missing member) and TCB-18 (two matching selections) give `operation-law-unselected` at the member, as the reader does and as its test `a_finite_hyper_body_profile_must_be_present_and_selected_exactly_once` asserts. TCB-19 (member on a behaviours hyperproperty) gives `invalid_package`/`invalid-value` at the member, as the reader does. TCB-17/18 are distinct from TCB-15 (zero match with member present). The FR-322 prose, AC-464 and TC-2028 step 4 name the same cases, and TC-2028's scope stays FR-322-AC-464.

## Verdict

**NOT MERGEABLE.** Two refusal codes disagree with both the reader and the schema (FND-001, FND-002). The new cases are not precise enough to run (FND-003..FND-005). Merging alone turns the reader's conformance test red (FND-006).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TCB-21..24 (non-object, extra field, empty `authority`, empty `identity`) are published as `invalid_package`/`invalid-value` at the member. The schema refuses all four: `HyperpropertyBody.temporal_profile` is `$ref DefinitionRef`, which is closed with `Nonempty` members. FR-322's closed-schema rule makes a schema refusal `malformed_wire` (AC-461 precedent), and README line 617 says "the schema gives the field its exact shape". The reader in PR 1136 refuses them as `operation-law-unselected` instead. `finite_profile` filters the member with `exact_members(["authority","identity"])`, so a non-object or an extra field gives `None`, which then fails the match. `definition_ref` accepts empty strings, which then match zero lock rows. No reader test covers TCB-21..24. Three artifacts give three answers, and the schema description added in this PR ("a non-exact DefinitionRef is invalid-value") contradicts the schema's own structure. | FR-322-checked-package-artifact.md:4927-4930; FR-322-AC-464 (line 6849); schema.json:6652; temporal-clock-binding-vectors.json:243-290 |
| FND-002 | high | TCB-20 (member on a non-hyperproperty node) is published as `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{N}/body/temporal_profile`. In PR 1136, `validate_body` returns `InvalidSemanticGraph` with no cause at `at.pointer()`, the body. Its test `a_behaviours_hyper_body_carries_no_profile_member_and_other_nodes_may_not` asserts `(InvalidSemanticGraph, None, /semantic_graph/nodes/{clause}/body)`. Under FR-322-AC-462's precedent, a body member on a node that may not carry it is `malformed_wire`. The code, cause and locus all differ from the reader. | FR-322-checked-package-artifact.md:4925-4927; FR-322-AC-464 (line 6849); temporal-clock-binding-vectors.json:231-241; schema.json:6652 |
| FND-003 | medium | TCB-17 conflicts with the `mutation_protocol` this PR leaves unchanged. The protocol says "For each case ... set its body.temporal_profile to D". TCB-17 leaves out the set op and relies on the emitter base omitting the member (generation step 4). No remove op is defined. FR-322 says the body SHALL carry the member, so once the emitter emits it, TCB-17 runs as an admitted package that never exercises the missing-member path. Fix: add an explicit member-removal op, and change the protocol to "apply exactly the case's mutations". | temporal-clock-binding-vectors.json:156,197-206 |
| FND-004 | medium | The fixtures and ops of TCB-19 and TCB-20 are not defined well enough to run. `behaviours_hyper_profile_fixture` appears only in the note, as "any checked-package positive whose hyperproperty carries a `behaviours` domain". It has no fixture object, source or generation steps, and the T/D resolution and lock replacement in `mutation_protocol` are defined only for `finite_hyper_profile_fixture`. TCB-20 uses `set-body-member`, the selector `unique-temporal-clause-node` and the placeholder `{temporal-clause-index}`, and none of them is defined. Its `[T, D]` replacement also drops the `protocol_profile` selection that `fixtures/positive-timed-clause-nodes.json` carries, which risks an earlier, unrelated refusal. | temporal-clock-binding-vectors.json:2 (note),219-241 |
| FND-005 | medium | The `value` encoding of TCB-21..24 is undefined. In other cases, `"D"` is a key into `profile_definitions`. TCB-21 passes a raw identity string, and TCB-22..24 pass objects with `"D.authority"`/`"D.identity"` placeholder strings, which neither the protocol nor the note defines. A test cannot tell whether a string is a key or a literal, or whether `D.authority` should be replaced. | temporal-clock-binding-vectors.json:243-290 |
| FND-006 | medium | PR 1136's conformance test `finite_hyper_profile_vectors_publish_the_body_profile_join` reads the filament-specification checkout and asserts that `finite_hyper_profile_cases` is exactly TCB-14..TCB-16. Merging this PR alone makes that test fail. The two PRs must land together, with the reader test and code changed to match whichever codes are settled under FND-001/FND-002. | temporal-clock-binding-vectors.json:162-291; filament-contract-ir tests/conformance_fspec/temporal_clock_bindings.rs:+939-989 |
| FND-007 | low | README line 617 still lists `finite_hyper_profile_cases` as TCB-14 to TCB-16 and does not mention TCB-17..24. | proposals/checked-package-v2/README.md:617 |

## Scope

- FR-322 "Finite domain" paragraph, lines 4909-4935 (examined; FND-001, FND-002)
- FR-322 vector summary, line 5019 (examined, clean)
- FR-322-AC-464 (examined; FND-001, FND-002)
- TC-2028 Test Procedure step 4 and Expected Results step 4 (examined, clean for traceability)
- schema.json HyperpropertyBody and DefinitionRef (examined; FND-001)
- temporal-clock-binding-vectors.json note, `finite_hyper_profile_fixture`, `mutation_protocol` and TCB-14..TCB-24 (examined; FND-003..FND-006)
- FR-322-AC-461, FR-322-AC-462 (context_only; closed-schema and misplaced-member precedent)
- filament-contract-ir PR 1136 at e7b81bbb, its src and tests (context_only)
