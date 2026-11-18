---
id: TC-2020
title: "Export package key and the export evidence verb"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
---

# TC-2020: Export package key and the export evidence verb

## Description

Verify the shape and determinism of an export outcome's `package` member and the `export --evidence` verb's outcome and option refusal. Scope: FR-817-AC-6 and FR-817-AC-7.

## Test Procedure

1. Export the counter request twice, then once with `limits` `{"export.nodes": 99999}`.
2. Run `export --evidence <request>` over an evidence request naming step 1's `package` and `Bounded`.
3. Run `export --evidence <request> --output m.tla`.

## Expected Results

- Step 1: `package` holds exactly `source_package`, `target_profile`, `generator` and `limits` with `export.output_bytes` 16777216, `export.nodes` 100000 and `export.work_units` 16777216; the first two are equal; the third differs.
- Step 2: an outcome document with operation `export` and one item for `Bounded`.
- Step 3: `invalid-request` naming `--evidence` and `--output`, exit 20, no file.
