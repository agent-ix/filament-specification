---
id: FR-803
title: "Hold every plan to the model's rules and the requested policy clauses"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-800
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-377
    type: references
---

# FR-803: Hold every plan to the model's rules and the requested policy clauses

## Description

When the planner judges a plan, it SHALL hold it to the model's own state
clauses and invariants at every state and to every policy clause the
request names over the plan's whole behaviour, so that no reported plan
breaks a rule, and SHALL prune a prefix as soon as a safety rule is broken.

## Inputs

The model's state clauses and invariants; the request's ordered list of
policy clause identities; the plan (FR-800).

## Outputs

An admissibility verdict per plan and per prefix, and per-policy monitor
states for the search node (FR-801).

## Behavior

### What a policy is

- The model's state clauses and invariants SHALL always apply; they need no
  naming.
- A *policy clause* SHALL be a `temporal` declaration of the package whose
  `over` binding is an object of `G`'s model (or the model itself), named in
  the request's `policy` list. Each is read under its own profile and clock.
- The plan's *behaviour* SHALL be its states from the initial state through
  `final`, followed by the terminal stutter of `final` repeated forever: the
  plan ends and the system rests in its final state, as a terminal state is
  read elsewhere (FR-377 "Terminal states and deadlocks"). Under a
  step-counting profile each step is one position; under a timed profile the
  positions carry the plan's delays (FR-808).
- A plan *keeps* a policy clause when the clause holds on its behaviour.

### Safety and non-safety policies

- A policy clause is a *safety* clause when every violation is shown by a
  finite prefix: `always` of a past or state formula, bounded `eventually`
  and bounded `until` with a finite upper interval bound, and their
  conjunctions. The planner SHALL compile a safety clause to a monitor whose
  state is part of the search node and SHALL prune a prefix at the first
  position where its monitor rejects; no completion of a rejected prefix is
  explored.
- Any other clause (an unbounded `eventually`, an unbounded `until`, a
  `release` whose obligation never closes on a finite prefix) SHALL be
  evaluated on the completed plan's behaviour, which is a lasso and so
  decides the clause; such a clause prunes nothing, and a plan that does not
  keep it is not admissible.
- A bounded `eventually[0, k]` whose deadline lies after the plan's last
  step is decided on the terminal stutter: it holds only if its operand
  holds at some position up to `k`, including the stutter positions.

### Admissibility

A plan is *admissible* when every step is enabled, every state clause and
invariant of the model holds at every state of the plan, every policy clause
is kept, the plan's length is a value of `D`, and the goal holds (FR-800).
A rule whose evaluation is undefined at a position does not hold there.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-803-AC-1 | Over FR-650-AC-1's counter, the policy `always holds(c.value != 4)` prunes every prefix at the state with value 4, and the search reports no node with value 4. | Test (TC-1953) |
| FR-803-AC-2 | The policy `eventually holds(c.value = 3)` is not refused: it is evaluated on each completed plan, so the result is `optimal` `inc, inc, inc, double` (values 1, 2, 3, 4, 8) at length 4, the first 4-step plan in canonical order that visits 3; neither 3-step plan to 8 visits 3. | Test (TC-1953) |
| FR-803-AC-3 | A bounded policy `eventually[0, 2] holds(c.value >= 4)` admits `inc, double, double` (value 4 at position 2) and prunes the prefix `inc, inc` at position 2 (value 3), so no reported plan begins `inc, inc`. | Test (TC-1953) |
| FR-803-AC-4 | A model invariant `value <= 10` excludes the step from 6 to 12 without naming any policy, and a policy over another model's object refuses as FR-806 states. | Test (TC-1953) |
| FR-803-AC-5 | With the policy `always holds(c.value != 8)` every plan reaching the goal value 8 breaks the policy at its final state, no plan is admissible, and the result is `infeasible` with an accepted closure certificate. | Test (TC-1953) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) and
  [FR-800](FR-800-define-the-plan-and-goal-model.md) own the plan and its
  result.
- FR-377 owns the terminal-stutter reading of a terminal state.
