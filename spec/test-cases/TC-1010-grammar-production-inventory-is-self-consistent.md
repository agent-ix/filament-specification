---
id: TC-1010
title: "Grammar production inventory is self-consistent"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-503
    type: verifies
---

# TC-1010: Grammar production inventory is self-consistent

## Description

Verify that the shared grammar's production inventory can be read from its `ebnf` blocks and is closed under use (FR-503). Scope: FR-503-AC-1 and FR-503-AC-2.

## Test Procedure

Read every fenced `ebnf` block of `proposals/filament-v1/shared-grammar.md`; collect each defined name with `=` or `=/`, then every right-hand-side name.

## Expected Results

The set is non-empty, every `=/` name is defined earlier with `=`, and every right-hand-side name is defined or is one of `ident`, `member-name`, `string`, `uint`, `decimal-literal` and `EOF`. The `decimal-literal` terminal matches `[0-9]+ '.' [0-9]+` and rejects exponent forms. The inventory includes definitions for `step-target` and `step-argument`; any other undefined name is reported with its line.
