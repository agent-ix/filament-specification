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

One planning result: `optimal`, `feasible`, `infeasible` or `incomplete`,
each with its plan, certificate and provenance as below.

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
| `incomplete` | a work, deadline or cancellation limit stopped the search | the FR-461 limit, the explored count and any best plan as `feasible` |

- Every plan in a result SHALL be replayed natively from the bound initial
  state before it is reported: each invocation enabled, every policy clause
  and the goal re-evaluated; a plan that fails replay is never reported, and
  the planner's run is a method failure naming the first failing step.
- An *optimality certificate* SHALL be one of:
  - `potential`: for an objective that is a sum of non-negative per-step costs
    (including `length`), a finite map from (state, remaining length) to an
    exact lower bound `pi` such that every mapped state's successors under
    every enabled invocation are mapped, `pi(s, k) <= cost(a) + pi(s', k - 1)`
    for every such step, `pi(s, k) <= 0` only where the goal and policy hold
    at `s`, and `pi(initial, max D)` equals the plan's objective value;
  - `closure`: for any other objective, the complete set of admissible-prefix
    states reachable within `max D` steps with each goal state's objective
    value, closed under every enabled invocation, in which no goal state
    improves on the plan.
- A *closure certificate* for `infeasible` SHALL be the complete set of
  admissible-prefix states reachable within `max D` steps, closed under every
  enabled invocation, holding no state at which the goal holds with a length
  in `D`.
- The core SHALL check every certificate by exact evaluation of the model's
  operations, policy, goal and objective. An accepted certificate gives the
  result the FR-331 certification `certified`; a rejected one demotes
  `optimal` to `feasible` and refuses `infeasible`, recording the rejected
  rule and the entry it failed at.

### Refusals

- A policy identity that names no temporal clause over `G`'s model refuses
  `missing_declaration`/`missing-name` naming it.
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
| FR-650-AC-2 | With `minimize Reach.steps.sum(s => if s.operation = M::Op::double then 5 else 1)` the same request returns `optimal` with the plan `inc, inc, inc, double` (values 1, 2, 3, 4, 8) and objective 8, beating `inc, double, double` at 11 and `double, double, double` at 15; seven `inc` steps at 7 exceed the length bound `Upto5`. | Test (TC-1603) |
| FR-650-AC-3 | With `satisfies { Reach.final.value = 21 }`, outside the counter's domain, the result is `infeasible` with an accepted closure certificate; with `domain` `Int[0, 2]` and goal value 8 it is `infeasible`. | Test (TC-1603) |
| FR-650-AC-4 | A policy clause `always holds(c.value != 4)` excludes every plan through value 4, and the returned plan replays with the policy holding at every state; a policy naming no clause refuses `missing_declaration`/`missing-name`; a policy `eventually holds(c.value = 3)` is evaluated on each completed plan and gives `optimal` `inc, inc, inc, double` (FR-803-AC-2). | Test (TC-1603) |
| FR-650-AC-5 | A tampered `potential` certificate with one successor missing is rejected naming the closure rule and that entry, and the result is `feasible`, not `optimal`; a search stopped by its work limit is `incomplete` naming the limit, carrying its best plan as `feasible`. | Test (TC-1603) |
| FR-650-AC-6 | A plan the planner reports that fails native replay is never reported: a fault-injected planner returning a plan with a disabled invocation gives a method failure naming that step. | Test (TC-1603) |

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
