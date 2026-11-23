---
id: TC-988
title: "Source-map provenance producer verification and package trust boundary"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-203
    type: verifies
---

# TC-988: Source-map provenance producer verification and package trust boundary

## Description

Verify that source-map provenance is checked while the exact source and
selected definition are available, that the package reader makes only
structural and selection-reference checks, and that consumers bind selected
definitions before source-level use.

## Test Procedure

1. Emit a package from one selected definition and source occurrence with an
   exact region. Repeat with the region one byte too long, outside the source,
   split at a UTF-8 continuation byte, and spanning a different occurrence.
2. Read a package carrying a structurally valid region whose end differs from
   the original source length by one byte. The reader receives only the
   package, retains the region and makes no source-authentication claim.
3. Pass the admitted package to an analyzer or driver with a missing, foreign
   or mismatched selected definition. Attempt a source-level decision using
   the provenance.

## Expected Results

The exact producer input emits; every altered producer region refuses before
emission. The package-only reader accepts the structurally valid region
without representing it as authenticated source content. The consumer refuses
source-level use unless it binds the package selection to its own admitted
definition.
