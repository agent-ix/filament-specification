---
id: FR-800
title: "Define the plan, its steps and its goal over a state model"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-194
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
---

# FR-800: Define the plan, its steps and its goal over a state model

## Description

When a `synthesis` declaration selects plan synthesis over a state model
object type `G` (FR-194 "Source form", FR-650), the planner SHALL treat a
plan as a finite sequence of steps from the bound initial state, each step
one enabled operation invocation of `G`'s model with arguments drawn from
finite argument domains, and SHALL read the declaration's `satisfies` block
as the goal over that plan.

## Inputs

The checked package; the `synthesis` declaration; the initial-state
snapshot the request binds.

## Outputs

The plan space the search of FR-801 enumerates, the step records a plan
carries, and the goal and objective expressions read over them.

## Behavior

### Steps and arguments

- A *step* SHALL be one invocation of an operation declared by `G`'s model,
  applied to the state reached by the previous step (the bound initial state
  for the first step) by the native reference runtime (FR-181). A step is
  *enabled* when the operation's precondition holds at that state and the
  post-state the runtime computes is well-typed and admits for the model;
  only enabled steps occur in a plan.
- Each operation parameter SHALL range over a finite *argument domain*:
  - a parameter whose type is a reference to an object type ranges over the
    keys of that object type's population in the state the step is applied
    to, so an object created by an earlier step is a candidate argument of
    every later step and a deleted one is not;
  - a parameter of any other type ranges over its type's value set, which
    SHALL be finite by FR-194's bound rule ("The bound"): every numeric
    component bounded, every collection with a maximum cardinality, every
    text with a maximum length, recursion only through a finitely bounded
    type.
- An operation with several parameters ranges over the product of their
  domains, in parameter declaration order.
- A plan over a model with several object types or populations is a plan
  over the whole model state: every step may invoke any operation of the
  model on any object of its receiver's population.

### The plan value

Inside the `satisfies` block and the objective the declaration's name SHALL
denote the plan, with these members:

| Member | Type | Meaning |
| --- | --- | --- |
| `length` | `Integer` | the number of steps |
| `final` | the model state | the state after the last step, read like a model object of `G` |
| `initial` | the model state | the bound initial state |
| `steps` | ordered collection of step records | one record per step, in plan order |
| `elapsed` | `Duration` of the model's time unit | timed models only (FR-808): the exact total delay |
| `probability` | `Rational` | probabilistic models only (FR-809): the exact probability of the goal |

A step record SHALL carry `index` (`Integer`, from 0), `operation` (the
invoked operation's name as a value of the model's operation enum),
`arguments` (a record with one field per parameter), `before` and `after`
(the states around the step) and, for a timed model, `delay` (the exact
delay before the invocation).

### The goal

- The goal SHALL be the `satisfies` block, a total Boolean expression over
  the plan. A plan *reaches the goal* when the block evaluates to `true`.
  An evaluation of the block that is undefined (FR-301 `UndefinedEvaluation`)
  does not reach the goal.
- The empty plan (length 0) SHALL be a candidate whenever 0 is a value of
  `D`; it reaches the goal when the block holds of the initial state.
- A plan's states SHALL be compared by their FR-181 canonical state key, so
  two prefixes that reach states with equal keys reach the same state.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-800-AC-1 | Over a model with object type `Server` (population `n1`, `n2`) and operation `drain(s: Reference<Server>)`, a plan of length 2 binds `s` to `n1` and then `n2`, and `Reach.steps` reads two records with `index` 0 and 1, `operation` `drain`, `arguments.s` `n1` and `n2`, and `before`/`after` states whose keys equal the runtime's states. | Test (TC-1950) |
| FR-800-AC-2 | An operation `spawn` that creates a `Server` `n3` makes `n3` a candidate argument of every later step and of no earlier one. | Test (TC-1950) |
| FR-800-AC-3 | With `domain` `Int[0, 3]` and a goal true in the initial state, the result is the empty plan with `length` 0. | Test (TC-1950) |
| FR-800-AC-4 | A goal `Reach.final.value / Reach.final.divisor = 2` that is undefined on a plan whose final `divisor` is 0 does not reach the goal on that plan. | Test (TC-1950) |
| FR-800-AC-5 | An operation parameter of type `Integer` with no bounds refuses as FR-806 states, naming the parameter. | Test (TC-1950) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds and certificates; [FR-194](FR-194-synthesize-bounded-candidates.md)
  owns the source form and the bound rule.
- FR-181 owns native execution and the canonical state key.
- [FR-808](FR-808-plan-over-timed-models.md) and
  [FR-809](FR-809-plan-over-probabilistic-models.md) extend the plan value
  for timed and probabilistic models.
