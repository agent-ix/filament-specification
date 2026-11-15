---
id: TC-1243
title: "Check rules in a Markdown specification"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-563
    type: verifies
---

# TC-1243: Check rules in a Markdown specification

## Description

Verify `check --markdown` and the default-build `run` extraction. Scope: FR-563-AC-1, FR-563-AC-2.

## Test Procedure

1. Run `check --markdown agent-ix doc spec.md --format json` over FR-563-AC-1's document.
2. Run a clause-run request selecting `Healthy` through `program.extraction`; run `check` over the Markdown file without `--markdown`.

## Expected Results

- Step 1: two items naming their units as FR-563 states, the second `ill_typed` with the Markdown line and column; exit `20`.
- Step 2: the run writes its outcome document; the plain `check` refuses `invalid_syntax`.
