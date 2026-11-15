---
id: TC-1006
title: "Decimal type requires its rounding mode"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1006: Decimal type requires its rounding mode

## Description

Verify that the source `Decimal[..]` type form requires its rounding mode, agreeing with FR-140 (FR-501 item 5). Scope: FR-501-AC-5.

## Test Procedure

Parse `Decimal[0, 9; 0, 2; exact]` and `Decimal[0, 9; 0, 2; nearest-even]` as parameter types, then `Decimal[0, 9; 0, 2]` as a parameter type, a `convert` target and a `Quantity` carrier.

## Expected Results

The two moded forms parse as type references; each mode-less form refuses `invalid_syntax`/`unexpected-token` at its closing `]`.
