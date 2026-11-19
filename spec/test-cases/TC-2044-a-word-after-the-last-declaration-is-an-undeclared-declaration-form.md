---
id: TC-2044
title: "A word after the last declaration is an undeclared declaration form"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-339
    type: verifies
---

# TC-2044: A word after the last declaration is an undeclared declaration form

## Description

Verify that a word after the last complete declaration refuses as an
undeclared top-level declaration form, and that a token that begins no
declaration refuses as a syntax error. Scope: FR-339-AC-7.

## Test Procedure

Beside `profile v = "filament.value.complete/v1";`, check
`function probe using v(): Boolean pure { true } garbage`, then
`function probe using v(): Boolean pure { true } }`.

## Expected Results

- The first refuses `unsupported_construct`/`declaration-form` naming
  `garbage` at its span, exit 21, with no checked package and no
  `invalid_syntax`.
- The second refuses `invalid_syntax`/`unexpected-token` at the trailing `}`,
  exit 20.
