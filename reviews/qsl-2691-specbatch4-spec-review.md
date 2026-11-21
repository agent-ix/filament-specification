---
id: SR-2026112104184601
title: "spec-review of completion spec batch 4 (filament-specification PR 778, filament-spec-language PR 3252)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification PR 778 head 98731286078a7bb36f35bec4087e0d249266f7c1 (FR-154, FR-194, FR-322, FR-402, FR-778, FR-802, FR-829, schema.json, shared-grammar.md, checked-term-data-vectors.json, integer-containment-vectors.json, TC-1602, TC-1890, TC-2084, TC-2090, TC-2099, TC-346); agent-ix/filament-spec-language PR 3252 head 25bf334be0d96911deceb4e156ab2acd3a8b68eb (FR-093, FR-1040, FR-187, FR-194, FR-488, FR-495, FR-498, FR-501, FR-531, FR-621, FR-660, FR-895, TC-1288, TC-1327, TC-1741, TC-2064, TC-2530, TC-2873, TC-3000, TC-3084, TC-3133, TC-3136, TC-3137, TC-629)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-402
    type: references
  - target: ix://agent-ix/filament-specification/FR-154
    type: references
---

## Summary

Ticket: QSL-2691 (completion spec batch 4, both PRs). One proportionate pass of quoin:spec-review over `git diff origin/main...<PR head>` in both repositories, reviewer-only, no builds. Untouched origin/main text was read where the new text could conflict with it.

