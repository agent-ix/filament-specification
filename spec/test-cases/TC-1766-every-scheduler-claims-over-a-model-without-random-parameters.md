---
id: TC-1766
title: "Every-scheduler claims over a model without random parameters"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-724
    type: verifies
---

# TC-1766: Every-scheduler claims over a model without random parameters

## Description

Decides a probability claim under every scheduler over TC-352's `Link` with
and without its random declarations, and the workload step probabilities
without them.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by
the requirement and compare the complete typed result (value, cause and
payload) with the expected result. `Link` and `Even` are TC-352's models;
`Every` is `probabilistic Every using inf on Link under every scheduler on origin { probability >= 0.9 [ eventually holds(m.delivered) ] }`.

## Vectors

| Vector | Input | Expected result | AC |
| --- | --- | --- | --- |
| NR-01 | `Every` over `Link` with both random declarations removed | Minimum 0, maximum 1; fails; counterexample: two sends with `lost = true`, never delivered | FR-724-AC-1 |
| NR-02 | `Every` over `Link` with both random declarations | Minimum `24/25`; holds | FR-724-AC-2 |
| NR-03 | Step probabilities at the initial state under `Even`, random declarations removed | `1/4` for each of `send_a(true)`, `send_a(false)`, `send_b(true)`, `send_b(false)`; sum 1 | FR-724-AC-3 |

## Expected Results

Each value is the exact rational stated; removing the random declarations
makes the arguments scheduler choices and the claim fails rather than being
refused.
