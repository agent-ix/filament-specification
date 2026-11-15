---
id: TC-1000
title: "Edition selection admits only 1-draft"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-500
    type: verifies
---

# TC-1000: Edition selection admits only 1-draft

## Description

Verify that the header admits exactly the edition `1-draft` and that every other edition string, `0-draft` included, refuses at the edition string literal (FR-500). Scope: FR-500-AC-1.

## Test Procedure

1. Parse `language "ix:native" edition "1-draft";` followed by `function f(): Boolean pure { true }`.
2. Parse the same unit with the edition strings `"0-draft"`, `"2-draft"` and `""`.

## Expected Results

- Step 1: the unit parses with no diagnostic.
- Step 2: each refuses `unknown_edition`/`unsupported-selection` at byte range 29..38, 29..38 and 29..31 respectively, with no other diagnostic and no checked package.
