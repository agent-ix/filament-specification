---
id: TC-1976
title: "Export request, outcome document and CLI verb"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
---

# TC-1976: Export request, outcome document and CLI verb

## Description

Verify the export verb's file writing, refusals, fidelity exit, limits and family check. Scope: FR-817-AC-1 to FR-817-AC-5.

## Test Procedure

1. Run `export <request> --output m.tla` on the counter request (`inc`, `value <= 5`); then run the request carrying `output` `{"file": "m.tla"}` with no option in a fresh directory.
2. Repeat step 1's command with `m.tla` present; then run the request carrying `output` with `--output` given.
3. Export below the fidelity floor.
4. Export with `limits` `{"export.output_bytes": 1}`; without the entry and with `--limit export.output_bytes=1`; with `--limit export.output_bytes=16777216`; with the entry and the option together.
5. Export with a mismatched family.

## Expected Results

- Step 1: the file, `assembled`, `output` naming `m.tla` with `bytes_written` equal to the text length, exit 0; the two files are byte-identical.
- Step 2: `invalid-request` naming the path, exit 20, file unchanged; then `invalid-request` naming `output` and `--output`, exit 20, no file.
- Step 3: exit 21, no file.
- Step 4: `incomplete` naming `export.output_bytes`, exit 22, twice; then `assembled`; then `invalid-request` naming `limits` and `export.output_bytes`, exit 20.
- Step 5: exit 20, no text.
