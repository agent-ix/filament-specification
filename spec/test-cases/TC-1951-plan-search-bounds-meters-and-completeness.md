---
id: TC-1951
title: "Plan search bounds, meters and completeness"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-801
    type: verifies
---

# TC-1951: Plan search bounds, meters and completeness

## Description

Verify the planner limits, the evaluation meter, cancellation, node merging and the separation from the verification planner's limit. Scope: FR-801-AC-1 to FR-801-AC-6 and FR-801-AC-8 to FR-801-AC-11.

Construct the native library fixture with domain package identity
`example/plan-invocation` as follows.
The selected model alias is `M`, the grammar is `M::Device`, and its
single complete population `devices` contains exactly the live receiver
`d0`. Its only direct attribute is required `v: Int[0,2]`, initially 0,
so FR-322 exposes `Reach.final.v` directly as well as the population's
object record. There are no supertypes, other attributes or operations,
creation/deletion effects, invariants, or policy clauses.
In declaration order its operations are `blocked()` (no result, false
precondition, empty frame and true postcondition)
and `choose(): Int[0,1]` (precondition true, frame only `self.v`, postcondition
`result = 0`). The native frame domain is all three `v` values times both
result values: six raw trials, three accepted pairs `(0, S0)`, `(0, S1)`,
`(0, S2)`, where `Si` is the same world with `d0.v = i`. Their full
canonical state keys order `S0`, `S1`, `S2`; all identities, populations
and other fields stay fixed. All invariants are true. The length domain
is `Int[1,1]`, the goal is false, and the policy list is empty. No initial
goal evaluation occurs because length 0 is outside that domain.
Limits other than the one being tested permit complete enumeration and
certificate checking; the default objective is `length`.

## Test Procedure

1. Run FR-650-AC-1's request with `plan_search.max_states` 2, then 1000.
2. Run it with a low `accounting.work_units`.
3. Cancel a run; run one past its deadline.
4. Plan over a model where `inc, dec` returns to the start.
5. Plan with a goal reading `Reach.steps[0]`.
6. Run with `planner.max_work=1` and with `plan_search.max_work=1`.
7. Over the counter, run twice with `plan_search.max_states` 2 and twice with `plan_search.max_work` 1.
8. Run the native invocation fixture with sufficient limits, then with
   `plan_search.max_work` 1. Repeat both with reversed raw trial generation
   order, which must not affect the canonical outcome list.
9. Set `accounting.work_units` to 1 with other accounting limits sufficient,
   and generate raw trials in result-first, then `v` order, beginning with
   `(0,S0)` (no field write). The constant preconditions add no nested
   language function call. The next denied evaluation-work charge occurs
   while evaluating `choose`, before its first complete outcome list:
   either its native precondition call after the blocked call, or the
   first scalar equality's two-work-unit plan-form charge, under the
   existing value-accounting schedule. Both exceed bound 1 at count 2.

10. Check the complete closure certificate of step 8 with
    `plan_search.max_check_work` 7, then 6. Its four entries include
    exactly three outgoing outcome records at entry 0; depth-1 entries
    have no outgoing step within the bound.

## Expected Results

- Step 1: `incomplete` naming `plan_search.max_states`, then `optimal`.
- Step 2: `incomplete` naming `accounting.work_units`.
- Step 3: `cancelled{source: requested}` and `cancelled{source: deadline}`, best plan as `feasible`, never `infeasible`.
- Step 4: `states` counts distinct nodes.
- Step 5: no merging; `optimal` with first step `double`.
- Step 6: unchanged, then `incomplete` naming `plan_search.max_work`.
- Step 7: `explored` `{states: 2, work: 6}` and `{states: 1, work: 1}`, plan `null`, each pair byte-identical.
- Step 8: with sufficient limits, `infeasible`, `explored` `{states: 4,
  work: 5}`: one initial and three depth-1 nodes; one work charge for
  `blocked`, one for all six raw trials of `choose`, and three goal
  evaluations. The three rejected result-1 trials use evaluation
  accounting only. With work 1, `incomplete`, `{states: 1, work: 1}`;
  `choose` is not evaluated after its invocation charge is denied. Both
  trial-generation orders give equal outcomes and counts.
- Step 9: `incomplete` with
  `limit-reached{limit: work_units, value: 1, setting: accounting.work_units}`,
  retaining denied count 2 under FR-461, `{states: 1, work: 2}` and no
  admitted depth-1 node or partial ordinal list. The two granted
  invocation charges remain counted.
- Step 10: seven units (four entry reads and three distinct successor
  rederivations) suffice; six stops the checker on its seventh charge,
  `incomplete` with `limit-reached{limit: max_check_work, value: 6,
  setting: plan_search.max_check_work}`, retaining denied count 7 and
  the completed search's `{states: 4, work: 5}`.
  Rejected raw frame trials consume evaluation accounting but no check
  work unit, and the three outcomes do not become one invocation unit on
  this checker counter.