Clean: A (FR-660-AC-7 code, cause, category and exit agree with FR-111's refusal table); E (FR-498 AC-3/AC-8 agree with FSpec FR-421 "Admitted forms", and FR-234/FR-235 keep `PunctualInterval` for timed temporal clauses only); K (FR-187-AC-6 order and record); L (FR-495-AC-26 values at `n = 4`); G grammar (`call-argument`, `lambda`, longest-match `=>`, ruling R stated in FSpec FR-194 and FR-802); the FR-194 window rule is sound: a window is definite only when completeness is asserted through `t_i - a`, which covers every stamp a continuation could add to it. The IEEE `let` order the D example assumes is specified (FSL FR-397 "Operand binders": the left operand's `let` outermost). `schema.json` matches the prose for `BodyLocation`, `ClaimNameRow.source_unit` and the binding `binder`. No new AC or TC id collides with origin/main or with an open PR (FSL #3098 and FSpec #640 touch the same files with other ids). No scope cut was found. FR-230-AC-68 is untouched. FR-056-AC-72 still reads "At the validator, reported ...", so the required observable-behaviour rewrite is absent and is listed as FND-017 for the fix round.

Two wire rules are unsound or contradicted: the reader cannot pair `body_locations` rows with walk visits, and BL-01/BL-03's `let`s lack the `binder` the PR makes mandatory. FSL FR-264 still requires the quadratic whole-path rows that FR-531-AC-41 replaces.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | D: a row's path is rebuilt from "the term the walk visits just before it", but no rule pairs a walk visit with a row. Rows are in key order, not walk order (BL-01's second `sqrt` is array row 8 and walk term 2). For BL-03's `+` node, visited twice, the opposite pairing of occurrences 0 and 1 to visits gives `{up: 1}` from the empty path and refuses the vector the AC admits. Fix: state that the k-th visit of `F`'s walk at `A` pairs with `A`'s row of rank k in ascending (`occurrence`, `term_ordinal`) order, and that the rebuild runs in walk order. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1462-1483, 1511-1513 |
| FND-002 | high | I: BL-01's and BL-03's packages (`f`, `h`) hold four `#ieee_operand` `let`s whose binding terms carry no `binder`, and the binder node is not in their `dependencies`. "The binder link" first rule refuses them at `arguments/0`, but FR-322-AC-490 and AC-510 require both to be admitted. Fix: add `binder` and the dependency to the four `let` nodes and regenerate the affected node ids and rows. | proposals/checked-package-v2/checked-term-data-vectors.json:5173-5190, 5413-5430; spec/objects/interfaces/FR-322-checked-package-artifact.md:3490-3493 |
| FND-003 | high | D: FSL FR-264's behaviour still says each `body_locations` row carries its whole path and grows with the square of depth. FR-264-AC-8 requires a 100,000-term sum to refuse `OutputLimit` at `1 << 27`, while FR-531-AC-41 requires it to emit within the 16 MiB default. Fix: rewrite FR-264 behaviour 5 to the relative form and replace AC-8 with the linear-size expectation. | spec/functional/FR-264-emit-and-read-v2-packages-at-schema-fixed-depth.md:65-71, 84 |
| FND-004 | medium | D: the least-`up` canonical form is a producer rule only. BL-02 writes `{up: 1, down: [0]}` after path `[0]`, whose least form is `{up: 0, down: []}`, and is admitted, contrary to "one sequence of paths has one encoding". Fix: add a reader refusal at `location/up` for a non-least `up` and write BL-02 canonically, or state that the reader admits non-least encodings. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1472-1474, 6521 |
| FND-005 | medium | I: the vector file's top-level note still says "IC-15 is admitted and not enforced: see its pending member", which contradicts AC-501, AC-512 and TC-2099. Fix: delete the sentence. | proposals/checked-package-v2/integer-containment-vectors.json:4 |
| FND-006 | medium | F: check 5 orders the scope by `name`, but nothing refuses two slot nodes of one name (different level or type) in one wrapper's scope. Their positions tie, so the level check has no defined answer. Fix: refuse a second scope slot node with an already-seen name at its `level` binding. | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:166, 321-328 |
| FND-007 | medium | F: FR-778-AC-13 and FR-322-AC-511 require the missing hyper `source_unit` refusal "before any unit resolution", but FR-322 places the check only "in the `claim_names` row visit". It does not say that visit precedes resolving the hyper clause's `temporal_bindings` `timestamp_unit`. Fix: state that a hyper clause's rows resolve their units after its `claim_names` row passes. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4679-4681 |
| FND-008 | medium | B: FR-154-AC-53 cites TC-234, but TC-234 is not changed and has no row or step for it. Fix: add a TC-234 input row covering AC-53's cases. | spec/functional/type-model/FR-154-admit-domain-package-model.md:988 |
| FND-009 | medium | G: FR-194-AC-12 refuses `Reach.steps.sum(Reach => 1)` `ambiguous-name`, but the rule refuses a parameter that "shadows a binder in scope". `Reach` is the declaration's name, not a binder. Fix: say "a binder or declaration name in scope", or make the example shadow a real binder. | spec/functional/analysis/FR-194-synthesize-bounded-candidates.md:107-108, 153 |
| FND-010 | medium | C: the new rule says a two-valued aggregate atom makes "the activation that reads it" `Pending` or `Indeterminate`. FR-501's lifting gives a definite verdict when the atom is outside the decision support (for example `A or true`), so two implementations would disagree. Fix: say only that the atom is never decisive, and that FR-501's lifting decides the activation. | spec/functional/FR-194-evaluate-window-aggregates-in-monitors.md:192-196 |
| FND-011 | medium | F: a slot's occurrence site `hyper <c> slot <k>` used `k` from `atom_slots()` (quantifier order). With scope-ordered levels and unread slots minting no node, `k` is now undefined, and `Fwd` and `Rev` could record different sites for one node. Fix: define `k` as the slot's scope position. | spec/functional/FR-488-emit-a-checked-hyper-clause-as-a-v2-hyperproperty-node.md:268 |
| FND-012 | medium | B: FSL says a reference type whose target is an alias `A` reads as `Reference<A>`. FSpec FR-154 reads it as `Reference<Clock>` ("the same holds with `target` an alias", AC-53). Fix: drop the parenthetical or align FSpec. | spec/functional/FR-1040-check-and-lower-clock-place-references-and-presence-in-population-forms.md:71-72 |
| FND-013 | medium | A: "before it reads any definition document" conflicts with the earlier rules that read every entry's `file` as a dependent file and build each `Definition` from its bytes. AC-7 means "validates no grammar schema". Fix: write "before it validates any definition document (FSpec FR-467)". | spec/functional/FR-660-supply-extension-definitions-through-the-request-extensions-member.md:82-87, 101-103 |
| FND-014 | medium | H: notes sort "by the `details` members", but those members are ASCII decimal strings, so lexicographic and numeric order differ (`"10"` before `"9"`). Fix: state numeric order for the counts and byte order for `at`. | spec/functional/FR-621-write-the-analyze-result-as-one-outcome-document-per-request.md:132 |
| FND-015 | medium | J (R): the inactive-clause rule (`satisfied` on a closed recording) sits after "An obligation is never `satisfied` by the absence of a violation" and does not reconcile the two. Fix: say that for an inactive clause the closed closure record is the deciding evidence, since no further activation can occur. | spec/functional/FR-895-assess-a-recorded-protocol-execution-against-its-obligations.md:99-107 |
| FND-016 | low | Change-log phrasing: FR-621 "Before this member the analyze document had no place for them" and "byte-identical to one written before this member"; FSpec FR-322-AC-501 "(the binder link ... makes the check exact: IC-15 is enforced)". Fix: describe the behaviour only. | spec/functional/FR-621-write-the-analyze-result-as-one-outcome-document-per-request.md:138-140, 155-156; spec/objects/interfaces/FR-322-checked-package-artifact.md:6533 |
| FND-017 | medium | Required addition absent: FR-056-AC-72 still reads "At the validator, reported `invalid_model_binding`/`malformed-declaration` with the validator's diagnostic". It is not written as observable behaviour (intake refuses `DECIMAL_POLICY_CONFLICT` for a policy on a target-bound value type). Fix: rewrite it that way in the fix round, and leave FR-230-AC-68 unchanged. | spec/functional/FR-056-admit-domain-package-model-declarations.md:1327 |

## Recommendations

Fix FND-001 to FND-003 before merge. All three are wire contradictions that a strict reader or the FSL emitter would hit at once. The medium items are one-sentence edits. FSL PR 3252 depends on FSpec PR 778's wire for D, F and I, so both PRs should merge after one fix round.

## Verdict

Not mergeable as is: FSpec PR 778 has FND-001, FND-002 and FND-004 to FND-009 and FND-016. FSL PR 3252 has FND-003, FND-010 to FND-015 and FND-017. The rulings F, G, I, J and K are stated correctly and consistently across both PRs. Items E, K and L are clean.
