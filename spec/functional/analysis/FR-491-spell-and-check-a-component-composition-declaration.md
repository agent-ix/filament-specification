---
id: FR-491
title: "Spell and check a component composition declaration"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-489
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-348
    type: references
  - target: ix://agent-ix/filament-specification/FR-176
    type: references
  - target: ix://agent-ix/filament-specification/FR-362
    type: references
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-380
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-490
    type: references
  - target: ix://agent-ix/filament-specification/FR-492
    type: references
---

# FR-491: Spell and check a component composition declaration

## Description

When a source unit declares a `composition-decl` over imported checked
component packages, the compiler SHALL check it into one FR-489 composition
declaration, carry it in the checked package as one
`correspondence`/`component_composition` node whose four premise lists stay
separate, and derive from it the composition item and its premise slots,
each with its own obligation identity and source correspondence.

## Inputs

- The unit's `composition-decl` declarations, recognized by the
  [complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md)
  "Component composition declaration" production.
- The checked packages the unit imports (`import-decl`), with their model
  and protocol subjects and temporal clauses, and the unit's selected
  profiles.

## Outputs

One checked composition declaration per `composition-decl`, its
checked-package node and its derived item and slots; or a located refusal
per inadmissible member.

## Behavior

### Declaration form

`compose S rule R { components members }` declares composition `S` under
rule `R`:

| Member | Meaning |
| --- | --- |
| `rule R` | the selected rule: `assume-guarantee` (FR-176), `protocol-refinement` (FR-177), `refinement-transfer` (FR-380) or `interface` (FR-348) |
| `component C from I using M;` | component `C` is the subject `M` (a model alias) of the checked package imported as `I`: its subject identity is that package's identity and `M`'s subject |
| `guarantee C::G;` | `G`, a temporal clause of `C`'s package over `M`, is a component guarantee of the composed claim |
| `rely C::G on D::H;` | a `relies` premise: guarantee `G` of `C` relies on the safety guarantee `H` of peer `D` |
| `depends C::G on D::H;` | a `liveness_dependencies` premise: liveness guarantee `G` of `C` depends on the progress of liveness guarantee `H` of peer `D` |
| `supplies fair K Q D::T::o;` | a fairness constraint (FR-362) the composition supplies to its components |
| `environment E by "s" ...;` | an `environment` premise, spelled as in [FR-490](../ir-proof/FR-490-spell-and-check-an-implementation-guarantee-declaration.md) |

The **composed claim** is the conjunction of the declared component
guarantees, each over its own component subject. The four FR-489 premise
lists are derived and kept separate:

- `environment` — the `environment` members;
- `relies` — the `rely` members;
- `liveness_dependencies` — the `depends` members;
- `fairness` — one premise per constraint of the resolved fairness set of
  each declared guarantee and of each `H` a `rely` or `depends` member
  names, each discharged only by a `supplies` constraint that implies it
  under FR-380 "Fairness implication".

### Static checks

The compiler SHALL refuse, in source order, naming the member:

1. A package that does not select `filament.model.complete/v1`:
   `unsupported_construct` at the declaration. Fewer than two `component`
   members, a member that is not one of the forms above, or a rule spelling
   outside the four: `invalid_syntax` at the offending token.
2. `I` naming no imported package, `M` naming no subject of it, or `C::G`
   or `D::H` naming no temporal clause of that component's package over its
   subject: `missing_declaration`/`missing-name`. Two `component` members of
   one name, or two over one subject identity: `invalid_model_binding`/
   `conflicting-binding`, naming both.
3. A `rely` or `depends` member whose `C::G` is not a declared `guarantee`,
   or whose `C` equals `D`: `invalid_model_binding`/`malformed-declaration`.
4. A `rely` whose `H` FR-363 classifies as `liveness`, or a `depends` whose
   `G` or `H` it does not classify as `liveness`: `invalid_model_binding`/
   `malformed-declaration`, naming the member and the form: a premise has
   one kind, and a guarantee of one kind never discharges a premise of
   another (FR-489).
