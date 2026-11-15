---
id: TC-1324
title: "Scale a quantity by an exact number"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-142-AC-41 FR-142-AC-42
    type: verifies
---

# TC-1324: Scale a quantity by an exact number

## Description

Verify FR-142 quantity scaling: `q * n`, `n * q` and `q / n` keep the unit `u`, `q / n` carries the nonzero obligation, and the refused forms.

## Test Procedure

Declare the dimension `T` with root `s` and `ms` (scale `1/1000`). Check and evaluate the admitted expressions of FR-142-AC-41 at `d = 3 ms`, `k = 4`, recording values, charges and the emitted operations; then check each refused form of FR-142-AC-42.

## Expected Results

The admitted expressions are typed `Quantity<ms>` with the values FR-142-AC-41 lists, charge one `unit.identity-read`, one `unit.rational-arithmetic` and `unit.result-retain`, and lower to `filament.op.quantity.scale` or `scale_div`; the refused forms give the codes and loci FR-142-AC-42 lists.
