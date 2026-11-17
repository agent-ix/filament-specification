---
id: TC-1921
title: "Population only in the catalog and the reader"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1921: Population only in the catalog and the reader

## Description

Verify that the operation catalog and both V2 schemas carry
`filament.op.collection.only`, `filament.op.collection.seq` and
`filament.op.refinement.history` together, and that the reader admits a
lowered `only` and refuses its malformed shapes. Scope: FR-322-AC-455.

## Test Procedure

1. Read both schemas' `OperationIdentity` and `OperationMember` enumerations
   and the catalog entries of the three identities.
2. Read an `only(x in P where c: e)` application in a clause function body
   and in a refinement row; then with a wrong `result_type`, a wrong member
   and a non-binder second operand.

## Expected Results

- Step 1: the three identities and the `history` member kind are enumerated
  together, `only`'s entry as FR-322 "Operation catalog" states.
- Step 2: both applications are admitted; each malformed shape refuses
  `ill_typed`/`operator-ineligible` at the application node.
