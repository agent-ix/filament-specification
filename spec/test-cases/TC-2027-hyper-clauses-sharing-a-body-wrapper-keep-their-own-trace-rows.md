---
id: TC-2027
title: "Hyper clauses sharing a body wrapper keep their own trace rows"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2027: Hyper clauses sharing a body wrapper keep their own trace rows

## Description

Check that two hyper clauses whose bodies are equal and whose quantifier
prefixes differ share one body wrapper and each carry their own per-trace
`temporal_bindings` rows, keyed by their own `hyperproperty` node in their
own quantifier order.

Scope: FR-322-AC-463.

## Test Procedure

1. Compile, under event-position, `Fwd` over `forall trace s of S forall trace n of N` and `Rev` over `forall trace n of N forall trace s of S` with equal bodies; read the graph and `temporal_bindings`, then read the package.
2. Reorder `Rev`'s two rows to `s` then `n` and read the package.
3. Compile a clause with a third quantifier `forall trace m of N` read only by its `match` block; read its rows; remove the `m` row and read the package.

## Expected Results

- Step 1: two `hyperproperty` nodes whose `body` references one wrapper; `Fwd`'s rows `s`, `n` and `Rev`'s rows `n`, `s`, each keyed by its own node; no row names the wrapper; the package is admitted.
- Step 2: `invalid_package`/`timed-node-reference` at the later row's `trace`.
- Step 3: three rows `s`, `n`, `m`; without `m`, `invalid_package`/`timed-node-reference` at that node's `/semantic_graph/nodes/{N}/body`.
