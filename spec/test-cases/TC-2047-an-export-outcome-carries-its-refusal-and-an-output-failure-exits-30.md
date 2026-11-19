---
id: TC-2047
title: "An export outcome carries its refusal, and an output failure exits 30"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
---

# TC-2047: An export outcome carries its refusal, and an output failure exits 30

## Description

Verify that a `refused` export outcome carries the closed `refusal` member in
its observer and below-floor shapes, that no other status carries it, and
that an export whose target file cannot be written is an output failure,
exit 30. Scope: FR-817-AC-14 and FR-817-AC-15.

## Test Procedure

1. Export `SlowEnough` over `Braking` under `any` with `observer` `tlc` in an
   implementation with no TLC backend; then under the default floor; then an
   assembled and a limit-stopped export.
2. Export under `any` with `output` `{"file": "missing-directory/m.tla"}`;
   then with `m.tla` already present.

## Expected Results

- Step 1: the observer `refusal` shape with exactly its six members, exit
  21; the below-floor shape with `failing` the failing record, exit 21; no
  `refusal` member on the other two.
- Step 2: exit 30, internal-failure, one `output-failure` diagnostic,
  `last_stage` the stage reached, nothing created; then `invalid-request`,
  exit 20.
