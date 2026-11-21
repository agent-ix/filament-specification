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

Verify the planner limits, the evaluation meter, cancellation, node merging and the separation from the verification planner's limit. Scope: FR-801-AC-1 to FR-801-AC-6 and FR-801-AC-8 to FR-801-AC-10.

The native invocation fixture has one live `Device` receiver `d0` with
mutable `v: Int[0,2]`, initially 0, and no other changing state or policy.
In declaration order its operations are `blocked()` (precondition false)
and `choose(): Int[0,1]` (precondition true, frame only `v`, postcondition
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
9. Set the existing evaluation-accounting budget so a named charge during
   `choose`'s raw trial evaluation is denied, after `blocked` and the
   invocation charge for `choose` have succeeded but before its complete
   outcome list is available. Observe the named counter and granted search
   counts; do not replace the accounting budget with a search-work charge.

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
- Step 9: `incomplete` names the denied accounting setting, with
  `{states: 1, work: 2}` and no admitted depth-1 node or partial ordinal
  list; the two granted invocation charges remain counted.
