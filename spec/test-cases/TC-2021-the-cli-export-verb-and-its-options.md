---
id: TC-2021
title: "The CLI export verb and its output and evidence options"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-301
    type: verifies
---

# TC-2021: The CLI export verb and its output and evidence options

## Description

Verify that the CLI routes `export`, accepts `--output` and `--evidence` on it, and refuses them together and on other verbs. Scope: FR-301-AC-7. The CLI binary owns option parsing, so the test lives in the driver repository.

## Test Procedure

1. Run `export <request> --output m.tla` over the counter request.
2. Run `export <request> --evidence e.json --output m.tla`, then `export --evidence e.json --output m.tla`.
3. Run `compile <request> --evidence e.json` and `run <request> --output m.tla`.

## Expected Results

- Step 1: exit 0, the file written, an outcome document with operation `export`.
- Step 2: each `invalid-request` naming the options, exit 20, no file.
- Step 3: each an unknown-option refusal, exit 20.
