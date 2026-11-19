---
id: TC-1563
title: "Probabilistic-model nodes in the checked package"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-640 FR-322
    type: verifies
---

# TC-1563: Probabilistic-model nodes in the checked package

## Description

Verify emission, reading, identity and reader refusals of the `probabilistic` nodes.

Scope: FR-640-AC-1 to FR-640-AC-5, FR-640-AC-7 and FR-322-AC-471.

## Test Procedure

1. Emit and read `Service` (FR-640-AC-1).
2. Emit and read the four delay families (FR-640-AC-2).
3. Reorder `Steady`'s weights; change one weight, one support value and one reward expression; emit a workload with a weight 0 (FR-640-AC-3).
4. Apply each reader mutation of FR-640-AC-4, and a reader budget one visit short.
5. Resolve `under Steady` and run a provider over the emitted package (FR-640-AC-5).
6. Emit the three-reward unit of FR-640-AC-3 in the three declaration orders and with `load`'s expression changed (FR-640-AC-3).
7. Read `Service`'s reward function clause kind and the `Server::attempt` precondition and `operation_bodies` reads; apply the FR-640-AC-7 and FR-322-AC-471 mutations (owner moved to `Server`, to a field member of `Server` and to `Server/reset`; `clause` rewritten to `"precondition"` and to `"body"`) (FR-640-AC-7, FR-322-AC-471).

## Expected Results

Each step gives the nodes, typed reads, identities, refusals (code, cause and JSON pointer) and step probabilities its AC states.
