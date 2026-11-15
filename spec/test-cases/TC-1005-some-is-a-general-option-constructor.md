---
id: TC-1005
title: "Some is a general option constructor"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1005: Some is a general option constructor

## Description

Verify that `Some(e)` constructs a present option in every expression position (FR-501 item 4). Scope: FR-501-AC-4.

## Test Procedure

Check and evaluate the functions of FR-501-AC-4, the two wrong-arity calls and a refinement row using `Some(e)`.

## Expected Results

`s` returns the present option holding `b`; `n` checks with `Some(b)` typed `Option<Integer>`; `Some(b, b)` and `Some()` refuse `ill_typed`/`type-mismatch` at the call; the refinement row keeps its meaning.
