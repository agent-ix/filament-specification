---
id: TC-1755
title: "Text outcome by default"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-718
    type: verifies
---

# TC-1755: Text outcome by default

## Description

Verify the default text outcome and its exit parity. Scope: FR-718-AC-1, FR-718-AC-2.

## Test Procedure

1. Run `parse`, `check`, `compile` and `run` over an accepted and a refused input with no `--format` and with `--format json`.
2. Run `run` and `compile` over a missing request path with no `--format`; run a verb with `--format bogus`.

## Expected Results

- Step 1: each text outcome names the category, exit status, item results and diagnostic code, cause and position; the exit statuses are equal.
- Step 2: exit `20` with the I/O failure; `--format bogus` exits `20` naming the option and value before any file is opened.
