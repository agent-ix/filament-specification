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
| `max_work` | `plan_search.max_work` | one per native operation/receiver/arguments invocation attempted (per applied schedule step for hybrid plans), one per policy-clause monitor update (one per clause, however many instances it has; FR-803), one per goal or objective evaluation, one per frontier comparison |
| `max_certificate_entries` | `plan_search.max_certificate_entries` | one per certificate entry the search writes |
| `max_check_work` | `plan_search.max_check_work` | one per certificate entry the core checker reads and one per distinct successor outcome it re-derives (not per invocation or raw frame trial) |
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
- A state at which a model invariant cannot be decided is not expanded
  (FR-650), so an exploration that excluded one is not complete: the run
  settles `incomplete` with cause `invariant-undetermined` naming the
  invariant, carries the best plan found that passes through no such state,
  and feeds no certificate.

### Search nodes and completeness

- A *search node* SHALL be the tuple of the FR-181 canonical state key, the
  state of every named policy clause's monitor or automaton, safety or not
  (FR-803; a clause's state is the states of all its instances, in instance
  order), the step count so far and, for
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
### Exploration order

The *canonical sequential exploration* SHALL be this one order, so that the
reported plan, every count and every `incomplete` document are fixed by the
request alone (FR-804):

1. **Frontier.** The search expands nodes layer by layer, breadth-first by
   step count: every node of step count `k` before any node of step count
   `k + 1`, for `k` from 0 to `max D - 1`. Layer 0 is the initial node.
2. **Order within a layer.** The nodes of a layer are expanded in ascending
   order of these keys, the first differing key deciding: for a per-step
   sum objective, the accumulated value, ascending for `minimize` and
   descending for `maximize` (FR-802 comparison); then the node's
   representative prefix in FR-804's step order. A node's *representative
   prefix* is the prefix that admitted it; when a later prefix reaches an
   admitted node, the representative becomes the later prefix only when its
   accumulated value is strictly better (cost dominance), so equal values
   keep the earlier prefix.
3. **Expansion.** A node's successors are generated in FR-804's step order:
   operations in declaration order, then receivers, then arguments in the
   canonical value order of their types, and each invocation's distinct
   outcomes in FR-804's `(result, post-state)` order.
4. **Charge order.** The initial node charges one `max_states` on
   admission, then one `max_work` per policy clause for its monitor's reading
   of the initial state, in the request's `policy` order, stopping at the
   first safety monitor that rejects; then, when no monitor rejected and 0 is
   in `D`, one `max_work` for the goal evaluation and, when the goal holds,
   one for the objective evaluation.
   In a native state-model search, each operation/receiver/arguments
   invocation of an expanded node charges one `max_work` before its
   precondition or invocation evaluation, including an invocation that is
   disabled or yields no admitted outcome. Every
   distinct `(result, post-state)` outcome of that invocation is covered by
   this one invocation charge; nondeterministic invocations are supported.
   Raw frame trials rejected by the postcondition or typed state admission charge
   evaluation accounting, not additional invocation work units. A denied
   invocation charge performs no invocation evaluation. The complete
   invocation outcome list is established before any outcome from it is
   admitted to the search; an evaluation-accounting stop during its
   enumeration stops `incomplete`, without assigning ordinals from a
   partial list. For each admitted outcome, in canonical outcome order,
   charge one `max_work` per policy clause's monitor or automaton update
   (FR-803), in the
   request's `policy` order, stopping at the first safety monitor that
   rejects; then, if a node with the same key is already admitted, one
   `max_work` for that frontier comparison and nothing more; otherwise one
   `max_states` for admitting the new node and, when its step count is in
   `D`, one `max_work` for the goal evaluation and, when the goal holds,
   one for the objective evaluation. The evaluation meter is charged by the
   evaluations themselves, beside these counts.
5. **Denial.** A charge is denied when it would take its count above the
   bound in force. The search stops at the first denied charge; `explored`
   reports the granted charges only, so `states` is the number of admitted
   nodes and `work` the number of granted `max_work` units.

A parallel search SHALL report the counts and the plan of this order.

The native invocation rule does not enumerate FR-819's continuous hybrid
trajectories. Hybrid plans retain their schedule/enclosure semantics and
their per-applied-step search charge.

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
| FR-801-AC-8 | Over the counter (FR-650 "The counter fixture"), `plan_search.max_states` 2 gives `incomplete` with `explored` `{states: 2, work: 6}` (initial admission and goal evaluation; `inc` applied, node `(2, 1)` admitted and its goal evaluated; `double` applied and compared with node `(2, 1)`; `inc` from `(2, 1)` applied and its admission denied) and plan `null`; `plan_search.max_work` 1 gives `incomplete` with `explored` `{states: 1, work: 1}`, the denied charge being the application of `inc` to the initial node; two runs of each give byte-identical documents. | Test (TC-1951) |
| FR-801-AC-9 | TC-1951's invocation fixture with domain `Int[1,1]`, false goal and empty policy completes with `explored` `{states: 4, work: 5}`: one disabled invocation, one invocation admitting three distinct outcomes, and three goal evaluations. Rejected raw frame trials charge only evaluation accounting; no extra invocation work is charged per trial or outcome. | Test (TC-1951) |
| FR-801-AC-10 | With `plan_search.max_work` 1, TC-1951's fixture stops `incomplete` with `explored` `{states: 1, work: 1}` before its second invocation. With `accounting.work_units` 1 instead, the denied count 2 gives `limit-reached{limit: work_units, value: 1, setting: accounting.work_units}`, `{states: 1, work: 2}`, and no partial outcome list or ordinal. | Test (TC-1951) |
| FR-801-AC-11 | TC-1951's four-entry closure certificate with three outgoing outcome records costs exactly seven `plan_search.max_check_work` units: four entry reads and three distinct successor outcomes rederived. Bound 7 suffices; bound 6 stops the checker on charge 7 naming `plan_search.max_check_work`. Raw trials and invocation count do not replace the per-outcome checker charge. | Test (TC-1951) |

## Dependencies

- [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) owns
  limit naming and the evaluation meter's settings.
- FR-181 owns the canonical state key and its byte limit.
- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds built from a complete exploration.
