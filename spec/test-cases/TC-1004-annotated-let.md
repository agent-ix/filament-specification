---
id: TC-1004
title: "Annotated let"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1004: Annotated let

## Description

Verify the annotated `let x: T = e in b` form and its checking at the declared type (FR-501 item 3). Scope: FR-501-AC-3.

## Test Procedure

Parse and check the four bodies of FR-501-AC-3.

## Expected Results

`let x: Int[0,3] = y + 1` over `y: Int[0,2]` checks with the annotation on the binder; over `y: Int[0,3]` it refuses `undefined_expression`/`unproved-range` at the `+`; `let x: Boolean = 1` refuses `ill_typed`/`type-mismatch` at `1`; the unannotated `let` parses with no annotation.
