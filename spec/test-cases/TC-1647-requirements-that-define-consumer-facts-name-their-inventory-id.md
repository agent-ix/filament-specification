---
id: TC-1647
title: "Requirements that define consumer facts name their inventory id"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-661
    type: verifies
---

# TC-1647: Requirements that define consumer facts name their inventory id

## Description

Inspect requirements against the inventory. Scope: FR-661-AC-4.

## Test Procedure

For each inventory fact, open the requirement it names and check it defines the fact and names the fact's id. Review requirements that define a fact read through a producer's public API for an inventory id.

## Expected Results

- Every inventory fact's requirement defines it and names its id.
- No requirement defines a consumer fact without an inventory id.
