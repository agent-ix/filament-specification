---
id: TC-1720
title: "A map value has one typed canonical form"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-181
    type: verifies
---

# TC-1720: A map value has one typed canonical form

## Description

A map value has one typed canonical form. Scope: FR-181-AC-31.

## Test Procedure

Encode a `Map<String, Int[0, 9]>` value with entries `b` to `2` and `a` to `1` inserted in both orders, compare the state keys of two states holding them, and read a supplied map form whose two entries share the key `a`.

## Expected Results

- Both insertion orders encode to the entries for `a` then `b` in the `map` form, and the two state keys are equal.
- The form with a repeated key is outside the form.
