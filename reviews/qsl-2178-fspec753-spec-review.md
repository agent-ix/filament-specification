---
id: SR-2026111919143300000001
title: "spec-review of QSL-2178 FR-829 clock-place reference term and FR-322 amendments"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #753 at c5e9de0c; spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md, spec/test-cases/TC-2084-clock-place-reference-term.md, spec/objects/interfaces/FR-322-checked-package-artifact.md (Reference<Clock> type row, family list, object-term placement rule, any_value group); context: FR-322 Presence, Data part of a clock-bearing record tuple or union, FR-322-AC-155/156/475, FR-772, FR-415, FR-272, FR-154 Map types, proposals/checked-package-v2 fixtures, QVC checked-operation-catalog-v1.json"
review_set: subset
---

## Summary

Spec review of FSpec PR #753. Ticket: QSL-2178. The PR adds FR-829, which carries `ref(p)` as the `filament.op.clock_reference.ref` application typed by one `scalar_type`/`clock_reference` node. It also adds TC-2084 and amends three places in FR-322: the `Reference<Clock>` type row, the scalar family list and `any_value` group, and the rule for where an `object` term may stand. The review checked the PR against the QSL-2178 ruling comment, against FR-322 "Presence", "Data part of a clock-bearing record, tuple or union" (d(p), FR-322-AC-475), FR-772 and FR-415, against the published presence vectors in `proposals/checked-package-v2`, and against the QVC operation catalog at filament-verification-contracts b79af1b. It also checked the FR-272 cause catalog and the open FSpec PRs (#695, #640) for id collisions.

## Examined scope

- Owner terms against the published presence vectors (examined). In every `proposals/checked-package-v2` fixture, `self` and `other` are `value`/`parameter` nodes whose `semantic_type` is `composite_type`/`reference` over the object type. Examples are `positive-timed-slot-nodes.json`, `positive-timed-clause-nodes.json` and `timed-slot-selector-vectors.json`, and the twelve `self`/`other` parameters across the fixtures were checked. The published `presence_read` (PR-01: `present(other.v)`) uses that reference parameter directly as operand 1. No fixture has a `self`/`other` typed by the object node. See FND-001.
- `Reference<Clock>` type row, family list and `any_value` group (examined). Adding `clock_reference` to `any_value` admits it in `filament.op.control.if`/`let`, `state.pre`, collection construction, `contains` and `remove_one`, which is correct. Collection `contains`/`remove_one` take equality from the family's `eq`, which is byte equality of the place id. No catalog group gives ordering or arithmetic over `any_value`. `structural_kind` does not list it. FR-154 "Map types" still refuses it as a map key, because the scalar key types are an enumerated list (PM-17 `ClockRef` key is unchanged). There is no literal `value_kind` for it, so no canonical literal ordering is needed. Hashing in the state key is the existing FR-415 place-id encoding.
- Data member and d(p) (examined). Before the PR, the `Clock` row gave `Reference<Clock>` no node, which made it a clock member under the "Data part" definition. With its own node it is a data member, as the ruling requires. The tuple clause of AC-5 (`(Clock, Reference<Clock>)` carries one term) checks d(p) correctly against FR-322-AC-475.
- Map, selection and population soundness (examined). `place_has_id` and the reader refusal `type-mismatch` at `/body/arguments/0` match FR-415 "Element places" and the ruling. The slot selection obligation stays on the selector. The binder visits only held objects. `ref` carries no presence obligation, and `deref` keeps it.
- Reader checks and catalog entries (examined). These follow FR-772's precedent for a leading `clock_place` argument checked before the catalog. Argument count `2 + s` agrees with FR-322-AC-156.
- Structural identity (examined). The path `["expression","place_reference",A1..An,"filament.op.clock_reference.ref","place_ref",R]` has the shape of FR-772-AC-1's slot-domain path.
- Refusal codes (examined). `ill_typed`/`type-mismatch` and `ill_typed`/`operator-ineligible` are in the FR-272 catalog (line 75).
- Ids (examined). FR-829 and TC-2084 are free: main ends at FR-828 and TC-2083, and open PRs #695 and #640 use neither.
- Each AC can fail, and no `unsupported`/`not-yet-implemented` text is left (examined).

## Verdict

