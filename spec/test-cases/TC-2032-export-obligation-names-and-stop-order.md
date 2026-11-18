---
id: TC-2032
title: "Export obligation names and stop order"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
---

# TC-2032: Export obligation names and stop order

## Description

Verify that the obligation set must be nonempty, that names resolve to declared obligations, and the order in which the floor and the limits stop an export. Scope: FR-817-AC-8 to FR-817-AC-10.

## Test Procedure

1. Export the counter with `obligations` `[]`, then without the member.
2. Export `["SlowEnough"]` over `Braking`, and `["CanInc", "Bounded", "Nope"]` over the counter.
3. Export `SlowEnough` with `export.output_bytes` 1, the counter's `Bounded` with the same limit, and `SlowEnough` with `export.nodes` 1.

## Expected Results

- Step 1: `invalid-value`, then `missing-member`, both at `/request/export/obligations`, exit 20, with no text.
- Step 2: one `SlowEnough` record. Then three records in request order: `CanInc` and `Nope` `refused` with `not-an-obligation`, `Bounded` `preserved`, exit 21.
- Step 3: `below-fidelity-floor` with exit 21, then `incomplete` on `export.output_bytes` with exit 22, then `incomplete` on `export.nodes` with exit 22.
