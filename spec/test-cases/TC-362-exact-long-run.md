---
id: TC-362
title: "Exact long-run fractions"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-411
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-412
    type: verifies
---

# TC-362: Exact long-run fractions

## Description

Compute long-run fractions by bottom components and check the gain–bias certificate.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the FSL design record each one comes from. `ev` selects `filament.temporal.event-position.false-extension/v1`, `inf` selects `filament.temporal.infinite-trace/v1` and `timed` selects `filament.temporal.timed/v1`.

- **`Avail`**, **`LongRun`** and **`Monthly`** (from FSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= rational(999, 1000) ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| LR-01 | `LongRun` | `proved`, `exact-value{1800/1801}` with a `long-run` certificate: gain `1800/1801`, bias 0 at `up` and `−2000/1801` at `down` |
| LR-02 | The checker on LR-01's certificate | Accepts after checking both multichain equations by exact equality |
| LR-03 | A weighted long-run claim whose bottom component has every cycle of total weight 0 | the component's unweighted fraction, settled as a verdict with its unweighted gain–bias certificate; never `zero-weight-component` |
| LR-04 | A `maximum` `long-run` certificate with per-component `policy` rows, then with one row removed, one row for a foreign state, one duplicate, one non-action choice, and one bias changed by `1/1000`; the same certificate under `workload` with and without `policy` | Accepted; `malformed` at `{"member": "policy"}` for the four policy defects; `inequality-fails` at the state for the bias; under `workload` `malformed` with a `policy` and accepted without (FR-412-AC-26) |
| LR-05 | `Every`, no fairness set, `>= 3/4`, refuted by deterministic scheduler rows inducing `1/2`; a witness inducing `3/4`; a randomised row; a row naming no action; a missing reachable row | Accepted; `inequality-fails` at the initial state; `malformed` at `{"member": "policy"}` for the last three (FR-412-AC-27) |
| LR-06 | A model with two components of gains `1/2` and `3/4` chosen by a scheduler: `>= 3/4` and `<= 3/4` under `Every`, and under a fairness set excluding neither | `refuted` with the witness; `proved` with component policies; same under the fairness set; never `unsupported` (FR-411-AC-18) |

## Expected Results

Values and certificate checks are exactly as stated.
