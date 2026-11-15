---
id: TC-1443
title: "Source object protocol joins, operation effects and identity rows"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-610
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-464
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-465
    type: verifies
---

# TC-1443: Source object protocol joins, operation effects and identity rows

## Description

Verify that protocol clauses and roles join a source-declared object type
through its `SourceOwner`, and that operation effects, clause functions and
identity rows carry the source forms. Scope: FR-610-AC-7 and FR-610-AC-8.

## Test Procedure

1. Emit a unit with `protocol Sell ... over (view: Order)` and `role Seller on
   Order` over the source `Order`; read the clause's `owner_model` and the
   role's `model`.
2. Change the `SourceOwner` of `Order`'s node to an authority no
   `lock.sources` entry holds; read. Retarget `owner_model` to a native record
   node; read.
3. Read `Order::bump`'s `operation_effects` row and the clause function of
   `pre Qs using P on Order::size`.
4. Read the source-object identity rows of `Order`, `Sub` and `Shape`; then
   supply them with a row of a foreign authority added, with `Order`'s row
   naming a frame node, and with `Order`'s row removed.

## Expected Results

Step 1 admits both edges. Step 2 refuses `missing_declaration`/
`missing-selection` at `Order`'s `node_id`, then
`invalid_model_binding`/`malformed-declaration` at `owner_model`. Step 3
matches FR-610-AC-8. Step 4 reads the rows back equal, and each altered
supply is refused.
