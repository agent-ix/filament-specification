---
id: TC-2043
title: "A unit with no declaration parses and refuses at check"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-339
    type: verifies
---

# TC-2043: A unit with no declaration parses and refuses at check

## Description

Verify that the complete-V1 parser admits a unit with no declaration and that
the empty-package rule refuses it at check. Scope: FR-339-AC-6.

## Test Procedure

Parse, format, check and compile the header-only unit
`language "ix:native" edition "1-draft";`, and the same header followed by
`profile v = "filament.value.complete/v1";`.

## Expected Results

- Parsing gives no `invalid_syntax` diagnostic, and formatting returns the
  source unchanged.
- Check and compile each refuse `missing_declaration`/`empty-package` at the
  `language` header, exit 20, with no checked package.
