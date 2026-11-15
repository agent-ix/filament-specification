---
id: TC-1242
title: "Extension definitions named to a command"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-562
    type: verifies
---

# TC-1242: Extension definitions named to a command

## Description

Verify extension entries named to `check` and their refusals. Scope: FR-562-AC-1, FR-562-AC-2.

## Test Procedure

1. Check FR-562-AC-1's unit with and without `--extensions set.json`.
2. Give an entry with an unknown member, one with no `file`, two entries with one identity, and a definition violating FR-467's `missing-member` rule; check a unit selecting no extension with and without the set.

## Expected Results

- Step 1: exit `0` with the set; `unknown_profile`/`unsupported-selection` at the identity literal without it.
- Step 2: each refusal with the code FR-562 Behavior states, exit `20`; the extension-free unit gives the same result both ways.
