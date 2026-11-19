---
id: TC-2052
title: "A zone certificate carries its state-graph part"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-418
    type: verifies
---
# TC-2052: A zone certificate carries its state-graph part

## Description

Verify that the `filament.backend-provider/v1` schema admits the zone
certificate's `state_graph` part of an `always possible` proof and refuses a
malformed one. Scope: FR-418-AC-17.

## Test Procedure

1. Validate FR-712-AC-2's `always possible` proof certificate carrying
   `"state_graph": {"form": "always-possible", "ef_nodes": [0, 2]}`.
2. Validate it with `form` `possible`, with an extra member in `state_graph`,
   and without `ef_nodes`.
3. Validate a zone certificate carrying no `state_graph`.

## Expected Results

- Step 1: admitted.
- Step 2: each fails the schema.
- Step 3: admitted.
