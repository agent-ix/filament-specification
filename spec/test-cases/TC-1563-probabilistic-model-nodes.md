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

Verify emission, reading, identity and reader refusals of the `probabilistic` nodes and of the `probabilistic_claim` node that carries what a claim reads and what it states.

Scope: FR-640-AC-1 to FR-640-AC-5, FR-640-AC-7 to FR-640-AC-11, FR-322-AC-471, FR-322-AC-472 and FR-322-AC-474.

## Test Procedure

1. Emit and read `Service` (FR-640-AC-1).
2. Emit and read the four delay families (FR-640-AC-2).
3. Reorder `Steady`'s weights; change one weight, one support value and one reward expression; emit a workload with a weight 0 (FR-640-AC-3).
4. Apply each reader mutation of FR-640-AC-4, and a reader budget one visit short.
5. Resolve `under Steady` and run a provider over the emitted package (FR-640-AC-5).
6. Emit the three-reward unit of FR-640-AC-3 in the three declaration orders and with `load`'s expression changed (FR-640-AC-3).
7. Read `Service`'s reward function clause kind and the `Server::attempt` precondition and `operation_bodies` reads; apply the FR-640-AC-7 and FR-322-AC-471 mutations (owner moved to `Server`, to a field member of `Server` and to `Server/reset`; `clause` rewritten to `"precondition"` and to `"body"`); emit FR-640-AC-6's unit with the reward `cost` on `Other::poke` returning `k`, read its function's owner, `declaration`, ordinal and `clause`, and read the package with that function's `declaration` rewritten to `["Other", "poke", "reward", "load"]` (FR-640-AC-7, FR-322-AC-471).
8. Emit FR-640-AC-8's unit with its four claims and read each claim node's body, path and typed read, and the `whole`/`each` variant of `Fair`; apply each change of FR-640-AC-3 to it and compare every claim `node_id` and `probabilistic_claims` row with the unchanged emission; emit the unit extended with `Avail` and `Calm` and apply each reader mutation of FR-640-AC-9 to it, and the budget one visit short to FR-640-AC-8's package; emit FR-322-AC-472's two claims named `A`, reverse their rows and empty a claim body, and swap the path operand in its one-claim package (FR-640-AC-3, FR-640-AC-8, FR-640-AC-9, FR-322-AC-472).
9. Read `P95`'s, `Avail`'s and `Cost`'s bindings 6 to 9 and `P95`'s path in FR-640-AC-8's package; apply each identity change of FR-640-AC-10 and compare every claim `node_id` and `probabilistic_claims` row with the unchanged emission; apply each spelling of FR-640-AC-10 and compare `node_id`s and bodies; emit its two claim pairs named `A`; apply each reader mutation of FR-640-AC-11 to FR-640-AC-9's package and rewrite the threshold segment of `P95`'s path in its one-claim package; emit FR-322-AC-474's pairs, reverse their rows, and spell a `bound` `value` `2/4` (FR-640-AC-10, FR-640-AC-11, FR-322-AC-474).

## Expected Results

Each step gives the nodes, typed reads, identities, refusals (code, cause and JSON pointer) and step probabilities its AC states.
