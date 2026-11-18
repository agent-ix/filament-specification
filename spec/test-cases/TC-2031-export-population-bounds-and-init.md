---
id: TC-2031
title: "Population bounds and Init come from declared source"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-812
    type: verifies
---

# TC-2031: Population bounds and Init come from declared source

## Description

Verify that population bounds come only from `bound` declarations, and that `Init` states the snapshot, the hybrid initial entries or the well-typedness. Scope: FR-812-AC-6 and FR-812-AC-7.

## Test Procedure

1. Export the `Account` model with `bound B on M::accounts [3]`, then without it.
2. Export a state model with a `constraint` clause and no `initial`.
3. Export `Braking`, which has no `hybrid-initial`, with no `initial`.
4. Export the counter with `initial` naming its snapshot.

## Expected Results

- Step 1: `Cardinality(DOMAIN v_Account) <= 3` and `v_Account.length <= 3`, and neither conjunct without the bound.
- Step 2: `Init == TypeOK` and `def Init (s : State) : Prop := WF s`, with no constraint conjunct.
- Step 3: one `Init` disjunct, mode `Moving` with `TRUE`.
- Step 4: `Init` states the snapshot's state.
