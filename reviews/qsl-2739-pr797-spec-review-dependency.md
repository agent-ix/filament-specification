---
id: SR-20261121000797005
title: "spec-review/dependency review of admitted composition metadata"
type: SpecReview
analysis: dependency
scope: "agent-ix/filament-specification; QSL-2739; FR-322, FR-491 and referenced schema/FR-490/FR-492"
review_set: subset
---

## Summary

Ticket: QSL-2739; PR #797. Frozen head 2491154ae6cb9123ecd423d7fa8fb262917ceab8; base bca4311db3a4d9d15c6e4e8f9878ce3a3b9a60b1. Two-file specification diff, read-only review. No Cargo, implementation tests, source edits or runtime admission claim. quoin 0.36.8; quire 0.52.0 (engine 0.62.2); spec-artifacts-process v0.36.0. No applicable AssuranceProfile is declared in the local spec tree; direct review applies. Exact backend model identifier is not exposed in this session; marker uses the advertised Codex identity.

Selected methods: code-review; spec-review/base, integrity, failure-domain, dependency, EARS, scope-boundary and domain-object audit. No Rust or production change means Rust and implementation gap review are not applicable. Evidence/risk-complexity/criterion-strength were not separately selected; no new verification method or risk model is introduced. Four new criteria compute untagged (zero binders) in the installed matrix: expected pending implementation, never counted as executed coverage. Matrix exit 0; unrelated corpus diagnostics were not promoted to this diff's findings. Scoped ISO validation exits 0 with three pre-existing FR-322 warnings (value_object vocabulary and line-116 EARS), no changed-statement warning. Full-repository validation/gates were not run.

## Verdict

PASS — FR-322 metadata admission is enablement; FR-491 composition member attribution and slot source order are feature semantics. The new FR-322 references edge to FR-491 is not a prerequisite cycle: FR-491 defines specialized rules on the FR-322 carrier. Logical order is contract schema/prose -> strict-reader support -> producer metadata -> package consumer projection/execution. FR-490/489/492 remain referenced semantic prerequisites; no source recompilation fallback is added.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

The line/part suffixes below locate verbatim examined fragments; they are not new specification artifact identities. Context-only entries identify supporting contracts.

