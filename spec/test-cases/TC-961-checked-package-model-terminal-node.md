---
id: TC-961
title: "Checked-package model terminal declaration node"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-476
    type: verifies
---

# TC-961: Checked-package model terminal declaration node

## Description

Check the `temporal`/`terminal` node of a model `terminal` declaration in
both modes, that mode and condition enter identity while the written name does
not, and the reader's refusals of a malformed node.

## Test Procedure

Compile each vector through the real local Rust boundary over FR-366's
counter model `M` (one object type `C` with field `value` and its one
population `M::Counters`), read the `temporal`/`terminal` node, and compare
node ids and the deadlock-freedom item's obligation identity.
For MT-07 to MT-16, edit the node of MT-01 or MT-02 as stated, adding to the
graph any node the edit references; then set the node's `dependencies` to the
body's targets, as TC-233 does for its patched
vectors, and feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| MT-01 | `terminal Done using v on M when { self.value = 3 };` | one node with no `declaration`; `mode` `when`; `parameters` holds one reference, to the `self` parameter node at level 0 typed `Reference<M::C>`; `condition` references the `expression` node of `self.value = 3`; occurrence role `claim` |
| MT-02 | `terminal Halt using v on M any;` | `mode` `any`; `parameters` and `condition` empty aggregates; `dependencies` the `model` node only |
| MT-03 | `terminal Done using v on M when { forall(c in M::Counters : c.value = 3) };` | `parameters` empty |
| MT-04 | MT-01 and MT-01 with `Done` renamed `Stop` | one node id |
| MT-05 | MT-01 and MT-02; MT-01 and MT-01 with condition `self.value = 2` | different node ids for each pair |
| MT-06 | the deadlock-freedom item over `M` with MT-01, and with no `terminal` declaration | different checked packages and so different obligation identities |
| MT-07 | MT-01 with `mode` and `condition` swapped in order | `invalid_semantic_graph` at the body |
| MT-08 | MT-01 with `mode` `some` | `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/1` |
| MT-09 | MT-02 with `condition` a reference to the added `expression` node of `self.value = 3`; MT-01 with `condition` the empty aggregate | `invalid_semantic_graph` at the `condition` binding, each |
| MT-10 | MT-01 with `model` a reference to the `self` parameter node; MT-01 with `model` a reference to the `C` `model`/`object_type` node | `invalid_model_binding`/`malformed-declaration` at the target |
| MT-11 | MT-01 with `condition` a reference to the added integer-typed `expression` node of `self.value + 1` | `ill_typed`/`non-boolean-root` at the target |
| MT-12 | a package holding MT-01's and MT-02's nodes for `M` | admitted: two terminal rows on `M`'s model node in ascending terminal node structural key, ordinals 0 and 1; no refusal (FR-476 reader step 5) |
| MT-13 | MT-01 with `parameters` holding the `self` node twice | `invalid_semantic_graph` at the `parameters` binding |
| MT-14 | MT-02 with `parameters` holding MT-01's `self` node and `condition` a reference to the added node of `self.value = 3` | `invalid_semantic_graph` at the `parameters` binding |
| MT-15 | MT-01 with its `self` entry replaced by an added parameter node named `result` at level 1; then by an added `self` node typed `Reference<M::D>` for a second object type `D` of `M` that no population covers, so `C` stays FR-366's `T` | `ill_typed`/`operator-ineligible` at the target, each |
| MT-16 | MT-01's node with a second `claim` occurrence (ordinal 1) added, with its own `source_map` entry, as two equal declarations on two aliases of one model identity produce | admitted as one row with ordinal 0, both occurrences retained |
| MT-17 | `terminal Done using v on M when { true };`, then `when { false }`; then MT-17 with `condition` referencing MT-01's `self` parameter node, and with `condition` referencing an added integer `value`/`literal` node `3` | `condition` references the Boolean `value`/`literal` node, `parameters` empty, admitted, node id different from MT-02's; then `ill_typed`/`operator-ineligible` and `ill_typed`/`non-boolean-root` at the target (FR-476-AC-5) |
| MT-18 | aliases `M` and `N` selecting one identity: MT-01 on `M` with MT-02 written on `N`; the same with the alias lines swapped and with the declarations swapped; `any` on both aliases | two nodes on one model node, rows ordinals 0 and 1, equal semantic graph (node ids and bodies) and rows under both swaps; one node with two `claim` occurrences, one row (FR-476-AC-6) |
| MT-19 | aliases `V` selecting `test/vault` and `N` selecting `test/vault-other`, a terminal on each | rows in the order `test/vault`, `test/vault-other`, each ordinal 0 (FR-476-AC-6) |

## Expected Results

Each mode has its own body, the written name never changes identity, the
deadlock-freedom identity reads the declaration, and each malformed node
refuses at the first failing reader step.