Not mergeable. FND-001 is high. The new "Owner terms" rule rests on a claim the published vectors contradict. Applied as written, it refuses the published presence and slot-domain vectors and contradicts FR-829-AC-1 against FR-829-AC-6. FND-002 is medium: one refusal pointer is undefined. FND-003 and FND-004 are low wording issues. The type node, family and group amendments, the data-member and d(p) treatment, the Map and selection rules, the catalog entries and the ids are correct.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | "Owner terms" says that `self`/`other` are already `object` terms "typed by its object type", so "the published presence vectors keep their encoding". It also says that a `reference` term placed directly at the owner position refuses `ill_typed`/`operator-ineligible`, for `presence_read` and the slot domain too. Every published fixture types `self`/`other` as `composite_type`/`reference` parameters. PR-01's `present(other.v)` and FR-772-AC-1's slot domain at `self` pass that reference parameter directly, so the rule refuses them (FR-322-AC-155, FR-772-AC-1). Inside FR-829, AC-1 admits `ref(self.x)` with `self` direct, while AC-6 refuses the binder `c` direct, and both have the same typing. Pick one encoding: either admit a `Reference<O>` term at the owner position (no `deref`, so binders and `other` match the vectors), or `deref` every owner, including `self`/`other`, and republish the vectors and FR-322-AC-155 and FR-772-AC-1. Amend FR-322 "Presence" to match | spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md:130 |
| FND-002 | medium | FR-829-AC-6 expects a `filament.op.collection.sum.integer` "whose summand is a place reference" to refuse `operator-ineligible` "at the offending argument". The catalog entry's operands are `collection_kind` and `binder`, so it has no summand argument, and the pointer is undefined. Name the pointer, for example the binder's body or `/body/arguments/1` under `member_family` | spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md:211 |
| FND-003 | low | AC-1 types `q` as `Sequence<Clock>[0, 2]` and attributes it to FR-415-AC-15's model, which has no `q`. AC-3 and AC-6 type `q` as `Sequence<Option<Clock>>[0, 2]`. TC-2084 has no single fixture, so the two `q`s conflict. Give AC-1 its own field name, or use one type throughout | spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md:206 |
| FND-004 | low | AC-5 gives the record `Tag`'s fields `r` and `n` "data index 0 and 1". FR-322 defines data index d(p) only for tuple positions, and a record's data part is read by field name in declaration order. Say "data part `r`, `n` in that order" | spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md:210 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The new `place_owner` group (`reference`, `object`) admits an `object` term at the owner position. FR-322's object-placement paragraph still says "`object` is a member of no other group, so the `inner:0` result of `filament.op.model.deref` is read only through a field projection", which is now false, so a reader admits `deref(c)` at an owner position. That gives one source owner a second encoding and a second node identity, which FR-829 "Owner terms" ("one encoding") forbids only for producers. No published vector has an object-node-typed owner. Either drop `object` from `place_owner`, or state a reader refusal for a `deref` application at an owner position, and correct the paragraph (object is a member of `field_owner` and `place_owner`) | spec/objects/interfaces/FR-322-checked-package-artifact.md:3478 |
| FND-006 | low | FR-829 states only the catalog entries. The QVC `checked-operation-catalog-v1.json` also has a `families` array and a `groups` map, and FR-829 and FR-772 need four matching changes there that the PR does not state or track: `clock_reference` added to `families` and to `groups.any_value`, a new `groups.place_owner`, and the `option.slots` operand changed from `object` to `place_owner`. QVC main 2714219 still has `"operands": ["object"]` for `option.slots`. `proposals/checked-package-v2/schema.json` names no operand group, so it needs no change. Name the QVC families and groups change, or file the QVC follow-up | spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md:170 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 0ebbc5a4: "Owner terms" now admits the `Reference<O>` term directly with no `deref` (group `place_owner`). FR-322 "Presence" and FR-772's entry take the same rule, the published `self`/`other` reference parameters are admitted unchanged, and AC-1 now agrees with AC-6 (binder `c` admitted) |
| FND-002 | fixed | AC-6 names `/semantic_graph/nodes/{n}/body/arguments/1`, the `binder` operand in the catalog order (`collection_kind`, `binder`) |
| FND-003 | fixed | AC-1 now uses `q: Sequence<Option<Clock>>[0, 2]` added to FR-415-AC-15's model, which is the `q` of AC-3 and AC-6 |
| FND-004 | fixed | AC-5 and the prose now say the record data part is `r`, `n` in order, and keep d(p) for tuple positions |
