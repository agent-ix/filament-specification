---
id: TC-1782
title: "Exploration reaches every per-attempt outcome and aggregate fact"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-736
    type: verifies
---

# TC-1782: Exploration reaches every per-attempt outcome and aggregate fact

## Description

Catches an explorer with no failure, timeout or expiry step.

Scope: FR-736-AC-1, FR-736-AC-2.

## Test Procedure

1. Explore TC-380's `Pay` with `attempts 2`, and its variant with no activation.

## Expected Results

- Step 1: Attempts `failed`, `timed-out` and `successful` are reached, with a retry keeping its predecessor's label and never three attempts; aggregate facts `pending`, `successful`, `failed` and `timed-out` are reached, and the untriggered registration has no fact.
