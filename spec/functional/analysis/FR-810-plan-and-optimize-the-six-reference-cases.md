---
id: FR-810
title: "Plan and optimize the six reference planning cases"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-800
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-802
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-803
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-804
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-808
    type: depends_on
---

# FR-810: Plan and optimize the six reference planning cases

## Description

When the planner runs on each of the six reference planning cases below, it
SHALL return the stated result, plan and exact objective value, and the
native evaluator SHALL accept every state of the returned plan against every
rule of the case. The cases are the conformance set for planning: plans
within policy (O1), cost-optimal dispatch (O2), makespan scheduling (O3),
throughput maximization (O4), multi-agent routing (O5) and cost-optimal
capacity (O6).

## Inputs

Each case's model, initial snapshot, `synthesis` declaration and policy
clauses, as described in its row.

## Outputs

One `filament.plan-outcome/v1` document per case.

## Behavior

Each case SHALL be published as a runnable example (a source unit, a model
and a snapshot) whose plan request reproduces the stated outcome.

The standard SHALL author and publish these fixtures as its own content,
because they are the conformance set every implementation is held to:
one directory per case under `proposals/planning-v1/fixtures/`, named
`o1-rollout`, `o2-battery-dispatch`, `o3-production-schedule`,
`o4-staffing`, `o5-robot-routing` and `o6-autoscaling`, each holding
`unit.native` (the `synthesis` declaration, the policy clauses and the
types), `model.semantic-ir.json`, `snapshot.json` (a
`filament.state.snapshot/v1` initial state) and `plan.json` (the FR-805
plan request members other than `package`, with `initial` naming
`snapshot.json`). The same directory holds the fixtures the other planning
requirements name: `counter` (FR-650 "The counter fixture"), `timed-go`
and `timed-tick` (FR-808), `probabilistic-try` (FR-809) and `tank`
(FR-819); `tank` holds only `unit.native` and `plan.json`, because its hybrid
declaration carries its initial set and a hybrid plan reads no model or
snapshot, and its `plan.json` carries the `dwell_grid`. They are published files of the standard's package, read by
reference; an implementation holds no copy of them and keeps only its own
host-surface wrapping (the request envelope that points at them). Each case
names the planning feature it exercises. In every case the returned plan
replays natively and every state satisfies the model rules and the policies.

| Case | Model | Rules and objective | Feature exercised |
| --- | --- | --- | --- |
| O1 rollout within policy | servers `n1` to `n5`, each `serving`, `drained` or `upgraded`, all on the old version; `drain(s)` (serving, old → drained), `upgrade(s)` (drained → upgraded), `rejoin(s)` (upgraded → serving, new) | policy `KeepServing`: `always holds(size(serving) >= 3)`; goal: all five new and serving; minimize `length`; `domain` `Int[0, 15]` | safety policy pruning, reference arguments, canonical tie order |
| O2 battery dispatch | four hours, load 1 kWh each, battery 4 kWh at `soc` 0.50; `charge` (grid draw 2, `soc` + 0.25), `idle` (draw 1), `discharge` (draw 0, `soc` − 0.25); price 0.10 in hours 0 and 1 and 0.40 in hours 2 and 3 | invariant `soc >= 0.20`; policy `always` ramp `abs(draw − previous draw) <= 1` from an initial draw of 1; goal `hour = 4`; minimize the per-step sum of price × draw | exact decimal per-step cost, potential certificate, step-reading policy |
| O3 production schedule | jobs `A` and `B` of 3 ticks each, machines `M1` and `M2`, `time` tick of 10 min; `start(j, m)`, `tick`; `M2` under maintenance in ticks 2 and 3 | invariant `NoWorkInMaintenance`; goal: both jobs done; minimize `elapsed` | timed (tick) planning, `elapsed` makespan |
| O4 staffing | crew of 8 at stations `process`, `pack`, `ship` (declared in that order) with 3, 2, 3 people; `move(from, to)` moves one person; throughput `min(10 × process, 6 × pack, 10 × ship)` | invariant: every station keeps at least 1 person and the crew stays 8; maximize `final.throughput`; `domain` `Int[0, 3]` | general objective over `final`, closure certificate, length tie rule |
| O5 robot routing | corridor cells `c0` to `c4` with a bay `b2` beside `c2` (cells declared `c0` … `c4`, `b2`); robot `a` at `c0` bound for `c4`, robot `b` at `c4` bound for `c0`; `move(r, c)` moves one robot to an adjacent free cell | invariant `NoSharedCell`; policy `eventually[0, 12] holds(allAtTarget)`; minimize `length` | multi-agent plan, bounded-eventually policy on the terminal stutter |
| O6 autoscaling | four hours with load 2, 2, 5, 3; `set(r)` with `r` in `Int[3, 8]` sets the hour's replicas and advances the hour; capacity equals replicas | invariants `capacity >= load × 1.2` and `replicas >= 3`; goal `hour = 4`; minimize the per-step sum of `r × 0.05` | exact decimal comparison in a rule, argument-domain enumeration |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-810-AC-1 | O1 returns `optimal` at length 15 with the plan `drain n1, drain n2, upgrade n1, upgrade n2, rejoin n1, drain n3, upgrade n3, rejoin n2, drain n4, upgrade n4, rejoin n3, drain n5, upgrade n5, rejoin n4, rejoin n5`, at least 3 servers serving in every state, and a `potential` certificate. | Test (TC-1960) |
| FR-810-AC-2 | O2 returns `optimal` with the plan `charge, idle, discharge, discharge` and objective exactly `0.30`, every state with `soc >= 0.25`; without the `soc` invariant it returns `idle, idle, discharge, discharge` at `0.20`, whose final `soc` is `0.00`. | Test (TC-1961) |
| FR-810-AC-3 | O3 returns `optimal` with `elapsed` exactly `60 min` and the plan `start(A, M1), tick, tick, tick, start(B, M1), tick, tick, tick`; without `NoWorkInMaintenance` it returns `start(A, M1), start(B, M2), tick, tick, tick` at `30 min`. | Test (TC-1962) |
| FR-810-AC-4 | O4 returns `optimal` with objective 20, the plan `move(process, pack), move(ship, pack)` (staff 2, 4, 2), and an accepted `closure` certificate. | Test (TC-1963) |
| FR-810-AC-5 | O5 returns `optimal` at length 10 with the plan `move(a, c1), move(a, c2), move(a, b2), move(b, c3), move(b, c2), move(b, c1), move(a, c2), move(a, c3), move(a, c4), move(b, c0)`, no state with both robots in one cell; with the policy bound tightened to `eventually[0, 9]` it returns `infeasible`. | Test (TC-1964) |
| FR-810-AC-6 | O6 returns `optimal` with the plan `set(3), set(3), set(6), set(4)` and objective exactly `0.80`, the comparison `6 >= 5 × 1.2` holding exactly. | Test (TC-1965) |
| FR-810-AC-7 | The standard's package publishes `unit.native`, `model.semantic-ir.json`, `snapshot.json` and `plan.json` under `proposals/planning-v1/fixtures/<case>/` for each of the six cases and for `counter`, `timed-go`, `timed-tick` and `probabilistic-try`, and `unit.native` and `plan.json` for `tank`; each `unit.native` checks at S3 against its `model.semantic-ir.json` (`tank`'s on its own), each `snapshot.json` admits for that model, and each `plan.json` is admitted by FR-805's reader. | Test (TC-2022) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) and FR-800 to
  FR-808 own every behaviour these cases exercise.
