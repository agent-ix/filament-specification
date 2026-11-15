---
id: TC-1649
title: "A cut diagnostic list ends with diagnostic-list-cut"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-272
    type: verifies
---
# TC-1649: A cut diagnostic list ends with diagnostic-list-cut

## Description

A cut diagnostic list ends with diagnostic-list-cut.

Scope: FR-272-AC-30.

## Test Procedure

Inspect the two `invalid_syntax` rows; parse a unit of 150 declarations, each with one independent syntax error, under a diagnostic bound of 100.

## Expected Results

- Both rows list `diagnostic-list-cut`.
- 100 syntax diagnostics, then one `invalid_syntax`/`diagnostic-list-cut` at the 101st error naming the bound and its setting; the outcome is a refusal, exit 20.
