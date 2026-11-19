---
id: FR-476
title: "Checked-package model terminal declaration body"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-366
    type: references
  - target: ix://agent-ix/filament-specification/FR-465
    type: references
  - target: ix://agent-ix/filament-specification/FR-341
    type: references
  - target: ix://agent-ix/filament-specification/FR-361
    type: references
  - target: ix://agent-ix/filament-specification/FR-475
    type: references
---

# FR-476: Checked-package model terminal declaration body

## Description

When a checked package carries a model `terminal` declaration
([FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md)), the
producer SHALL emit it as one `filament.checked-semantic-graph/v2` node with
`node_tag: "temporal"` and `semantic_form: "terminal"`, whose body is the
aggregate below, and the reader SHALL refuse a node whose rules below do not
hold, with the first refusal in the reader order below. The node is the only
carrier of the declaration: a deadlock-freedom item, its classifier and its
replay read the model's intended terminal states from this node and from
nothing else.

## Properties

- **body** — `{term: "aggregate", members: [binding("model", …),
  binding("mode", …), binding("parameters", …), binding("condition", …)]}`,
  in this order:
  - `model` — a `reference` to the `model` node of the model alias the
    declaration's `on` names, the node an FR-402 `subjects` binding names for
    that alias;
  - `mode` — a `literal` text term, `when` or `any`;
  - `parameters` — an aggregate of at most one `reference`: to the `self`
    `value`/`parameter` node at level 0, typed `Reference<M::T>` for the
    object type `T` that [FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md)
    binds `self` to, when the condition reads `self`; empty otherwise and for
    `any`;
  - `condition` — for `when`, a `reference` to the predicate node of the
    Boolean state predicate `P`; for `any`, the empty aggregate
    `{term: "aggregate", members: []}`. The predicate node is one of the two
    node forms [FR-475](FR-475-checked-package-state-graph-claim-body.md)
    admits as a predicate, with `semantic_type` the `scalar_type`/`boolean`
    node: an `expression` node, the predicate's Boolean term, or a
    `value`/`literal` node, a constant predicate such as `true` or `false`,
    as [FR-322](FR-322-checked-package-artifact.md)'s body grammar encodes
    it. A predicate that reads no state is a valid Boolean condition:
    `terminal T using v on M when { true }` intends every terminal state of
    the model and `when { false }` none, exactly as the same constant does
    as a state-graph claim predicate. A constant predicate reads no `self`,
    so its `parameters` is empty.
