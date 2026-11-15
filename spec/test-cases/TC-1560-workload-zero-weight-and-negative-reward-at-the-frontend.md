---
id: TC-1560
title: "Workload weight 0 and negative rewards at the frontend"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-405
    type: verifies
---

# TC-1560: Workload weight 0 and negative rewards at the frontend

## Description

Verify that a workload admits weight 0 and refuses only negative weights, and that the frontend, not a provider, refuses a reward that can be negative.

Scope: FR-405-AC-10, FR-405-AC-11.

## Test Procedure

Run each vector through the real frontend boundary and compare the complete typed result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| WZ-01 | `workload Steady on Service` with `Server::reset: 0`, weight 1 on `Server::request` and `Server::attempt` | Admitted; weight 0 recorded for `Server::reset` |
| WZ-02 | `Steady` with weight 0 on every operation | Admitted |
| WZ-03 | `Steady` with `Server::reset: rational(-1, 1)` | `invalid_probabilistic_model`/`invalid-weight` naming `Steady` and `Server::reset` |
| WZ-04 | `Steady` with `Server::reset` omitted | `invalid_probabilistic_model`/`incomplete-workload` |
| NR-01 | `reward refund on Shop::buy = n - 3`, `n: Int[0, 5]` | `invalid_probabilistic_model`/`negative-reward` naming `Shop::buy`, `refund`, `n = 0`; no checked package, no provider call |
| NR-02 | `reward refund on Shop::buy = n` | Admitted |

## Expected Results

Each vector gives exactly its expected result.
