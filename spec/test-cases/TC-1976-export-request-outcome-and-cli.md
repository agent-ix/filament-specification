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

1. Run `export --out m.tla` on the counter request.
2. Repeat with `m.tla` present.
3. Export below the fidelity floor.
4. Export with a low `export.output_bytes`, then raise it.
5. Export with a mismatched family.

## Expected Results

- Step 1: the file, `assembled`, exit 0.
- Step 2: refused, exit 20, file unchanged.
- Step 3: exit 21, no file.
- Step 4: `incomplete`, exit 22, then `assembled`.
- Step 5: exit 20, no text.