```yaml
{
  "clean": true,
  "scope": [
    {
      "id": "FR-322-line-7-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  - target: ix://agent-ix/filament-specification/FR-491"
    },
    {
      "id": "FR-322-line-8-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "    type: references"
    },
    {
      "id": "FR-322-line-162-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "| claim_names | absent exactly when the graph has no state/state_clause, relation/refinement, correspondence/component_composition or claim node other than a claim/value_validity node; see Field sources |"
    },
    {
      "id": "FR-322-line-1038-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  `protocol`, `claim`, `extension` or `hybrid` node, a"
    },
    {
      "id": "FR-322-line-1039-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  `correspondence`/`component_composition` node, or a `value` node other than an `enum_value`, that"
    },
    {
      "id": "FR-322-line-1151-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  node, its *owner*, of the per-obligation occurrences a refinement owner"
    },
    {
      "id": "FR-322-line-1152-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  carries on its own node (\"Refinement step occurrences\" below), and of the"
    },
    {
      "id": "FR-322-line-1153-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  per-member occurrences a composition declaration carries on its own node"
    },
    {
      "id": "FR-322-line-1154-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  ([FR-491](../../functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md),"
    },
    {
      "id": "FR-322-line-1155-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  \"Composition member occurrences\"). For a compiler-derived node, its ordinal is the node's position among the derived"
    },
    {
      "id": "FR-322-line-1658-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  state clause), each `relation`/`refinement` node, each"
    },
    {
      "id": "FR-322-line-1659-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "  `correspondence`/`component_composition` node and each `claim` node (a"
    },
    {
      "id": "FR-322-line-5010-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "A state clause, a refinement, a component composition and a non-temporal claim are named by the"
    },
    {
      "id": "FR-322-line-5029-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`relation`/`refinement`, `correspondence`/`component_composition` or `claim` node other than a `claim`/`value_validity`"
    },
    {
      "id": "FR-322-line-5064-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "**Composition declaration names.** A"
    },
    {
      "id": "FR-322-line-5065-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`correspondence`/`component_composition` node SHALL carry its declared"
    },
    {
      "id": "FR-322-line-5066-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "composition name in `declaration.qualified_name` and exactly one"
    },
    {
      "id": "FR-322-line-5067-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`claim_names` row, keyed to its `declaration` occurrence at ordinal 0. The"
    },
    {
      "id": "FR-322-line-5068-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "row's `qualified_name` SHALL equal the node's"
    },
    {
      "id": "FR-322-line-5069-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`declaration.qualified_name`, and its `name` SHALL be that path's last"
    },
    {
      "id": "FR-322-line-5070-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "segment. Its `declaration_region` SHALL be the whole composition"
    },
    {
      "id": "FR-322-line-5071-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "declaration, equal to the single region of that declaration occurrence."
    },
    {
      "id": "FR-322-line-5072-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "The node's `node_id.path` SHALL equal this declaration path under"
    },
    {
      "id": "FR-322-line-5073-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "\"Structural node identity\": a top-level `compose Sys` has path `[\"Sys\"]`,"
    },
    {
      "id": "FR-322-line-5074-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "not a derived correspondence path. No other `correspondence` form becomes"
    },
    {
      "id": "FR-322-line-5075-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "eligible for `declaration` or `claim_names` under this rule."
    },
    {
      "id": "FR-322-line-5077-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "A consumer SHALL resolve a composition by these admitted names. For"
    },
    {
      "id": "FR-322-line-5078-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "default source-order selection, the consumer SHALL order composition"
    },
    {
      "id": "FR-322-line-5079-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "declarations by their `declaration_region` source's position in"
    },
    {
      "id": "FR-322-line-5080-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`lock.sources`, then start byte, then end byte. The consumer SHALL NOT"
    },
    {
      "id": "FR-322-line-5081-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "derive a composition name from a structural path, expression-origin label"
    },
    {
      "id": "FR-322-line-5082-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "or source text. The member coordinates in"
    },
    {
      "id": "FR-322-line-5083-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "[FR-491](../../functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md)"
    },
    {
      "id": "FR-322-line-5084-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "give the source order of the composition's members separately from this"
    },
    {
      "id": "FR-322-line-5085-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "declaration order."
    },
    {
      "id": "FR-322-line-5125-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   `state`/`state_clause`, `relation`/`refinement`,"
    },
    {
      "id": "FR-322-line-5126-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   `correspondence`/`component_composition` or `claim` node, or that"
    },
    {
      "id": "FR-322-line-5137-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   `/claim_names/{i}/name`. A `correspondence`/`component_composition`"
    },
    {
      "id": "FR-322-line-5138-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   row whose `qualified_name` differs from its node's"
    },
    {
      "id": "FR-322-line-5139-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   `declaration.qualified_name` refuses the same code and cause at"
    },
    {
      "id": "FR-322-line-5140-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "   `/claim_names/{i}/qualified_name`."
    },
    {
      "id": "FR-322-line-5176-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "region. A composition row also SHALL equal its declaration occurrence's"
    },
    {
      "id": "FR-322-line-5177-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "single region, rather than merely contain it; a mismatch refuses"
    },
    {
      "id": "FR-322-line-5178-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`invalid_package`/`invalid-value` at"
    },
    {
      "id": "FR-322-line-5179-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`/claim_names/{i}/declaration_region` in the same region-check step. The"
    },
    {
      "id": "FR-322-line-5180-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "package writer (S8) writes it from the checked declaration's span."
    },
    {
      "id": "FR-322-line-6696-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "| FR-322-AC-514 | An admitted package for top-level `compose Sys` carries `declaration.qualified_name` `[\"Sys\"]`, `node_id.path` `[\"Sys\"]`, one declaration occurrence at ordinal 0 and one `claim_names` row with `name` `Sys`, `qualified_name` `[\"Sys\"]` and the exact whole declaration region. Strict readback resolves bare `Sys` to that occurrence without source text or expression-origin names. Two compositions whose node-id order reverses their declaration-region source order retain source-order default selection by locked source-unit position and byte interval. Another correspondence form remai"
    },
    {
      "id": "FR-322-line-6696-part-2",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "ns ineligible for declaration metadata and a claim-name row. | Test |"
    },
    {
      "id": "FR-322-line-6697-part-1",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "| FR-322-AC-515 | Mutate AC-514's otherwise admitted composition package independently: remove its naming row and omit `claim_names` when no other claim-bearing node remains; give the row a different `qualified_name` with matching last-segment `name`; change its declaration region to a larger containing interval; and introduce a distinct declaration with the same qualified name. The missing row/member refuses `invalid_package`/`invalid-value` at `/claim_names`; the differing path refuses the same at the row's `/qualified_name`; the larger interval refuses the same at its `/declaration_region`;"
    },
    {
      "id": "FR-322-line-6697-part-2",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": " the distinct duplicate name refuses `ambiguous_declaration`/`ambiguous-name` at the later row's `/qualified_name`. Existing source-map and declared-node path checks remain in force. | Test |"
    },
    {
      "id": "FR-491-line-177-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "- **semantic_type** and **dependencies** as FR-490's node, the dependencies"
    },
    {
      "id": "FR-491-line-178-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  being every `subject`, `clause`, `context` and `assumption` target."
    },
    {
      "id": "FR-491-line-179-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "- **declaration** — `{qualified_name}`, the composition's declared"
    },
    {
      "id": "FR-491-line-180-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  package-local name, with the naming row required by"
    },
    {
      "id": "FR-491-line-181-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)"
    },
    {
      "id": "FR-491-line-182-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  \"Composition declaration names\". Its **node_id** uses this declaration"
    },
    {
      "id": "FR-491-line-183-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  path under FR-322's structural-id rule."
    },
    {
      "id": "FR-491-line-184-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "- **occurrences** and **source_map** — the declaration and member"
    },
    {
      "id": "FR-491-line-185-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  occurrences below, replacing FR-490's declaration-only occurrence rule"
    },
    {
      "id": "FR-491-line-186-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "  for this node form."
    },
    {
      "id": "FR-491-line-188-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "### Composition member occurrences"
    },
    {
      "id": "FR-491-line-190-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "The composition node SHALL carry one `declaration` occurrence at ordinal 0,"
    },
    {
      "id": "FR-491-line-191-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "whose `source_map` entry holds exactly one region: the whole composition"
    },
    {
      "id": "FR-491-line-192-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "declaration, from `compose` to its closing byte."
    },
    {
      "id": "FR-491-line-194-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "The composition node SHALL carry one `generated` occurrence per body"
    },
    {
      "id": "FR-491-line-195-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "member on its own node. The member sequence is the concatenation of the"
    },
    {
      "id": "FR-491-line-196-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "body arrays `components`, `guarantees`, `relies`,"
    },
    {
      "id": "FR-491-line-197-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "`liveness_dependencies`, `supplies` and `environment`, in that order,"
    },
    {
      "id": "FR-491-line-198-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "preserving each array's canonical order. A member's zero-based position"
    },
    {
      "id": "FR-491-line-199-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "in that sequence SHALL be its `generated` ordinal. An empty array"
    },
    {
      "id": "FR-491-line-200-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "contributes no position. These ordinals identify body members, not their"
    },
    {
      "id": "FR-491-line-201-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "source order; a member's ordinal can change when a different member is"
    },
    {
      "id": "FR-491-line-202-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "inserted earlier in the canonical sequence."
    },
    {
      "id": "FR-491-line-204-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "The package writer SHALL capture each member's exact authored region"
    },
    {
      "id": "FR-491-line-205-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "before canonical body ordering. The corresponding `generated` entry's"
    },
    {
      "id": "FR-491-line-206-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "`regions` SHALL contain exactly that one region, under the declaration's"
    },
    {
      "id": "FR-491-line-207-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "source reference and strictly within its declaration region. Distinct"
    },
    {
      "id": "FR-491-line-208-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "member regions SHALL be nonempty and pairwise nonoverlapping. The writer SHALL"
    },
    {
      "id": "FR-491-line-209-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "carry no other occurrence on the composition node. The entry is admitted"
    },
    {
      "id": "FR-491-line-210-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "producer provenance; a reader SHALL NOT parse source text or an"
    },
    {
      "id": "FR-491-line-211-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "expression-origin label to recover or verify it."
    },
    {
      "id": "FR-491-line-213-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "Within each member kind, the consumer SHALL recover authored order from"
    },
    {
      "id": "FR-491-line-214-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "these member regions, by start byte then end byte, before deriving slots"
    },
    {
      "id": "FR-491-line-215-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "or applying source-order static checks. Canonical body-array order SHALL"
    },
    {
      "id": "FR-491-line-216-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "NOT substitute for member source order. A component-guarantee, rely,"
    },
    {
      "id": "FR-491-line-217-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "liveness-dependency or environment slot SHALL use the region of the"
    },
    {
      "id": "FR-491-line-218-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "member that produces it. A fairness slot SHALL reuse the region of the"
    },
    {
      "id": "FR-491-line-219-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "first member that derives its premise, visiting guarantees in authored"
    },
    {
      "id": "FR-491-line-220-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "order, then relies in authored order, then liveness dependencies in"
    },
    {
      "id": "FR-491-line-221-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "authored order; a premise already derived is not derived again. A"
    },
    {
      "id": "FR-491-line-222-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "fixed-point slot SHALL use the region of the first cycle edge in"
    },
    {
      "id": "FR-491-line-223-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "authored order. Several slots derived from one member reuse its admitted"
    },
    {
      "id": "FR-491-line-224-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "coordinate; the declaration region is not a substitute member region."
    },
    {
      "id": "FR-491-line-226-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "After FR-322's generic source-map checks, the reader SHALL examine the"
    },
    {
      "id": "FR-491-line-227-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "composition node's entries in `source_map` order and refuse the first"
    },
    {
      "id": "FR-491-line-228-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "invalid entry as `invalid_package`/`invalid-value` at `/source_map/{i}`:"
    },
    {
      "id": "FR-491-line-229-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "a role or ordinal outside the declaration/member pairs above; a"
    },
    {
      "id": "FR-491-line-230-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "`regions` array with other than one region; or a member region whose"
    },
    {
      "id": "FR-491-line-231-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "source differs from the declaration's, whose interval is empty or not"
    },
    {
      "id": "FR-491-line-232-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "strictly within that declaration region, or which overlaps an earlier"
    },
    {
      "id": "FR-491-line-233-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "member entry's region. When all entries pass, the reader SHALL refuse a"
    },
    {
      "id": "FR-491-line-234-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "missing required declaration or member occurrence as"
    },
    {
      "id": "FR-491-line-235-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "`invalid_package`/`invalid-value` at `/source_map`. Generic duplicate-key,"
    },
    {
      "id": "FR-491-line-236-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "unknown-node and total-occurrence/source-map checks retain their"
    },
    {
      "id": "FR-491-line-237-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "existing refusals and precedence. The member-coordinate check SHALL"
    },
    {
      "id": "FR-491-line-238-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "run after the node's identity, dependency, body-term and canonical-order"
    },
    {
      "id": "FR-491-line-239-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "checks and before static checks 2 to 5."
    },
    {
      "id": "FR-491-line-242-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "order of checks and the member-coordinate check above, then static checks"
    },
    {
      "id": "FR-491-line-243-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "2 to 5 above on the read values; a"
    },
    {
      "id": "FR-491-line-258-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "| FR-491-AC-7 | A `compose Sys` whose two guarantees are authored in the reverse of their canonical body order carries a declaration occurrence at ordinal 0 and one generated occurrence per member, keyed by the canonical concatenation above. Each reads back with its own literal authored region, and its component-guarantee slots retain authored guarantee order and those exact regions. A fairness premise derived first by the first authored guarantee reuses that guarantee's coordinate. Reordering the node's source-map entries changes neither member attribution nor slot order. | Test |"
    },
    {
      "id": "FR-491-line-259-part-1",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "| FR-491-AC-8 | Independently mutate AC-7's otherwise valid package, retaining generic source-map validity: remove a member's occurrence and matching entry together; change both to an ordinal equal to the member count; change a member region to another locked source, outside the declaration interval, equal to the declaration interval, empty, overlapping an earlier member's region, or two nonoverlapping regions; and replace the member's role in both its occurrence and entry with `expression`. The missing pair refuses `invalid_package`/`invalid-value` at `/source_map`; each other mutation refuse"
    },
    {
      "id": "FR-491-line-259-part-2",
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "s the same code and cause at that entry's `/source_map/{i}`. No member inherits the declaration's region, and no source text is recompiled. | Test |"
    },
    {
      "id": "FR-322-schema-DeclarationTagRules",
      "path": "proposals/checked-package-v2/schema.json",
      "role": "context_only",
      "excerpt": "    \"DeclarationTagRules\": {\n      \"if\": {\n        \"anyOf\": [\n          {\n            \"properties\": {\n              \"node_tag\": {\n                \"enum\": [\n                  \"expression\",\n                  \"state\",\n                  \"temporal\",\n                  \"correspondence\"\n                ]\n              }\n            },\n            \"required\": [\n              \"node_tag\"\n            ]\n          },\n          {\n            \"properties\": {\n              \"node_tag\": {\n                \"const\": \"relation\"\n              },\n              \"semanti"
    },
    {
      "id": "FR-322-schema-correspondence-body",
      "path": "proposals/checked-package-v2/schema.json",
      "role": "context_only",
      "excerpt": "                  \"description\": \"FR-451, FR-490, FR-491: each semantic_form takes its own body.\",\n                  \"allOf\": [\n                    {\"if\": {\"properties\": {\"semantic_form\": {\"const\": \"abstraction_relation\"}}}, \"then\": {\"properties\": {\"body\": {\"$ref\": \"#/$defs/AbstractionRelationBody\"}}}},\n                    {\"if\": {\"properties\": {\"semantic_form\": {\"const\": \"implementation_guarantee\"}}}, \"then\": {\"properties\": {\"body\": {\"$ref\": \"#/$defs/ImplementationGuaranteeBody\"}}}},\n                    {\"if\": {\"properties\": {\"semantic_form\": "
    },
    {
      "id": "FR-490",
      "path": "spec/functional/ir-proof/FR-490-spell-and-check-an-implementation-guarantee-declaration.md",
      "role": "context_only",
      "excerpt": "- **semantic_type** — the node's own `node_id`; **dependencies** — exactly\n  the unique, node-id-ascending set of `clause`, `relation` and every\n  `context` and `assumption` target; **occurrences** — one, role\n  `declaration`, at the `guarantee-decl`; each member keeps its own source\n  region in the node's `source_map` entry, so every derived item's source\n  correspondence resolves to the member that produced it; **node_id** —\n  FR-322's structural id `{package, path, ordinal}`.\n\nThe reader runs this step after FR-451's abstraction step, node b"
    },
    {
      "id": "FR-492",
      "path": "spec/objects/interfaces/FR-492-planner-settlement-request.md",
      "role": "context_only",
      "excerpt": "- `slots` — exactly the declaration's derived premise slots, in derived\n  order (FR-490 \"Derived items\", FR-491 \"Derived item and slots\"). Each is\n  `{role, identity, result?, record?}`: `role` the slot's closed role;\n  `identity` the slot key the declaration derives; `result` a\n  premise result, present exactly on a filled item-backed slot; `record` an\n  environment record, present exactly on a filled `environment` slot. A slot\n  the declaration derives with no item (an operation with no `progress`\n  member, a required scheduling premise no `e"
    }
  ],
  "findings": [],
  "bindings": []
}
```
