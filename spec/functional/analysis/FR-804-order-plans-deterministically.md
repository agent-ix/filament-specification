---
id: FR-804
title: "Order plans totally and return the same plan on every run"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-802
    type: depends_on
---

# FR-804: Order plans totally and return the same plan on every run

## Description

When several admissible plans share the best objective value, the planner
SHALL return the first of them in one total canonical plan order, and SHALL
produce byte-identical outcome documents for equal requests, independent of
worker count, hash order, platform and timing.

## Inputs

The admissible plans of a complete or stopped exploration and their exact
objective values (FR-802).

## Outputs

The one plan an `optimal` or `feasible` result reports.

## Behavior

### The plan order

Plans SHALL be ordered by these keys, the first differing key deciding:

1. the objective value, ascending for `minimize` and descending for
   `maximize` (FR-802 comparison);
2. the length, shorter first;
3. the steps, compared lexicographically from step 0. Two steps compare by
   - for a timed model, the delay, smaller first (FR-808);
   - the operation, by its declaration order in the model;
   - the arguments, parameter by parameter in declaration order, each by the
     canonical value order of its type: integers and exact numbers by value,
     enum values by declaration order, Booleans `false` before `true`, text
     by Unicode scalar sequence, records and tuples field by field, options
     with absent first, collections by their canonical element sequence, and
     object references by the canonical order of the referenced key.

This order is total on distinct plans. The step order is the canonical order
FR-650-AC-1 relies on.

### Determinism

- The search MAY run in parallel, but the reported plan, objective,
  certificate, explored counts and outcome document SHALL be the ones a
  sequential exploration in canonical order yields.
- An `incomplete` result's best plan SHALL be the first in the plan order
  among the admissible plans the explored nodes contain, and the explored
  counts SHALL be those of the canonical exploration up to the denied
  charge, so the same limits give the same `incomplete` document. A
  deadline or a cancellation depends on timing; the document records it as
  such and is the only outcome exempt from byte equality.
- No outcome member SHALL depend on wall-clock time, a random seed, a path,
  a locale or a hash iteration order.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-804-AC-1 | Over FR-650-AC-1's counter, the two 3-step plans `inc, double, double` and `double, double, double` tie at length 3 and the reported plan is `inc, double, double`. | Test (TC-1954) |
| FR-804-AC-2 | With an objective on which a 2-step plan and a 3-step plan tie, the 2-step plan is reported. | Test (TC-1954) |
| FR-804-AC-3 | With a step `move(r: Robot, c: Cell)` the plans `move(a, c1)` and `move(b, c0)` order by `r` first (`a` before `b`), and `move(a, c0)` before `move(a, c1)` by enum declaration order of `Cell`. | Test (TC-1954) |
| FR-804-AC-4 | The same request run with one worker and with eight gives byte-identical outcome documents, and two runs stopped by the same `plan_search.max_work` give byte-identical `incomplete` documents with the same best plan. | Test (TC-1954) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) and
  [FR-802](FR-802-compute-plan-costs-exactly.md).
