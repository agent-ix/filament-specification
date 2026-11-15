---
id: TC-1003
title: "Indexed read refused outside a hyper body"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1003: Indexed read refused outside a hyper body

## Description

Verify that an indexed read parses in every temporal clause's `holds(...)` and guard, and is refused with a named `unsupported_construct` outside a hyper body over behaviours (FR-501 item 2). Scope: FR-501-AC-2.

## Test Procedure

1. Parse and check the temporal clause of FR-501-AC-2 whose formula reads `v.l @ a`.
2. Parse and check the same clause with the `on each ... when (w.l @ a = 0)` guard.
3. Parse the hyper clause of FR-501-AC-2.

## Expected Results

- Steps 1 and 2: no syntax diagnostic; each refuses `unsupported_construct`/`expression-form` at the indexed read, with the same message, which contains `hyper`.
- Step 3: the clause parses with two indexed reads.
