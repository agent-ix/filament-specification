---
id: TC-1783
title: "Instance recovery summaries are reached and kept apart from outcome labels"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-736
    type: verifies
---

# TC-1783: Instance recovery summaries are reached and kept apart from outcome labels

## Description

Catches a summary that overwrites a registration fact.

Scope: FR-736-AC-3.

## Test Procedure

1. Explore a protocol with two templates and inspect each instance's summary.

## Expected Results

- Step 1: `partially-recovered`, `recovered`, `unrecovered`, `untriggered` and `pending` are reached, and no summary value is admitted as an attempt or aggregate label.