5. A `supplies` or scheduling `environment` operation that is no operation
   of a component subject: `missing_declaration`/`missing-name`.

### Dependency graph and cycles

The **dependency graph** has a node per declared guarantee and an edge
`C::G → D::H` per `rely` and `depends` member. A strongly connected
component of more than one node, or a node with an edge to itself, is a
**cycle**. A cycle is not a static refusal: it is settled by FR-489 "Cycles"
for the composition item alone, so the unit's other items are unaffected. A
cycle is admitted for settlement only when a profile the unit selects
declares FR-176's monotone complete lattice, initial element and
convergence rule; it then derives one `fixed-point` premise slot, filled by
the FR-176 composition result over the cycle's guarantees, which carries
every iteration (FR-176-AC-4). Otherwise the composition item refuses
`invalid_model_binding`/`malformed-declaration`, naming the cycle's
guarantees in edge order, and no guarantee of the cycle is available as an
assumption.

### Derived item and slots

A checked declaration `S` derives:

| Rule | Item | Settled by |
| --- | --- | --- |
| `assume-guarantee` | one planner-local composition item (FR-368 "Item kinds"), whose obligation identity binds `S`'s node id, the package identity and every component subject identity | the planner, from [FR-492](../../objects/interfaces/FR-492-planner-settlement-request.md)'s settlement request |
| `protocol-refinement`, `refinement-transfer`, `interface` | one FR-331 `composition` request item over the component subjects, negotiated as [FR-348](FR-348-admit-compositional-verification.md) states; FR-489's four lists are its `premises` member, spelled as FR-492's `slots` | the routed composition-capable provider; with none registered, `unsupported` (FR-348-AC-4) |

The premise slots are, in this order: one `component-guarantee` slot per
declared guarantee, one `rely` slot per `rely`, one `liveness-dependency`
slot per `depends`, one `fairness` slot per derived fairness premise, one
`environment` slot per `environment` member, and one `fixed-point` slot per
admitted cycle. Their items and slot keys (FR-492) are:

