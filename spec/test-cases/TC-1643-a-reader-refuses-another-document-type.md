---
id: TC-1643
title: "A reader refuses another checked-package document type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1643: A reader refuses another checked-package document type

## Description

Verify that a reader admits only its pinned release's document type and that a producer writes it. Scope: FR-660-AC-4.

## Test Procedure

Take a positive fixture of the pinned release from `proposals/checked-package-v2/fixtures/`. Replace `contract_version` with `filament.checked-package/v3` and read it; repeat with `filament.checked-package/v1`. Emit a package with the producer built against the same release.

## Expected Results

- Both documents refuse `unknown_contract_version` with no cause at `/contract_version`, and no package is returned.
- The emitted package's `contract_version` is exactly the release's document type.
