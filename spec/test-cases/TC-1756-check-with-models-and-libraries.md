---
id: TC-1756
title: "Check with supplied models and libraries"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-719
    type: verifies
---

# TC-1756: Check with supplied models and libraries

## Description

Verify `check --model` and `--library`. Scope: FR-719-AC-1, FR-719-AC-2.

## Test Procedure

1. Run `check` over the model-selecting unit with and without `--model`.
2. Run it with a repeated operand, an operand with no `=`, a `--model` for a unit with no `model` declaration, and an unimported `--library`.

## Expected Results

- Step 1: exit `0`; without the option exit `20` `missing_import` naming the identity and `--model`.
- Step 2: each exits `20` naming the option.
