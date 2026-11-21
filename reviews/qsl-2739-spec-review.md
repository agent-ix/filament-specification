---
id: SR-2026112106460400000005
title: "Spec review: PR 785 FR-290 component_composition node-to-family row"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/protocol/FR-290-protocol-claim-kind.md (Reading the family from a request item, node-to-family table row at line 254) at 956c3ddf against f73bd9f3"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-290
    type: references
---

## Summary

Ticket: QSL-2739. PR agent-ix/filament-specification#785 adds one row to the FR-290 node-to-family table: a `correspondence`/`component_composition` claim node whose rule is `protocol-refinement`, `refinement-transfer` or `interface` (FR-491) has declaration family `protocol`. The row leaves out `assume-guarantee`, which stays planner-local.

What I checked:

- **Node tag and form.** FR-491 "Checked-package node" emits `S` with `node_tag: "correspondence"`, `semantic_form: "component_composition"` and a required body member `rule`, holding one of the four rule spellings. FR-322's Body grammar admits a `component_composition` body. The row's `tag`/`semantic_form` and its "whose rule is" condition match.
- **Rule spellings.** The three spellings are exactly FR-491's non-`assume-guarantee` rules, as listed in its rule table and its "Derived item and slots" table.
- **Agreement with the claim-form table.** The claim-form row at line 155 assigns `composition`/`protocol`, and the `protocol` family's "Kinds that apply" already lists `composition`. So the new row puts the derived FR-331 composition item of each of those three rules into a family whose kinds include `composition`, and the item no longer refuses `inapplicable-kind`. The `assume-guarantee` item is planner-local (FR-491 table, claim-form row at line 154), so it is not a request item and needs no family.
- **Position.** The row sits before `any protocol node`. A `correspondence` node is not a `protocol` node, so the order does not change which row matches.
- **Validation.** Scoped `quire validate` on FR-290 passes, and `git diff --check f73bd9f3..956c3ddf` is clean.

One case is not covered: the `fixed-point` slot item.

## Verdict

**CONDITIONAL**: the row is correct for the main derived item of the three non-`assume-guarantee` rules. It does not give a family to the FR-176 fixed-point item of an `assume-guarantee` composition, so that item contradicts the claim-form table (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The rule condition leaves the FR-176 fixed-point item of an `assume-guarantee` composition with no family. FR-491 derives "one `fixed-point` premise slot" for every admitted cycle, whatever the rule. That slot is item-backed: "one FR-331 `composition` item anchored at `S`'s node" (FR-491:150), filled by "the FR-331 `composition` result of FR-176's item" (FR-489:116-119). Under `assume-guarantee`, `S`'s node has rule `assume-guarantee`, so it matches no row and has no family (line 257). Every kind is then inapplicable, and the item settles `inapplicable-kind`. But the claim-form row at line 155 assigns "FR-176's fixed point over an accepted cycle" the kind `composition` and the family `protocol` for any rule, and line 276 says `negotiate_*` SHALL NOT refuse such an item. FR-176's fixed point is the assume-guarantee discharge rule, so `assume-guarantee` is the main case for an accepted cycle. Suggested fix: extend the row (or add a second one) so that the `fixed-point` composition item anchored at a `component_composition` node of any rule also takes `protocol`. Or, if FR-491 meant the fixed-point item's claim to name another node, say which node in FR-491. | spec/objects/protocol/FR-290-protocol-claim-kind.md:254, 155, 257, 276; spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md:113-120, 150-154; spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:108-121 |

## Scope

- FR-290 node-to-family row `correspondence`/`component_composition` (line 254) (examined; FND-001)
- FR-290 claim-form rows for `assume-guarantee` and the non-AG composition rules (lines 154-155) (examined, clean)
- FR-290 `protocol` declaration-family row, its kinds that apply (examined, clean)
- FR-290 "A claim that names a node of no row" and the `SHALL NOT refuse inapplicable-kind` rule (context_only)
- FR-491 rule table, "Derived item and slots", "Checked-package node" (context_only)
- FR-489 "Cycles" (context_only)
- FR-322 Body grammar `component_composition` (context_only)

## Dispositions

Round 1, against cbe2b388 (delta 956c3ddf..cbe2b388, one added row in FR-290 at line 255).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The new row gives the `protocol` family to "the `fixed-point` `composition` item an accepted cycle derives", anchored at a `correspondence`/`component_composition` node of any rule, including `assume-guarantee`. That is exactly the FR-491:150 item and FR-489 "Cycles". It now agrees with claim-form row 155 ("FR-176's fixed point over an accepted cycle", `composition`/`protocol`) and with the SHALL NOT refuse rule at line 277. `composition` is in the `protocol` family's kinds. The qualifier names an item, not only a node, but a reader can check it. FR-331 says `premises` "is carried only by a `composition` item a composition declaration derives". Under `assume-guarantee` the main item is planner-local, never a request item, so the fixed-point item is the only request item that can name an AG node. The reader checks that against the package as a producer obligation, as it already does for derived items (FR-331-AC-44). `assume-guarantee` semantics are unchanged: row 154 still says "none; planner-local, not a request item", the row says so again, and the "node of no row" rule, FR-491, FR-489 and FR-492 are untouched. The first row (non-AG rules) and the new row both give `protocol`, so they never conflict. Scoped `quire validate` on FR-290 passes and `git diff --check f73bd9f3..cbe2b388` is clean. |
