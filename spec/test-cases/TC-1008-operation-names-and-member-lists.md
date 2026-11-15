---
id: TC-1008
title: "Operation names and member lists"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1008: Operation names and member lists

## Description

Verify the operation name forms and the non-empty `join` and `align skip` lists (FR-501 items 7 and 8). Scope: FR-501-AC-7 and FR-501-AC-8.

## Test Procedure

Parse the state clauses of FR-501-AC-7 and the `parallel` and hyper clauses of FR-501-AC-8.

## Expected Results

The three- and two-segment clauses parse, the two-segment one binding the source object operation or refusing `missing_declaration`/`missing-name` when no `T` is declared, and `on T` refuses `invalid_syntax`/`unexpected-token` at the `{`; `join all [a, b]` and `align skip { V::Vault::mix }` parse; `join all []` refuses at `]` and `align skip {}` at `}`, each `invalid_syntax`/`unexpected-token`.