- **declaration** — none. A `temporal` node carries no `declaration` (the
  schema's `DeclarationTagRules`). The node has a derived node id and is nested
  under its model by the `model` reference: the written name `T` of
  `terminal T using v on M …` is presentation, carried only by the node's
  source occurrence, so renaming `T` changes neither the node id nor any
  obligation identity.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **occurrences** — one occurrence of role `claim` per source declaration
  that checks to this node, at the `terminal-decl` it was produced from, as
  every other undeclared clause node carries
  ([FR-341](FR-341-checked-package-state-clause-body.md),
  [FR-370](FR-370-checked-package-temporal-clause-body.md)), with ordinals
  0, 1, … in source order. Two declarations check to one node only when
  they are written on two model aliases that select one model identity and
  have equal mode, parameters and condition (FR-366 "Several
  declarations"); the node then carries both, one `claim` occurrence and
  one `source_map` entry each, so neither declaration's locus is lost.
- **dependencies** — exactly the unique, node-id-ascending set of the body's
  `reference` targets: the `model` node, the `self` parameter node when
  present and, for `when`, the condition node.

The body carries no `using` profile alias: the profile is the unit's value
profile, which the lock already selects.

## Identity

The node id follows FR-322's rules over the body, so the mode, the model
and the condition each enter the node id. Two packages whose `terminal`
declarations differ only in mode, or only in the condition, are not
structurally equal. A model subject binds its checked package
through the package identity
([FR-361](../../functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md)),
so every obligation identity over the subject, the derived deadlock-freedom
item's included, is a different identity when the model's `terminal` node
differs; the request carries no separate terminal member.

## Several terminal nodes on one model node

A package names a model by the identity of its model selection, never by a
source alias: the `model` binding references the selection's
`model`/`model_root` node, whose structural key is `{package: <selected
identity>, path: ["model", "model_root"], ordinal: 0}`. Two aliases that
select one model identity therefore reference one model node, and the
package cannot tell their declarations apart by alias. A model node MAY be
referenced by several `temporal`/`terminal` nodes, one per distinct
declaration on it, and one terminal node MAY carry several `claim`
occurrences (above). Each is carried; none is merged into another or
dropped.

- **Terminal row order.** The package's terminal nodes are ordered by the
  structural key of their `model` target (FR-322's ascending node-id order,
  so by selection identity in UTF-8 byte order), then by their own
  structural key. This order is a function of the semantic graph:
  reordering the source declarations, the `model` aliases or the graph
  array leaves it unchanged, though source regions and the order of equal
  declarations' `claim` occurrences follow the text.
- **Terminal ordinal.** A terminal node's *ordinal* is its zero-based
  position among the terminal nodes of its model node in that order.
- **Terminal key.** A terminal node is keyed by (the selection identity of
  its model node, its ordinal). This key, not a source alias, is how every
  consumer names a terminal declaration: a deadlock classifier's report, a
  replay locus and a verdict join. A declaration among a node's several
  `claim` occurrences is named by the node's key and the occurrence's
  ordinal. A source alias is a display label only.

FR-366 "Several declarations" states how the declarations combine when a
terminal state is classified.

## Reader order

The terminal step is a body step of
[FR-322](FR-322-checked-package-artifact.md) "Body steps": it runs after the
abstraction step and before the protocol step. It visits
every `temporal`/`terminal` node in ascending node-id order. For one
node, the checks run in this order and the first failing check is its
defect:

1. **Shape.** The body is an aggregate of exactly the four bindings in order,
   else `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/body`.
2. **Mode.** `mode` is `when` or `any`, else `invalid_package`/`invalid-value`
   at `/semantic_graph/nodes/{n}/body/members/1`.
3. **Mode fit**, in body order: `parameters` holds at most one entry, and
   none when `mode` is `any`, else `invalid_semantic_graph` at the
   `parameters` binding; then `condition` is a `reference` exactly when
   `mode` is `when`, else `invalid_semantic_graph` at the `condition`
   binding. An `any` node with a non-empty `parameters` and a `condition`
   reference therefore refuses at `parameters`.
4. **Targets**, in body order: a `model` target that is not a
   `model`/`model_root` node refuses `invalid_model_binding`/`malformed-declaration`; a `parameters`
   target that is not a `value`/`parameter` node, or a `condition` target
   that is neither an `expression` node nor a `value`/`literal` node (the two
   predicate node forms above), refuses `ill_typed`/`operator-ineligible`;
   a `parameters` target that is not named `self` at level 0, or whose type is
   not `Reference<M::T>` for FR-366's `T` of the referenced model, refuses
   `ill_typed`/`operator-ineligible`, as FR-341-AC-4 refuses a mistyped
   `self`; a `condition` target whose `semantic_type` is not the
   `scalar_type`/`boolean` node refuses `ill_typed`/`non-boolean-root`. The
   locus is the target.
5. **Rows.** Once every `temporal`/`terminal` node's own checks hold, the
   reader orders the nodes into terminal rows and assigns each its ordinal
   ("Several terminal nodes on one model node"). This step refuses nothing:
   several terminal nodes on one `model` node, and several `claim`
   occurrences on one terminal node, are admitted, because a package records
   a model by selection identity and cannot see the source aliases whose
   declarations FR-366 limits to one each. The one-per-alias rule is the
   checker's (FR-366), applied to the source before a package exists.

A `dependencies` list that differs from the body's targets is FR-322's
graph-shape refusal, `invalid_semantic_graph` at the node's `/dependencies`,
which precedes this step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-476-AC-1 | `terminal Done using v on M when { P }` produces one `temporal`/`terminal` node whose body has the bindings `model`, `mode`, `parameters` and `condition` in that order, with `mode` `when` and `condition` a reference to `P`'s `expression` node; `terminal Halt using v on M any` produces `mode` `any`, an empty `parameters` and an empty `condition`; neither node carries a `declaration`, its occurrences have role `claim` and its `dependencies` are exactly the body's targets. | Test (TC-961) |
| FR-476-AC-2 | A `when` condition that reads `self` holds one reference in `parameters`, to the `self` parameter node at level 0 typed `Reference<M::C>`; one that reads no `self` has empty `parameters`. | Test (TC-961) |
| FR-476-AC-3 | Changing the mode or the condition changes the node id; renaming `T` does not; the deadlock-freedom item over a subject whose model has a `terminal` node has a different obligation identity from the same subject whose model has none. | Test (TC-961) |
| FR-476-AC-4 | With `dependencies` rebuilt after each edit: a body with `mode` and `condition` swapped in order refuses `invalid_semantic_graph`; `mode` `some` refuses `invalid_package`/`invalid-value`; an `any` node with a `condition` reference, a `when` node with an empty `condition`, a `parameters` with two entries, and an `any` node with both a `parameters` entry and a `condition` reference (at `parameters`) refuse `invalid_semantic_graph`; a `model` target that is a `value`/`parameter` node, and one that is a `model`/`object_type` node, refuse `invalid_model_binding`/`malformed-declaration`; a `parameters` entry naming a parameter `result` or typed `Reference<M::D>` for another object type refuses `ill_typed`/`operator-ineligible`; a `condition` naming an integer-typed expression node refuses `ill_typed`/`non-boolean-root`. | Test (TC-961) |
| FR-476-AC-5 | `terminal Done using v on M when { true };` produces a node whose `condition` references the Boolean `value`/`literal` node of `true`, with empty `parameters` and `dependencies` the `model` node and that literal node; the reader admits it, and `when { false }` likewise. The `true` node's id differs from MT-02's `any` node's id. With `dependencies` rebuilt: `condition` referencing the `self` `value`/`parameter` node refuses `ill_typed`/`operator-ineligible` at the target, and referencing an added integer `value`/`literal` node of `3` refuses `ill_typed`/`non-boolean-root` at the target. | Test (TC-961) |
| FR-476-AC-6 | In a unit whose aliases `M` and `N` select one model identity, `terminal Done using v on M when { self.value = 3 };` with `terminal Halt using v on N any;` produces two terminal nodes whose `model` binding references the one `model_root` node of that identity; the reader admits the package, and the rows are the two nodes in ascending terminal node structural key (the test establishes that order by comparing the rebuilt keys), with ordinals 0 and 1 and keys (that identity, 0) and (that identity, 1). The same unit with the two `model` alias lines swapped, or with the two `terminal` declarations swapped, gives an equal semantic graph (equal node ids and bodies) and equal rows; only source regions in the `source_map` follow the text. `terminal Halt using v on M any;` with `terminal Stop using v on N any;` produces one node with two `claim` occurrences, ordinals 0 and 1, each with its own `source_map` entry at its declaration; the reader admits it as one row with ordinal 0. In a unit whose alias `V` selects `test/vault` and alias `N` selects `test/vault-other`, a terminal on `N` and one on `V` give rows in the order `test/vault`, then `test/vault-other`, each with ordinal 0, the reverse of the aliases' order. | Test (TC-961) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `terminal` among the
`temporal` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the node form, keys,
  `declaration`, `dependencies` and the reader's refusal order.
- [FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md) owns
  the declaration's grammar and meaning and the derived deadlock-freedom item.
- [FR-465](FR-465-checked-package-protocol-declaration-bodies.md) "Terminal"
  is the protocol member with the same `when`/`any` layout.
- [FR-341](FR-341-checked-package-state-clause-body.md) binds `self` for a
  state predicate the same way.

## References

- STD-303 (this requirement); filament-spec-language FR-124 and ADR-018 DL-1 to
  DL-3.
