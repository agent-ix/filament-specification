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

Each member is an unsigned 64-bit count, and a request that leaves a member
unset receives its published default:

| Member | Type | Default |
| --- | --- | --- |
| `max_states` | unsigned 64-bit integer, at least 1 | 10000000 search nodes |
| `max_work` | unsigned 64-bit integer, at least 1 | 100000000 work units |
| `max_certificate_entries` | unsigned 64-bit integer, at least 1 | 16777216 entries |
| `max_check_work` | unsigned 64-bit integer, at least 1 | 100000000 work units |
| `max_state_key_bytes` | unsigned 64-bit integer, at least 1 | 1048576 bytes |

The defaults follow the other engines' published defaults for the same
kind of resource: `max_states` is the model checker's state default and
`max_work` and `max_check_work` its transition default, since one search
node or one checked entry costs about as much as one explored state or
transition; `max_certificate_entries` is the certificate checkers' edge
default, since a plan certificate is read by the same kind of checker; and
`max_state_key_bytes` bounds one canonical state key at one mebibyte, far
above any finite model's key and well below the certificate byte ceiling.
The evaluation meter's defaults are the `accounting.<counter>` defaults of
FR-461 Behavior 6, not members of this table.

- **Overriding.** A caller sets a member through the limits type's setter in
  the library, the plan request's `limits` member under the bare member
  name (FR-805), or `--limit plan_search.<member>=<n>` on the CLI (FR-461
  Behavior 3). A set member replaces its default for that request only; an
  unset member keeps its default. The outcome's `limits` member lists all
  five by setting name with the value in force, defaults filled (FR-805).
- **Invalid values.** A value of 0, a negative value, a non-integer, or an
  integer above 18446744073709551615 refuses the request
  `invalid_runtime_input`/`invalid-value` at the member's pointer
  (`/limits/<member>`) before any search; a value of the wrong JSON kind
  refuses `invalid_runtime_input`/`wrong-value-kind` there; an unknown member
  of `limits` refuses `invalid_runtime_input`/`unknown-member` there; and an
  unknown setting name on the CLI refuses as FR-461-AC-4 states (FR-805,
  FR-806).

The settings are distinct from the verification planner's
`planner.max_work` (FR-492): the two engines share no limit.

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
  state of every named policy clause's monitor or automaton, safety or not
  (FR-803), the step count so far and, for
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
| FR-801-AC-7 | FR-650-AC-1's request with no `limits` member writes an outcome whose `limits` is exactly `{plan_search.max_states: 10000000, plan_search.max_work: 100000000, plan_search.max_certificate_entries: 16777216, plan_search.max_check_work: 100000000, plan_search.max_state_key_bytes: 1048576}`; the same request with `limits` `{max_states: 500}` lists `plan_search.max_states` 500 and the four other defaults. `limits.max_work` 0, `-1`, `1.5` and 18446744073709551616 each refuse `invalid_runtime_input`/`invalid-value` at `/limits/max_work`, `"10"` refuses `wrong-value-kind` there, and `limits.max_steps` refuses `unknown-member` at `/limits/max_steps`, each exit `20` before any search. | Test (TC-2003) |

## Dependencies

- [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) owns
  limit naming and the evaluation meter's settings.
- FR-181 owns the canonical state key and its byte limit.
- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds built from a complete exploration.
