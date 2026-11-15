---
id: TC-1521
title: "Decision reads the protocol input through a capture"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-053
    type: verifies
---

# TC-1521: Decision reads the protocol input through a capture

## Description

Exercise the rule that a decision reads the protocol input only through a protocol capture, and the locus of a direct read's refusal. Scope: FR-053-AC-5.

## Test Procedure

Link a choice whose guards read `capture ready: Boolean = input.ready`; then the same choice with guards reading `input.ready` directly.

## Expected Results

The capture form is admitted without a `visible` entry; each direct read refuses `ill_typed`/`operator-ineligible` located at the term `input.ready`, naming the guard slot, and never at the guard block, the choice or the protocol declaration.
