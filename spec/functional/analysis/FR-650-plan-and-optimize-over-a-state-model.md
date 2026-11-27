---
id: FR-650
title: "Plan and optimize over a state model within policy"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-194
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
---

# FR-650: Plan and optimize over a state model within policy

## Description

When a `synthesis` declaration's `grammar` names a state model object type,
the planner SHALL search the finite space of operation plans of that model up
to the declared plan length, keep only plans that satisfy the model's policy
at every step and the declaration's goal, order them by the declared
objective, and return a natively replayed plan with a certificate of its
optimality or feasibility, or a certificate that no plan exists.

## Inputs

A checked package; a `synthesis` declaration over a state model
([FR-194](FR-194-synthesize-bounded-candidates.md) "Source form"); the
initial-state snapshot the request binds; an optional list of policy clause
identities; and work, deadline and cancellation limits.

## Outputs

One planning result: `optimal`, `feasible`, `infeasible`, `incomplete` or
`inconclusive`, each with its plan, certificate and provenance as below.

## Behavior

### The plan candidate

- A *plan* SHALL be a finite sequence of operation invocations of the model
  object type `G`, each with arguments from its parameter domains, applied
  from the bound initial state by the native reference runtime (FR-181). The
  declaration's `domain D` SHALL be an integer range type; a plan's length
  SHALL be a value of `D`. A `D` that is not an integer range type with both
  bounds refuses `unsupported_projection`/`unbounded-extent` at the name.
- Inside `satisfies` and the objective, the declaration's name SHALL denote
  the plan, with the members `length` (`Integer`), `final` (the state reached,
  read like a model object of `G`), `steps` (an ordered collection of step
  records, each with `operation`, the invoked operation's name as an enum
  value, `arguments`, a record of its arguments, and `before` and `after`,
  the states around it) and, for a timed model, `elapsed` (the exact total
  delay). The `satisfies` block is the *goal*.
- A plan is *admissible* when every invocation is enabled (its precondition
  holds) where it is applied, every state clause and invariant of the model
  holds at every state of the plan, every policy clause the request names (a
  temporal clause over the model) holds on the plan's behaviour (its states
  followed by the terminal stutter of the final state) under its profile,
  and the goal holds ([FR-803](FR-803-hold-plans-to-policy-clauses.md)).
- A state at which a model invariant is violated SHALL be inadmissible. A
  state at which an invariant cannot be decided (its evaluation is
  undefined) SHALL NOT be expanded. A run that excluded any state for that
  reason SHALL settle `incomplete` with cause `invariant-undetermined`
  naming the invariant, carrying the best plan found that passes through no
  such state. It SHALL NOT settle `optimal` or `infeasible` and writes no
  certificate, because the excluded state may begin an admissible plan
  better than the best one or the only one. A checker SHALL NOT accept a
  certificate whose derivation rests on an undetermined invariant.
- If an undefined precondition guard makes a contract conjunction have no
  value at a reachable state, the run SHALL settle `incomplete` with cause
  `{"type": "contract-undefined", "clause": <name>}`, naming the clause
  (FR-1012-AC-9 / UE-6). This cause takes precedence over
  `invariant-undetermined` when both apply.
- An `incomplete` result has one cause. A work limit, the deadline or a
  cancellation that stopped the search wins: the cause is that stop, even
  when the search had also excluded an undetermined state or encountered an
  undefined contract conjunction. `invariant-undetermined` is the cause only
  when the search otherwise ran to completion and no contract conjunction
  was undefined.
- The objective SHALL be the `minimize` or `maximize` expression; a
  declaration with none minimizes `length`. A sum of per-step costs is
  written as a collection query over `steps`; the final-state value as an
  expression over `final`; a schedule's makespan as `elapsed`.

### Result

| Result | When | Carries |
| --- | --- | --- |
| `optimal` | an admissible plan whose objective no admissible plan of length in `D` improves | the plan, its objective value, and an optimality certificate the core accepted |
| `feasible` | an admissible plan whose optimality is not certified | the plan, its objective value, and the best certified bound (`lower` for `minimize`, `upper` for `maximize`) when one exists |
| `infeasible` | no admissible plan of any length in `D` | a closure certificate the core accepted |
| `incomplete` | a work, deadline or cancellation limit stopped the search, or the search excluded a state at which an invariant could not be decided | the FR-461 limit or the `invariant-undetermined` cause, the explored count and any best plan as `feasible` |
| `inconclusive` | a policy clause the request names has no instance (FR-803) | the FR-331 cause `no-instance`, no plan and no certificate |

