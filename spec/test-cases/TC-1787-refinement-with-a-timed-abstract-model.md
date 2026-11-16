---
id: TC-1787
title: "A refinement with a timed abstract model is decided on the product with derived clocks"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-740
    type: verifies
---

# TC-1787: A refinement with a timed abstract model is decided on the product with derived clocks

## Description

Catches a refusal of the timed abstract side, or abstract clocks treated as free.

Scope: FR-740-AC-1, FR-740-AC-2, FR-740-AC-3.

## Test Procedure

1. Check the two timed models and the `x >= 0` variant; the `y <= 3` invariant against a 4-unit wait; a row mapping `y`; an untimed concrete model, and the empty-zone variant.

## Expected Results

- Step 1: Proved with a certificate over both clocks, then refuted with a replaying delay below 1; refuted at the delay crossing 3; the row is refused; the untimed model holds, then is refuted by an empty zone.
