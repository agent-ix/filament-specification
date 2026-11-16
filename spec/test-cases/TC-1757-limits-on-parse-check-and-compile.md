---
id: TC-1757
title: "Limits on parse, check and compile"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-720
    type: verifies
---

# TC-1757: Limits on parse, check and compile

## Description

Verify `--limit` and the byte ceilings on `parse`, `check` and `compile`. Scope: FR-720-AC-1, FR-720-AC-2.

## Test Procedure

1. Run `parse` over a 6,000-deep nest at the defaults, then `parse`, `check` and `compile` with the node setting raised.
2. Give `check` an evaluation-budget setting and `parse` a setting of a stage it does not run; compile a request over its request-file ceiling, then with the ceiling raised.

## Expected Results

- Step 1: exit `22` naming the node setting, then exit `0` for each verb.
- Step 2: exit `20` naming the setting and the accepted settings before any file is read; the compile refusal names the option, and the raised ceiling compiles.