- Every plan in a result SHALL be replayed natively from the bound initial
  state before it is reported: each invocation enabled, every policy clause
  and the goal re-evaluated; a plan that fails replay is never reported, and
  the planner's run is a method failure naming the first failing step.
- Every certificate SHALL be keyed by FR-801's *search node*: the canonical
  state key, the state of every named policy clause's monitor or automaton,
  the step count `k` and every accumulator the node keeps. The remaining
  length of a node is `max D - k`, so FR-801's node determines it; a model
  with no policy and no kept accumulator has nodes that are exactly (state,
  step count). One key serves the search, the certificate entries and the
  checker, because a state alone does not decide its completions once a
  monitor or an accumulator is part of the node. Entries are listed in the
  canonical exploration order (FR-801 "Exploration order"), and an entry is
  named by its zero-based index in that list. Each entry holds its node, its
  value (`pi`, or the goal node's objective value) and one successor record
  per enabled, prefix-admissible step from it, in FR-804's step order, each
  record naming the step and the index of the successor's entry; the
  checker re-derives every step and rejects `closure` when a re-derived step
  has no record or names the wrong node.
  For native state-model plans, a successor's step identity includes the
  exact operation, receiver, arguments and selected outcome ordinal under
  FR-804/FR-805. The checker rederives the complete native result/post-state
  list before the simulator's graph-edge coalescing. It requires exactly
  one record for every distinct enabled, prefix-admissible outcome,
  including outcomes with equal post-states but different results; a
  duplicate, missing or malformed identity rejects `closure` at that entry.
  `max_check_work` charges once per entry read and once per distinct
  successor outcome rederived, not once per invocation or raw frame trial.
- An *optimality certificate* SHALL be one of:
  - `potential`: for an objective that is a sum of non-negative per-step costs
    (including `length`), a finite map from search nodes to an exact lower
    bound `pi` satisfying four rules: `closure`, every successor node of a
    mapped node under every enabled, prefix-admissible step is mapped;
    `descent`, `pi(n) <= cost(a) + pi(n')` for every such step `a` from `n`
    to `n'`; `goal`, `pi(n) <= 0` at every node whose length is in `D` and
    at which the goal holds and every non-safety policy automaton accepts on
    the terminal stutter; and `initial`, `pi(initial node)` equals the plan's
    objective value. The `goal` rule does not require the converse: with
    zero-cost steps a node at which the goal does not hold may correctly
    have `pi(n) <= 0`, because a goal node may be reachable from it at no
    further cost;
  - `closure`: for any other objective, the complete set of admissible-prefix
    search nodes reachable within `max D` steps with each goal node's
    objective value, satisfying `closure` (closed under every enabled step)
    and `improvement` (no goal node improves on the plan).
- A *closure certificate* for `infeasible` SHALL be the complete set of
  admissible-prefix search nodes reachable within `max D` steps, satisfying
  `closure` and `goal-free` (no node at which the goal holds with a length in
  `D`).
- The core SHALL check every certificate by exact evaluation of the model's
  operations, policy, goal and objective, re-deriving each entry's
  successors itself. An accepted certificate gives the result the FR-331
  certification `certified`. A rejected optimality certificate demotes
  `optimal` to `feasible` with certification `uncertified`. A rejected
  closure certificate for `infeasible` leaves no definite answer: the result
  is `incomplete` with cause `certificate-rejected{rule, entry}` and no plan,
  because a rejected proof of absence proves nothing and a refusal would
  blame the request. Every rejection records the first failing rule, by the
  rule names above, and the entry it failed at.

### Test seams

An implementation SHALL provide two library test seams so that the replay
and certificate checks can be shown to reject a bad plan or a bad
certificate, which a correct search never produces:

- a *plan substitution* seam, which replaces the plan the search reports
  before native replay with caller-given invocations, extended to accept
  step records and separately supplied recorded post-states for negative
  tests. A recorded post-state is optional test data; when supplied it must
  match the selected native outcome. When none is supplied, replay advances
  through that selected outcome's state. For native state-model steps the extended library input uses FR-805's
  step member/value shapes, including optional `receiver` and
  `outcome_ordinal`, and can represent malformed member kinds for negative
  tests. This is test data supplied through the library seam, not a new
  request or outcome-reader surface. Replay validates presence, kinds and
  range, resolves the invocation and selected outcome, and compares its
  rederived identity with the supplied step, and its post-state with any
  recorded post-state supplied. A defect gives the existing method failure, exit `30`, naming
  the first failing step, operation and offending supplied member when
  present; no plan outcome document is written. A valid in-range ordinal
  selecting another result with the same post-state is another valid step,
  not a defect merely because its identity differs from the original plan;
  the substituted plan still has to satisfy its goal and policies; and
- a *certificate tamper* seam, which removes a named successor or replaces
  its claimed step identity or successor-entry index in a named entry of
  the internal certificate before checking. Its library input likewise
  permits malformed native receiver/ordinal member values. The certificate
  is the internal entry/successor structure above, not FR-805's wire
  summary. The checker validates each supplied claim against the rederived
  outgoing edge set, including ordinal identity even when post-states are
  equal. A malformed, duplicate, missing or wrong claimed identity or
  target rejects the existing `closure` rule at the first offending entry.
  The existing demotion of optimality to `feasible`, or of infeasibility
  to `incomplete` with `certificate-rejected{rule, entry}`, then applies.

Neither seam SHALL be reachable from a plan request, a host request surface
or a CLI option: the request member lists of FR-805 stay closed, and a
request member naming either seam refuses `invalid_runtime_input`/
`unknown-member`. A seam exists to test the checks, and a wire member would
let a caller forge a result.

### The counter fixture

The acceptance criteria of FR-650 and FR-800 to FR-806 that name "the
counter" run over one published fixture, `proposals/planning-v1/fixtures/counter/`:
a model `M` with one object type `Counter` holding `value: Int[0, 20]`, one
counter `c` with `value = 1` in the initial snapshot, and two operations
declared in this order: `inc` (precondition `value < 20`, effect `value =
value + 1`) and `double` (precondition `value <= 10`, effect `value = 2 *
value`), whose operation enum is `M::Op` with cases `inc` and `double`. The
fixture's unit declares `type Upto5 = Int[0, 5];`, `type Cost = Integer;` (the
result type a typed `sum<Cost>` objective names) and the temporal clauses
`NotFour` (`always holds(c.value != 4)`) and `SomeTime` (`eventually
holds(c.value = 3)`) over `c: M::Counter`, and FR-650-AC-1's declaration
`synthesis Reach using p grammar M::Counter domain Upto5 satisfies {
Reach.final.value = 8 };` on one line, so a criterion that changes the goal
or adds an objective edits that one declaration. Unless a criterion says
otherwise, a run over the counter has the goal `Reach.final.value = 8`,
`domain Upto5`, the objective `length` and an empty policy list.
Every counter invocation has receiver `c` and no arguments. Operation-only
plan names in these criteria abbreviate those full invocations. Each
invocation has one native outcome, so its wire step carries `c`'s exact
FR-181 typed reference and omits `outcome_ordinal`.

