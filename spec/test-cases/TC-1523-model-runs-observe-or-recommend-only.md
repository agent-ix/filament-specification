---
id: TC-1523
title: "Model runs observe or recommend only"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-182
    type: verifies
---

# TC-1523: Model runs observe or recommend only

## Description

Exercise the observe-or-recommend boundary of model runs. Scope: FR-182-AC-6.

## Test Procedure

Explore and simulate a protocol whose `attempt` writes a model field, recording adapter requests, process starts and connections; inspect the dependencies of the implementation's evaluation and analysis components.

## Expected Results

Only the returned model states change; no adapter request, process or connection occurs; the components have no adapter, process or network dependency.
