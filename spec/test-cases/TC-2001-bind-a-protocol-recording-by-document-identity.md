---
id: TC-2001
title: "Bind a protocol recording to an analyze request by document identity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-821
    type: verifies
---

# TC-2001: Bind a protocol recording to an analyze request by document identity

## Description

Provision, name and refuse FR-737 recordings in an FR-520 analyze request.
Scope: FR-821-AC-1 to FR-821-AC-4.

## Test Procedure

1. Supply a recording with no `identity` as `{file, identity}` and name it in
   a `global-conformance` item; then supply the same recording declaring that
   identity as `{file}`.
2. Bind the same recording under a second identity.
3. State a malformed identity; state one that differs from the declared one;
   supply an unidentified recording as `{file}`; provision a recording under
   a snapshot's identity.
4. Name no provisioned document in `recording`; name a snapshot; put
   `recording` on a `temporal-satisfaction` item.

## Expected Results

1. Both runs are assessed with equal verdicts, and the result carries the
   provisioned `recording` identity.
2. Equal `closure.premises` and verdicts; a different request identity.
3. Each refuses `invalid-request`, exit 20, at its pointer, assessing no item.
4. Each refuses `invalid-request` at `/items/0/recording`.
