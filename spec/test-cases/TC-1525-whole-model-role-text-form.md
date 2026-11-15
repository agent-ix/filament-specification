---
id: TC-1525
title: "Whole-model role text form"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-465
    type: verifies
---

# TC-1525: Whole-model role text form

## Description

Exercise the mandatory text form of a role on a whole model. Scope: FR-465-AC-13.

## Test Procedure

Read the package of a protocol with `role w on M` and `role v on M::Cell` through a strict reader.

## Expected Results

Both roles are admitted; the text `model` of `w` is never refused `malformed_wire`.
