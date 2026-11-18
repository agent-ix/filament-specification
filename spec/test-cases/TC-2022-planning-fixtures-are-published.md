---
id: TC-2022
title: "The planning fixtures are published and admitted"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-810
    type: verifies
---

# TC-2022: The planning fixtures are published and admitted

## Description

Verify that the standard publishes every planning fixture and that each one checks, admits and reads. Scope: FR-810-AC-7.

## Test Procedure

1. List the published files under `proposals/planning-v1/fixtures/` for the six cases, `counter`, `timed-go`, `timed-tick`, `probabilistic-try` and `tank`.
2. Check each `unit.native` at S3 against its `model.semantic-ir.json`.
3. Admit each `snapshot.json` for its model and read each `plan.json`, wrapped as the `synthesis` member of a host request over its checked unit, with FR-805's reader.

## Expected Results

- Step 1: the four files in each of the ten model directories; `unit.native` and `plan.json` in `tank`.
- Step 2: every unit checks with no refusal.
- Step 3: every snapshot admits and every wrapped plan request is admitted.