- **component-guarantee** — item-backed: the item of `G` over `C`'s
  subject, checked under exactly the assumptions FR-176 gives `G` from its
  declared `rely` and `depends` members (the peer guarantees `G` may assume,
  each restricted to the shared interface), produced by `C`'s provider. The
  assumption set is part of the slot key, so a result for `G` checked under
  other assumptions, or under none, is another key. This is what makes the
  conjunction sound: each `G` holds of the composition once every
  assumption it was proved under is itself discharged (FR-176 "Discharge
  fixed point").
- **rely** and **liveness-dependency** — not item-backed: discharged from
  the `guarantees` pool or a `discharges` entry (FR-489 "Implication").
- **fairness** and **environment** — not item-backed: discharged by a
  `supplies` constraint, and filled by an environment record.
- **fixed-point** — item-backed: one FR-331 `composition` item anchored at
  `S`'s node, whose `premises` member names the cycle's component-guarantee
  slot keys and whose `options` name the declaring profile; its slot key
  binds those, so only FR-176's result over exactly this cycle under this
  profile fills it.

### Checked-package node

`S` is emitted as one `filament.checked-semantic-graph/v2` node with
`node_tag: "correspondence"` and `semantic_form: "component_composition"`:

- **body** — `{term: "component_composition", rule, components, guarantees,
  relies, liveness_dependencies, supplies, environment}`, all required:
  - `rule` — one of the four rule spellings;
  - `components` — `{name: Identifier, package: PackageIdentity, subject:
    NodeRef}` per `component`;
  - `guarantees` — `{component: Identifier, clause: NodeRef}` per
    `guarantee`;
  - `relies` and `liveness_dependencies` — `{guarantee: {component,
    clause}, on: {component, clause}}` per `rely` and per `depends`;
  - `supplies` — FR-490's scheduling `{kind, granularity, context,
    operation}` per `supplies`;
  - `environment` — FR-490's environment entries.
- **Canonical order** — `components` by `name` bytes; the other arrays by
  their first `component` name, then `clause` node id, then the second
  member's; `environment` by `name`. Out of order refuses
  `invalid_semantic_graph`.
- **semantic_type** and **dependencies** as FR-490's node, the dependencies
  being every `subject`, `clause`, `context` and `assumption` target.
- **declaration** — `{qualified_name}`, the composition's declared
  package-local name, with the naming row required by
  [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  "Composition declaration names". Its **node_id** uses this declaration
  path under FR-322's structural-id rule.
- **occurrences** and **source_map** — the declaration and member
  occurrences below, replacing FR-490's declaration-only occurrence rule
  for this node form.

### Composition member occurrences

The composition node SHALL carry one `declaration` occurrence at ordinal 0,
whose `source_map` entry holds exactly one region: the whole composition
declaration, from `compose` to its closing byte.

The composition node SHALL carry one `generated` occurrence per body
member on its own node. The member sequence is the concatenation of the
body arrays `components`, `guarantees`, `relies`,
`liveness_dependencies`, `supplies` and `environment`, in that order,
preserving each array's canonical order. A member's zero-based position
in that sequence SHALL be its `generated` ordinal. An empty array
contributes no position. These ordinals identify body members, not their
source order; a member's ordinal can change when a different member is
inserted earlier in the canonical sequence.

The package writer SHALL capture each member's exact authored region
before canonical body ordering. The corresponding `generated` entry's
`regions` SHALL contain exactly that one region, under the declaration's
source reference and strictly within its declaration region. Distinct
member regions SHALL be nonempty and pairwise nonoverlapping. The writer SHALL
carry no other occurrence on the composition node. The entry is admitted
producer provenance; a reader SHALL NOT parse source text or an
expression-origin label to recover or verify it.

Within each member kind, the consumer SHALL recover authored order from
these member regions, by start byte then end byte, before deriving slots
or applying source-order static checks. Canonical body-array order SHALL
NOT substitute for member source order. A component-guarantee, rely,
liveness-dependency or environment slot SHALL use the region of the
member that produces it. A fairness slot SHALL reuse the region of the
first member that derives its premise, visiting guarantees in authored
order, then relies in authored order, then liveness dependencies in
authored order; a premise already derived is not derived again. A
fixed-point slot SHALL use the region of the first cycle edge in
authored order. Several slots derived from one member reuse its admitted
coordinate; the declaration region is not a substitute member region.

The V2 schema SHALL admit the generic occurrence-role vocabulary on a
`component_composition` node; the exact role, ordinal and region constraints
above are reader checks, so a well-shaped wrong role reaches the located
semantic refusal below. The schema's declaration-presence rule SHALL apply
to this form: a declaration occurrence requires its `declaration` metadata.
Other correspondence forms retain their existing declaration exclusions
and occurrence-role restrictions.

After FR-322's generic source-map checks, the reader SHALL examine the
composition node's entries in `source_map` order and refuse the first
invalid entry as `invalid_package`/`invalid-value` at `/source_map/{i}`:
a role or ordinal outside the declaration/member pairs above; a
`regions` array with other than one region; or a member region whose
source differs from the declaration's, whose interval is empty or not
strictly within that declaration region, or which overlaps an earlier
member entry's region. When all entries pass, the reader SHALL refuse a
missing required declaration or member occurrence as
`invalid_package`/`invalid-value` at `/source_map`. Generic duplicate-key,
unknown-node and total-occurrence/source-map checks retain their
existing refusals and precedence. The member-coordinate check SHALL
run after the node's identity, dependency, body-term and canonical-order
checks and before static checks 2 to 5.

The reader runs this step after FR-490's guarantee step, with FR-490's
order of checks and the member-coordinate check above, then static checks
2 to 5 above on the read values; a
target in another package resolves through FR-322's dependency packages.
Reading the emitted bytes back yields the same declaration, the same four
separate lists and the same slots with the same identities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-491-AC-1 | A `Producer`/`Consumer` declaration with three guarantees, one `rely`, one `depends` and one `supplies` member checks and derives the composition item and, in order, three component-guarantee slots (each keyed with its FR-176 assumption set), one rely slot, one liveness-dependency slot and one fairness slot per resolved constraint, the four lists kept separate. | Test (TC-985) |
| FR-491-AC-2 | One `component` member, an unknown rule spelling and a member of another form refuse `invalid_syntax`; an unknown import alias, subject or clause refuses `missing_declaration`/`missing-name`; two components of one name refuse `invalid_model_binding`/`conflicting-binding`. | Test (TC-985) |
| FR-491-AC-3 | A `rely` on a liveness guarantee, a `depends` on a safety guarantee, a `rely` whose guarantee is undeclared, and a `rely` of a component on itself each refuse `invalid_model_binding`/`malformed-declaration` naming the member. | Test (TC-985) |
| FR-491-AC-4 | A mutual `depends` pair, with no selected profile declaring FR-176's lattice, checks; its composition item refuses `invalid_model_binding`/`malformed-declaration` naming both guarantees, and the unit's other items are derived unchanged. Under a profile declaring the lattice, the pair derives one fixed-point slot. | Test (TC-985) |
| FR-491-AC-5 | Under `interface`, the declaration derives one FR-331 `composition` request item whose `premises` member holds the four lists as FR-492 slots, and no planner-local item. | Test (TC-985) |
| FR-491-AC-6 | The emitted `correspondence`/`component_composition` node reads back to the same declaration, lists and slot identities; each slot's source correspondence resolves to its member; a node whose `relies` are out of order refuses `invalid_semantic_graph`. | Test (TC-985) |
| FR-491-AC-7 | A `compose Sys` whose two guarantees are authored in the reverse of their canonical body order carries a declaration occurrence at ordinal 0 and one generated occurrence per member, keyed by the canonical concatenation above, and passes the V2 schema. Each reads back with its own literal authored region, and its component-guarantee slots retain authored guarantee order and those exact regions. A fairness premise derived first by the first authored guarantee reuses that guarantee's coordinate. Reordering the node's source-map entries changes neither member attribution nor slot order. | Test |
| FR-491-AC-8 | Independently mutate AC-7's otherwise valid package, retaining V2-schema and generic source-map validity: remove a member's occurrence and matching entry together; change both to an ordinal equal to the member count; change a member region to another locked source, outside the declaration interval, equal to the declaration interval, overlapping an earlier member's region, or two nonoverlapping regions; and replace the member's role in both its occurrence and entry with `expression`. The missing pair refuses `invalid_package`/`invalid-value` at `/source_map`; each other mutation refuses the same code and cause at that entry's `/source_map/{i}`. Separately, a region whose `start >= end` fails the preceding generic source-map check as `invalid_source_map` at `/source_map/{i}/regions/{j}`, before any composition member-coordinate check. No member inherits the declaration's region, and no source text is recompiled. | Test |

## Dependencies

- [FR-489](FR-489-carry-liveness-and-fairness-premises-through-composition.md):
  the settlement of the item and slots this declaration derives.
- [FR-348](FR-348-admit-compositional-verification.md),
  [FR-176](../protocol/FR-176-compose-assume-guarantee-contracts.md) and
  [FR-380](FR-380-transfer-claims-through-refinement.md): the rules.
- [FR-490](../ir-proof/FR-490-spell-and-check-an-implementation-guarantee-declaration.md):
  the environment member and node conventions.
- [FR-492](../../objects/interfaces/FR-492-planner-settlement-request.md):
  the settlement request and slots.

## References

- Tickets: Linear QSL-1542.
