---
id: TC-1160
title: "Clock selection by clock role"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-540
    type: verifies
---

# TC-1160: Clock selection by clock role

## Description

Bind trace-subject declarations to clock configurations through a selection keyed by clock role, and refuse missing, unused and mismatched entries (FR-540-AC-1 to AC-3).

## Test Procedure

| Vector | Input | Expected | AC |
| --- | --- | --- | --- |
| CS-01 | Event-position, fixed-sample and timestamped-event clauses over a trace subject on roles `events`, `sample-clock`, `plant-ms`, one entry per role; a second clause on `events` | One binding row per clause with its role and configuration; both `events` rows equal | FR-540-AC-1 |
| CS-02 | CS-01 without `plant-ms`; with an extra `spare`; with a timestamp unit for `events` | `missing-clock-selection`, `unused-clock-selection`, `clock-kind-mismatch`, each naming the role, before any package | FR-540-AC-2 |
| CS-03 | An event-position clause over a model subject, no entry | Binds the model's step sequence; no refusal | FR-540-AC-2 |
| CS-04 | CS-01 with another `events` sequence authority; CS-01 with entries reordered | Different binding identity; equal identity | FR-540-AC-3 |

## Expected Results

Each vector's complete typed result (rows, refusal cause, named role and locus) equals the expected column.
