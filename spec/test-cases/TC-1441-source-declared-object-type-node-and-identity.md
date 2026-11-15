---
id: TC-1441
title: "Source-declared object type node, member resolution and identity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-610
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1441: Source-declared object type node, member resolution and identity

## Description

Verify the checked-package form of a source-declared object type and that its
identity never meets a domain-package declaration's. Scope: FR-610-AC-3 and
FR-610-AC-4 (FR-322 "Source-declared object types" items 1, 2 and 4 and
"Identity against a domain declaration").

Catches a source type keyed by member content, a field member resolved
through a domain document, and a node that mixes owner forms.

## Test Procedure

1. Emit the unit of TC-1440 step 1 and read the `Order`, `Sub` and `Shape`
   nodes: `node_tag`, `semantic_form`, `declaration`, `owner`,
   `semantic_type`, body bindings and occurrences.
2. Re-emit with a field added to `Order`, then with `Order` renamed; compare
   node ids.
3. Read a `field` member `total` and a member `ghost` on `Order`'s node with
   no domain document supplied.
4. Emit a unit that declares `Order` and selects a domain package declaring
   `Order` as `M`; compare the two `object_type` nodes.
5. Apply each mutation to the step 1 package and read it: `Order`'s owner
   replaced by a `ModelOwner`; a `model`/`object_type` node with a
   `SourceOwner` and its `declaration` removed; `Order`'s `SourceOwner`
   authority changed to one no `lock.sources` entry holds.
6. Validate against the V2 schema a `relation`/`population` node with a
   `declaration` and a `SourceOwner`, and a `relation`/`relationship` node with
   a `declaration`.

## Expected Results

Step 1 matches FR-610-AC-3. In step 2 the added field leaves the id unchanged
and the rename changes it. Step 3 resolves `total` to its binding and refuses
`ghost` `missing_declaration`/`missing-name`. Step 4 gives two nodes with
different ids and owners. Step 5 refuses `malformed_wire` at the owner,
`malformed_wire` at the owner and `missing_declaration`/`missing-selection` at
the node id, in that order. Step 6 admits the population node and refuses the
relationship node.
