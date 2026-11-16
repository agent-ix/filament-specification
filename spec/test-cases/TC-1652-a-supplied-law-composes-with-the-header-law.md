---
id: TC-1652
title: "A supplied law composes with the header law"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-570
    type: verifies
---
# TC-1652: A supplied law composes with the header law

## Description

A law name a command or request supplies selects the law of a unit with no
law header, conflicts with a different header law, and never reaches a
library.

Scope: FR-570-AC-6.

## Test Procedure

1. Compile a complete-value unit with `a div b` and no law header, supplying `floor`; evaluate `-7 div 3`.
2. Compile the unit with a `floor` header, supplying `truncating`.
3. Supply `ceiling`.
4. Compile a request whose program supplies `floor` and whose library uses `div` with no header and no law of its own.

## Expected Results

- Step 1: admitted; `-3`.
- Step 2: `invalid_package`/`conflicting-definition` at the header's identity literal.
- Step 3: `unknown_profile`/`unsupported-selection`; no source read.
- Step 4: `missing_declaration`/`missing-selection` at the library's `div`.
