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

Scope: FR-740-AC-1 to FR-740-AC-6.

## Test Procedure

1. Check the two timed models and the `x >= 0` variant; the `y <= 3` invariant against a 4-unit wait; a row mapping `y`; an untimed concrete model, and the empty-zone variant.
2. Check the hidden-field `mode` model (FR-740-AC-4), the abstract tick-source model with its liveness half (FR-740-AC-5), and zone certificates with an undelayed failing edge on and off the reached path (FR-740-AC-6).

## Expected Results

- Step 1: Proved with a certificate over both clocks, then refuted with a replaying delay below 1; refuted at the delay crossing 3; the row is refused; the untimed model holds, then is refuted by an empty zone.
- Step 2: Candidates keep separate clock blocks and die separately, abstract time comes only from the abstract tick source and its liveness half is decided, and the undelayed failing edge is accepted or refused as FR-740-AC-4 to FR-740-AC-6 state.
