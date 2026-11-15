---
id: TC-1164
title: "Timed clock rows for trace subjects"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-540
    type: verifies
---

# TC-1164: Timed clock rows for trace subjects

## Description

Check that a timed clause over a trace subject carries exactly one closed
`timed {unit}` row naming its clock role, and that a timed clause over a
timed-model subject keeps the no-row rule and the clock name `model-time`
(FR-322-AC-450).

## Test Procedure

Validate the `timed` alternative against `ClockConfiguration` in
`proposals/checked-package-v2/schema.json` (with `unit`, without it, with an
empty `unit`, with an extra member), then decide each case of
`proposals/checked-package-v2/timed-clock-no-row-vectors.json` (TN-01 to TN-09)
by the FR-322 "Temporal clock bindings" rules.

## Expected Results

- The `timed` alternative is admitted only closed with a non-empty `unit`.
- TN-01 and TN-06 are admitted; TN-02, TN-03 refuse at the clause body and
  TN-04, TN-07, TN-08, TN-09 at `/temporal_bindings/0`, each
  `invalid_package`/`timed-node-reference`.
