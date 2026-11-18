---
id: TC-2009
title: "An analyze package selection names a handoff directory"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
---

# TC-2009: An analyze package selection names a handoff directory

## Description

Verify that a `native-analyze/1` request's `package` member is the closed
`{directory}` selection of a checked-package handoff directory, that the
directory's identity file is required and its equality file optional, and
that the earlier `{file}` spelling is refused. Scope: FR-520-AC-7.

## Test Procedure

1. Read a request whose `package` is `{"directory": "handoff"}` over a
   handoff directory holding `checked-package.json` and `identities.json`.
2. Read the same request with `package` `{"file": "package.json"}`.
3. Remove `identities.json` from the directory and read the request.
4. Replace `identities.json` with `{}` and read the request.
5. Restore it, remove `equality.json`, and read the request; then write
   `not json` as `equality.json`.

## Expected Results

- Step 1: admitted.
- Step 2: `invalid-request` naming `file`.
- Step 3: `invalid_package`, `missing-file`, naming `identities.json`.
- Step 4: `invalid_package`, `malformed-file`.
- Step 5: admitted; then `invalid_package`, `refused-file`.
