---
id: TC-1765
title: "Frontend refusals reach the CLI with their standard cause"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-723
    type: verifies
---

# TC-1765: Frontend refusals reach the CLI with their standard cause

## Description

Runs the CLI `compile` verb on timed-profile probabilistic claims and on a
corpus of frontend refusals, and compares the CLI outcome with the frontend's
library result. Planned; the rows run in the driver repository against the
CLI binary.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| CR-01 | A probabilistic claim `using` the timed profile over a subject whose model declares `time dense unit ms`, event `eventually[0 ms, 4 ms] holds(P)` | Exit `0`, no diagnostic | FR-723-AC-1 |
| CR-02 | The same claim over a subject whose model declares no time binding | One diagnostic, `invalid_timed_model`/`model-time-on-untimed-model` naming the model at the profile alias; no cause outside FR-272's closed set; no cause naming a timed event profile | FR-723-AC-2 |
| CR-03 | Each refusal of TC-353's vectors and of CR-02, through the CLI and through the frontend library | Equal code, cause variant, payload and locus | FR-723-AC-3 |

## Expected Results

The CLI reports exactly the frontend's standard causes and refuses nothing the
frontend admits.
