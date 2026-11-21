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
   - the receiver, when the operation has one, by the canonical value order
     of its object reference;
   - the arguments, parameter by parameter in declaration order, each by the
     canonical value order of its type: integers and exact numbers by value,
     enum values by declaration order, Booleans `false` before `true`, text
     by Unicode scalar sequence, records and tuples field by field, options
     with absent first, collections by their canonical element sequence, and
     object references by the canonical order of the referenced key;
   - the selected outcome's zero-based ordinal in the invocation's
     canonical outcome list below.

This order is total on distinct plans. The step order is the canonical order
FR-650-AC-1 relies on.

### Invocation outcomes

A native state-model invocation is the operation, receiver (absent for a
receiverless operation) and argument vector in parameter declaration order.
Invocation enumeration SHALL order by operation, then receiver, then arguments as
above. For each invocation the runtime may admit several `(result,
post-state)` pairs. Equal pairs are coalesced by the result's FR-181
canonical element encoding and the post-state's full canonical state key.
The list contains the native type/frame/postcondition-admitted outcomes,
before planner policy, goal or prefix-admissibility filtering. Those
filters do not renumber the invocation's outcomes.
Distinct pairs SHALL be sorted first by result in the canonical value
order of its declared type, then by ascending unsigned-lexicographic
post-state-key bytes. This is also their visitation order and the order
used to assign outcome ordinals. A resultless operation has the same
absent result for every pair, so only the post-state key orders its outcomes.

At a `Float32` or `Float64` result leaf, the selected floating-point
profile's `totalOrder` SHALL decide result order, retaining signed zeros
and distinct NaN payloads. This rule orders invocation results; it does
not widen supported objective families or change FR-802's NaN-objective
rules. Other result structures use the recursive type order above.

A step's identity SHALL include its invocation and selected outcome
ordinal. FR-805 omits the wire ordinal for a single-outcome invocation,
whose identity selects its only outcome. Rendered step names include an
ordinal only when the invocation has more than one distinct outcome.
Replay and certificate verification SHALL rederive the same ordered
outcome list from FR-181's native result/post-state relation and check the
selected identity under FR-650's library seam and internal checker
contracts. The ordinal selects the result as well as the post-state;
there is no additional result member on the wire step. Changing an
in-range ordinal changes the step identity, even when the post-state is
equal. It is not by itself an invalid step: replay checks the supplied
identity and post-state, while certificate checking compares claimed
identities with the complete rederived edge set. A changed claim that
duplicates another ordinal and loses the original edge rejects `closure`.
This invocation outcome list does not enumerate FR-819's continuous hybrid
trajectories; hybrid schedule steps retain their existing delay/transition
order and enclosure validation.

### Determinism

- The search MAY run in parallel, but the reported plan, objective,
  certificate, explored counts and outcome document SHALL be the ones the
  canonical sequential exploration of FR-801 "Exploration order" yields.
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
| FR-804-AC-5 | TC-1954's two-operation/two-receiver/Boolean-argument vector orders invocation tuples by operation, then receiver, then argument; one invocation's distinct pairs are visited in ascending result order and then unsigned-lexicographic canonical post-state-key order, irrespective of their generation order, with consecutive ordinals from 0. Equal result/post-state pairs occur once. | Test (TC-1954) |
| FR-804-AC-6 | Under the selected IEEE profile, TC-1954's `Float32` result bits `80000000`, `00000000`, `7fc00001`, `7fc00002`, and its `Float64` result bits `8000000000000000`, `0000000000000000`, `7ff8000000000001`, `7ff8000000000002`, order as listed for an otherwise equal post-state; all four results remain distinct and NaN results do not increment `explored.nan_objectives` when the objective is `length`. | Test (TC-1954) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) and
  [FR-802](FR-802-compute-plan-costs-exactly.md).