### Refusals

- A policy identity that names no `temporal` declaration refuses
  `missing_declaration`/`missing-name` naming it; one that names a temporal
  clause whose `over` binding is not an object of `G`'s model refuses
  `ill_typed`/`type-mismatch` naming it (FR-806).
- A policy clause that is not a safety clause (one whose violation is not
  shown by a finite trace) is not refused: it prunes no prefix and is
  evaluated on each completed plan's behaviour (FR-803). The complete
  refusal set is FR-806's.
- A request whose bound initial snapshot does not admit for `G`'s model
  refuses with the snapshot's own admission cause.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-650-AC-1 | Over a model with operations `inc` and `double` on a counter in `Int[0, 20]` from `value = 1`, `synthesis Reach using p grammar M::Counter domain Upto5 satisfies { Reach.final.value = 8 };` with `type Upto5 = Int[0, 5];` returns `optimal` with objective 3 (`length`) and the plan `inc, double, double`, the first of the two 3-step plans in canonical order since `inc` is declared before `double`, and a `potential` certificate the core accepts, certification `certified`. | Test (TC-1603) |
| FR-650-AC-2 | With `minimize sum<Cost>(s in Reach.steps : if s.operation = M::Op::double then 5 else 1)` the same request returns `optimal` with the plan `inc, inc, inc, double` (values 1, 2, 3, 4, 8) and objective 8, beating `inc, double, double` at 11 and `double, double, double` at 15; seven `inc` steps at 7 exceed the length bound `Upto5`. | Test (TC-1603) |
| FR-650-AC-3 | With `satisfies { Reach.final.value = 21 }`, outside the counter's domain, the result is `infeasible` with an accepted closure certificate; with `domain` `Int[0, 2]` and goal value 8 it is `infeasible`. | Test (TC-1603) |
| FR-650-AC-4 | A policy clause `always holds(c.value != 4)` excludes every plan through value 4, and the returned plan replays with the policy holding at every state; a policy naming no clause refuses `missing_declaration`/`missing-name`; a policy `eventually holds(c.value = 3)` is evaluated on each completed plan and gives `optimal` `inc, inc, inc, double` (FR-803-AC-2). | Test (TC-1603) |
| FR-650-AC-5 | Over the counter with goal value 8, the certificate tamper seam removing the successor reached by `double` from the entry of the initial node makes the core reject the `potential` certificate with `rule` `closure` and `entry` 0, and the result is `feasible` with the plan `inc, double, double`, certification `uncertified`, never `optimal`; a search stopped by its work limit after an admissible plan was found is `incomplete` naming the limit and carrying that best plan. | Test (TC-1603) |
| FR-650-AC-6 | The plan substitution seam replacing the reported plan of the counter run with `double, double, double, double, double` (the fifth `double` applied at value 16, where its precondition `value <= 10` fails) gives a method failure, exit `30`, naming step index 4 and `double`, and no plan outcome document is written. | Test (TC-1603) |
| FR-650-AC-7 | Over the counter with goal value 21, the certificate tamper seam removing one successor from entry 0 of the closure certificate makes the core reject it with `rule` `closure` and `entry` 0, and the result is `incomplete` with cause `certificate-rejected{rule: closure, entry: 0}` and plan `null`, never `infeasible`; a plan request carrying a member `replay_plan`, `tamper` or `engines` refuses `invalid_runtime_input`/`unknown-member` at that member's pointer, exit `20`, before any search. | Test (TC-1603) |
| FR-650-AC-8 | Over the counter with goal value 8 and `minimize sum<Cost>(s in Reach.steps : if s.operation = M::Op::double then 0 else 1)`, the result is `optimal` with objective 0 and the plan `double, double, double` (values 1, 2, 4, 8), the zero-cost step `double` from value 4 entering the goal from a node at which the goal does not hold; its `potential` certificate maps that node and the initial node, at neither of which the goal holds, to `pi` 0, and the core accepts it, certification `certified`. | Test (TC-1603) |
| FR-650-AC-9 | Over the counter with goal value 8 and a model invariant `Ratio` of `Counter`, `12 / (value - 3) != 5`, which is undefined (a division by zero) at `value = 3` and true at every other value, the state reached by `inc, inc` is not expanded and the result is `incomplete` with cause `{"type": "invariant-undetermined", "invariant": "Ratio"}`, the best plan `inc, double, double` (which passes through no state with `value = 3`), certificate `null` and certification `uncertified`, never `optimal`; with goal value 3 no plan passes through no such state, and the result is `incomplete` with that cause and plan `null`, never `infeasible`. | Test (TC-1603) |
| FR-650-AC-10 | TC-1603's native seam vectors give method failure exit `30`, the first failing step/operation/member and no outcome document for malformed substituted receivers or ordinals. Two valid ordinals selecting `(0,S0)` and `(1,S0)` both replay as different identities; tampering an internal certificate claim to duplicate one ordinal and omit the other rejects `closure` at its entry even though post-states match, applying the existing certificate demotion. | Test (TC-1603) |

## Dependencies

- [FR-194](FR-194-synthesize-bounded-candidates.md) owns the `synthesis`
  source form, the finite-domain rule and the objective.
- FR-181 owns native execution of operation invocations.
- FR-331 owns certification values.
- [FR-800](FR-800-define-the-plan-and-goal-model.md) to
  [FR-810](FR-810-plan-and-optimize-the-six-reference-cases.md) specify the
  plan model, the bounded search and its limits, exact costs, policies, the
  plan order, the request and outcome documents, refusals, the checked
  package nodes read, timed and probabilistic plans and the reference cases.
