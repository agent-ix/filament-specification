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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The vectors file's top-level `note` was not updated. It still says TCB-20 and TCB-21..24 "refuse `invalid-value` at the member", that `behaviours_hyper_profile_fixture` "is any checked-package positive whose hyperproperty carries a `behaviours` domain", and that "the malformed-member cases are reader-level refusals ... not by the schema-validation test". The case rows, the new fixture objects, FR-322, AC-464, TC-2028, schema.json and the README now say `malformed_wire` and give the fixture an exact construction. The note contradicts the cases in the same file. | temporal-clock-binding-vectors.json:2 |
| FND-009 | high | `behaviours_hyper_profile_fixture` (TCB-19) is the finite fixture with only `domain` replaced by the text literal `behaviours`. By FR-402 that is not a well-formed `behaviours`-domain hyperproperty. Its quantifier Groups have no `alias` binding, which FR-402 hyperproperty-step check 1 (Shape) refuses as `invalid_semantic_graph` at the Group. `subjects` is empty, so no Group alias resolves. `body` references a Boolean `expression`, not a body wrapper, which check 3.1 refuses as `ill_typed`/`operator-ineligible`. `dependencies` still holds the finite-domain node, which the body no longer references. The body steps run before the temporal-bindings joins, so a conforming reader refuses with one of those codes, not `invalid_package`/`invalid-value` at the member. The fixture note "its reader refusal is raised at the member before any row join" orders the check only against row joins, not against the FR-402 body step. Derive TCB-19 from a real behaviours-domain positive (for example FR-402-AC-11's `Whole` or the HB-01 package), or state in FR-322 that the member check runs before the hyperproperty body step. | temporal-clock-binding-vectors.json:166-169 (fixture object),298-322 (TCB-19); FR-402-hyperproperty-obligation-identity.md:33-74,286-322 |

## Dispositions

Round 1, against 848d0209 (fix delta 746855ab..848d0209). I schema-validated with Python `jsonschema` against `proposals/checked-package-v2/schema.json`. TCB-20 on `fixtures/positive-timed-clause-nodes.json` node 8 fails with `additionalProperties` (`temporal_profile`) at `/semantic_graph/nodes/8/body`. HyperpropertyBody with TCB-21..24's members fails at `/temporal_profile` (type), `/temporal_profile` (additionalProperties), `/temporal_profile/authority` (minLength) and `/temporal_profile/identity` (minLength). The lock's `profile_selections` has no `uniqueItems`, so TCB-18 is schema-valid. `quire validate` passes on FR-322 and TC-2028, with only the pre-existing EARS warnings at FR-322 line 116.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-322:4928-4933, AC-464, TC-2028 step 4, the schema description and TCB-21..24 now say `malformed_wire` at the failing schema pointer. Measured schema pointers match. This is consistent with AC-461's closed-schema rule. |
| FND-002 | fixed | TCB-20, FR-322:4925-4928, AC-464 and TC-2028 now say `malformed_wire` at `/semantic_graph/nodes/{N}/body`, citing AC-462. The measured schema error is at `/semantic_graph/nodes/8/body`. |
| FND-003 | fixed | `mutation_protocol` now applies "exactly the case's mutations in the order listed". The new `remove-body-temporal-profile` op is a no-op when the member is absent, and TCB-17 uses it. |
| FND-004 | fixed | `positive_temporal_clause_fixture` and `behaviours_hyper_profile_fixture` are now objects. `set-body-member`, `unique-temporal-clause-node`, `set-hyperproperty-domain-behaviours` and the `{temporal-clause-index}` placeholder are defined. TCB-20 no longer replaces the lock. Whether the behaviours construction is correct is a separate question, raised as FND-009. |
| FND-005 | fixed | The protocol now separates `value` (a profile_definitions key) from `literal` (verbatim JSON). TCB-21..24 use `literal` with concrete strings. |
| FND-006 | deferred | Cross-repo landing order. filament-contract-ir #1136 is being updated to these codes, per the coordinator. #815 must not merge before #1136's conformance test expects TCB-14..24. |
| FND-007 | fixed | README:617 now lists TCB-14 to TCB-24 with the settled codes. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | low | FR-322 states the behaviours-domain member refusal (`invalid_package`/`invalid-value` at the member) but not where it runs. It does not say whether it comes before or after FR-402's hyperproperty body step, the temporal step, or the row-coverage and row checks, and it does not say that it applies when `temporal_bindings` is absent. TCB-19's base is otherwise admitted (FR-402-AC-11), and `Whole` under `inf` has no rows, so TCB-19's result does not depend on the order. A package that also has an FR-402 defect, or a row defect, has no specified first defect. State the step, for example "in the hyperproperty step, before check 1" or "with the temporal-bindings joins, whether or not rows exist". | FR-322-checked-package-artifact.md:4924-4926; FR-402-hyperproperty-obligation-identity.md:276-290 |
| FND-011 | low | `behaviours_hyper_profile_fixture.generation` says "the FR-402-AC-11 emitter test" but gives no repository, source file, test name, command or capture step. `finite_hyper_profile_fixture` gives all of these. The test exists as agent-ix/filament-spec-language `fsl-analyze/tests/it/hyper_object_free.rs`, traced `FSpec-TC-2034`/`FSpec-FR-402-AC-11`, so the base can be found and the case can run, but only after that search. | temporal-clock-binding-vectors.json:166-170 |

## Dispositions (round 2)

Round 2, against 9f2b4530 (fix delta 848d0209..9f2b4530). FND-006 keeps its round-1 `deferred` row as its latest outcome. Reachability of TCB-19: `Whole` is a declared node, so its `node_id` is its declaration path and does not change when the body gains a member (FR-322 "structural key"). schema.json's HyperpropertyBody admits `temporal_profile` on any hyperproperty, so the case is schema-valid. FR-402-AC-11 admits the unmodified `Whole` at the temporal and hyperproperty steps. FR-402's Shape check constrains `members` only. The added member is therefore the package's only defect, and the stated `invalid-value` is its first defect whatever the order of the steps.

| FND | outcome | reason |
| --- | --- | --- |
| FND-008 | fixed | The top-level note now gives TCB-15/17/18 as `operation-law-unselected`, TCB-19 as `invalid-value` on the FR-402-AC-11 fixture, and TCB-20..24 as `malformed_wire` with the stated loci. It agrees with the cases, FR-322, AC-464 and TC-2028. |
| FND-009 | fixed | `behaviours_hyper_profile_fixture` is now the unmodified emitted FR-402-AC-11 `Whole` package (wrapper `over` the model root, Groups with `alias`, `subjects` naming V). TCB-19's only mutation is `set-body-temporal-profile` D, with the lock as emitted. `set-hyperproperty-domain-behaviours` was removed. The base is well-formed, so the member check is reachable (see above). The ordering and fixture-locator gaps are new low findings FND-010 and FND-011. |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | medium | The `behaviours_hyper_profile_fixture` text describes a package that cannot be produced today. On filament-spec-language main, `an_object_free_hyper_wrapper_ranges_over_the_model_root` is `#[ignore = "QSL-2887: the test's hyper source does not parse at S1 (invalid_syntax: expected identifier)"]`. With the published `--include-ignored` command, `World::new`/`emitted` panics before any package exists. The fixture still says "exactly as that criterion emits it", that `emitted(&world)` "returns the complete checked-package bytes", and that "the base is a well-formed behaviours-domain hyperproperty that the reader admits". None of that holds until QSL-2887 is fixed. The vector should say that TCB-19's base depends on QSL-2887 and cannot be produced until then, which also matches the reader's named TCB-19 exception. | temporal-clock-binding-vectors.json:166-175 |

## Dispositions (round 3)

Round 3, against b87c2a87 (fix delta 9f2b4530..b87c2a87). I read the reader's check order at filament-contract-ir#1136 8fc3d38b. Its member shape is a term-stage `malformed_wire`. In `temporal_binding::validate`, the behaviours `invalid-value` comes before the finite lock join `operation-law-unselected` (`finite_profile`, called before any row is read), which comes before the row-coverage and row `clock.kind` checks. Rows may be absent. This matches FR-322:4935-4944, AC-464 and TC-2028 step 4. The reader runs `temporal_binding::validate` (mod.rs:4207) before `validate_hyperproperties`, the FR-402 step (mod.rs:4479). `quire validate` passes on FR-322 and TC-2028, with only the pre-existing EARS warnings at line 116.

| FND | outcome | reason |
| --- | --- | --- |
| FND-010 | still-open | Partly fixed. FR-322 now states the order (1) shape, (2) behaviours, (3) lock join, (4) rows, and that it applies with or without `temporal_bindings`. That matches the reader. It places only (1) before the FR-402 hyperproperty step. For (2) and (3) it says only "before any later check of the same node", which does not say whether the FR-402 step is later. A behaviours hyperproperty with both an FR-402 defect and the member therefore still has no specified first defect. The reader reports `invalid-value`, because it runs the bindings check before `validate_hyperproperties`. Fix: state that (2) and (3) also run before the FR-402 hyperproperty step. |
| FND-011 | fixed | The fixture now names the repository `agent-ix/filament-spec-language`, the source file `fsl-analyze/tests/it/hyper_object_free.rs`, the test `an_object_free_hyper_wrapper_ranges_over_the_model_root`, the command, and the capture step (the `bytes` from `emitted(&world)`). All of these exist on main. That the test is currently ignored is FND-012. |
