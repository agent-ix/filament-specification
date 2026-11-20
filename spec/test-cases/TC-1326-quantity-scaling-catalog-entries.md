---
id: TC-1326
title: "Quantity scaling operations in the catalog and the reader"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-451
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-142-AC-41
    type: verifies
---

# TC-1326: Quantity scaling operations in the catalog and the reader

## Description

Verify the FR-322 "Quantity scaling" entries: the reader admits
`filament.op.quantity.scale` and `filament.op.quantity.scale_div` applications
of the catalogued shape, refuses every other shape, and the V2 schema and the
operation catalog enumerate the two identities after `filament.op.quantity.pow`.

## Test Procedure

Read the schemas' `OperationIdentity` enumeration and the catalog entries of the
two identities. Build applications over a `quantity` typed by the unit node of
`ms` and an `integer`, `decimal` or `rational` factor, then the refused shapes of
FR-322-AC-451, and run the full reader on each.

## Expected Results

The enumerations hold the two identities directly after `filament.op.quantity.pow`,
each catalog entry is operator `binary`, operands `["quantity", "exact_numeric"]`,
result `operand:0`, with no law, mode or member; the admitted applications read
with result type the unit node of `ms`; each refused shape gives the code and
cause FR-322-AC-451 lists.
