---
id: TC-2034
title: "A body wrapper with no slot ranges over the model root"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-402
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-370
    type: verifies
---

# TC-2034: A body wrapper with no slot ranges over the model root

## Description

Verify that a hyper clause whose body reads no slot parameter has a body wrapper whose `over` is the `model_root` node of its first subject, and that the reader refuses a model-root `over` everywhere else. Scope: FR-402-AC-11.

## Test Procedure

1. Read the package of `Whole` (FR-402-AC-11).
2. Edit `NonInterference`'s wrapper `over` to the vault `model_root` node and read.
3. Edit `Whole`'s wrapper `over` to the root of a selected package no `subjects` binding names and read.
4. Edit a single-trace clause's `over` to a `model_root` node and read.

## Expected Results

- Step 1: admitted; the wrapper's `over` is the vault root, among its `dependencies`.
- Steps 2 to 4: each `invalid_model_binding`/`malformed-declaration` at `/semantic_graph/nodes/{w}/body/arguments/0`.
