---
id: FR-801
title: "Bound, meter and complete the plan search"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-800
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-461
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
---

# FR-801: Bound, meter and complete the plan search

## Description

When the planner searches the plan space of FR-800, it SHALL explore only
plans whose length is a value of the declaration's `domain`, SHALL charge
every unit of search and evaluation work to a named limit, and SHALL report
`optimal` or `infeasible` only after a complete exploration of that bounded
space, and `incomplete` naming the limit, its value and its setting
whenever a limit, a deadline or a cancellation stops it first.

## Inputs

The plan space (FR-800); the request's planner limits, deadline and
cancellation; the evaluation meter's `filament.value.accounting/v1` limits.

## Outputs

A complete exploration that FR-650's result and certificates are built
from, or an `incomplete` stop record.

## Behavior

### The bounds

- The plan length SHALL be bounded by `max D`, the greatest value of the
  `domain` type; a plan whose length is not a value of `D` is never a
  result, although its prefixes are explored when a longer length is in `D`.
- Argument domains are finite (FR-800), so each state has finitely many
  steps and the bounded space is finite.

### The planner limits

The planner's limits type SHALL have these members, each a limit under
FR-461 Behavior 5 with the engine name `plan_search`:

| Member | Setting | Charged |
| --- | --- | --- |
| `max_states` | `plan_search.max_states` | one per distinct search node admitted (below) |
| `max_work` | `plan_search.max_work` | one per step applied by the runtime, one per policy-monitor update, one per goal or objective evaluation, one per frontier comparison |
| `max_certificate_entries` | `plan_search.max_certificate_entries` | one per certificate entry the search writes |
| `max_check_work` | `plan_search.max_check_work` | one per certificate entry the core checker reads and one per successor it re-derives |
| `max_state_key_bytes` | `plan_search.max_state_key_bytes` | FR-181's state-key byte limit, per key |

Each member has a published positive default. The settings are distinct
from the verification planner's `planner.max_work` (FR-492): the two
engines share no limit.

- Every expression evaluation the search or the checker performs (the
  model's operations, clauses and invariants, the policy clauses, the goal
  and the objective) SHALL charge the evaluation meter, whose limits are the
  ten `ScalarLimitsV1` counters with the settings `accounting.<counter>`
  (FR-461 Behavior 6). A meter is built fresh for each request.
- A denied charge SHALL stop the search. The stop is `incomplete` with
  cause `limit-reached{limit, value, setting}` naming the member or counter,
  the bound in force and the count the denied charge would have reached.
- A caller cancellation stops it `incomplete` with `cancelled{source:
  requested}`, and the request deadline with `cancelled{source: deadline}`
  (FR-331 incomplete causes).
- An `incomplete` result SHALL carry the explored counts (`states`,
  `work`) and, when an admissible plan was found, the best one found so far
  as FR-650 `feasible`; it never claims optimality or infeasibility.

### Search nodes and completeness

- A *search node* SHALL be the tuple of the FR-181 canonical state key, the
  state of every policy monitor (FR-803), the step count so far and, for
  each per-step sum or count over `steps` that the goal or the objective
  reads, its accumulated exact value. Two prefixes with equal nodes have
  equal sets of admissible completions and equal objective contributions,
  so the search expands one of them.
- A per-step sum objective (FR-802) MAY drop the accumulated value from the
  node and keep only the prefix with the better accumulated value (cost
  dominance), because a completion adds the same cost to each.
- When the goal or the objective reads `steps` other than through a per-step
  sum or count (an indexed read, a comparison of two steps, an order-sensitive
  query), the search SHALL NOT merge prefixes: every node is a distinct plan
  prefix. The space is still finite.
- The exploration is *complete* when every node reachable from the initial
  node by enabled steps within `max D` steps, and admissible as a prefix
  (FR-803), was expanded. Only a complete exploration yields `optimal` or
  `infeasible`, and only a complete exploration feeds a `potential` or
  `closure` certificate.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-801-AC-1 | Over FR-650-AC-1's counter with `plan_search.max_states` set to 2, the result is `incomplete` with `limit-reached{limit: max_states, value: 2, setting: plan_search.max_states}`, and rerunning with `--limit plan_search.max_states=1000` returns `optimal`. | Test (TC-1951) |
| FR-801-AC-2 | With `accounting.work_units` set below the work the goal evaluations need, the result is `incomplete` naming `accounting.work_units`, not a `plan_search` setting. | Test (TC-1951) |
| FR-801-AC-3 | A cancelled run is `incomplete` with `cancelled{source: requested}`; a run past its deadline with `cancelled{source: deadline}`; each carries its best plan as `feasible` when one was found and never `infeasible`. | Test (TC-1951) |
| FR-801-AC-4 | Over a model where `inc, dec` returns to the start state, the search admits one node for the start state at each step count and reports `states` equal to the number of distinct nodes, not the number of plan prefixes. | Test (TC-1951) |
| FR-801-AC-5 | A goal `Reach.steps[0].operation = M::Op::double` disables merging: `states` equals the number of admissible prefixes, and the result is still `optimal` with the plan whose first step is `double`. | Test (TC-1951) |
| FR-801-AC-6 | `--limit planner.max_work=1` changes no plan search limit, and `--limit plan_search.max_work=1` stops the search `incomplete` naming `plan_search.max_work`. | Test (TC-1951) |

## Dependencies

- [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) owns
  limit naming and the evaluation meter's settings.
- FR-181 owns the canonical state key and its byte limit.
- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds built from a complete exploration.
