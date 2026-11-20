---
id: TC-1972
title: "Export encodings of temporal clauses, fairness, protocol subjects and dense time"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-813
    type: verifies
---

# TC-1972: Export encodings of temporal clauses, fairness, protocol subjects and dense time

## Description

Verify direct and monitor-based temporal encodings, the constant identity
universe of quantified clauses, past-operator history encodings, the
terminal step, step intervals under stuttering, fairness, protocol subjects,
dense time and proof-free Lean output. Scope: FR-813-AC-1 to FR-813-AC-24.

## Test Procedure

1. Export `always holds(c.value <= 5)` and `eventually always holds(c.value
   <= 5)` over the counter with its snapshot, to both targets.
2. Export an `until` clause with and without a snapshot, and a clause over
   `(n: Int[0, 2])`.
3. Export `always[0, 3] holds(c.ok)` and insert stuttering steps into a
   behaviour.
4. Export weak fairness and `fair weak each` over `inc`.
5. Export the past clauses of FR-813-AC-8, AC-9, AC-10 and AC-23 and
   evaluate them on the stated behaviours.
6. Export the clauses of FR-813-AC-11 and AC-21 over the counter that
   deadlocks; run TLC as an observer and evaluate the Lean definitions.
7. Export the clause `T` required by protocol `Hand` and its guard, choice
   and overflow variants (FR-813-AC-12 to AC-17).
8. Export the `Lamp` clauses of FR-813-AC-18 and AC-19, and a clause over
   `Braking`.
9. In a build that lacks each construct of FR-813 "Constructs not lowered",
   export a clause holding it.
10. Inspect the Lean files.

## Expected Results

- Steps 1 to 4: the exact texts of AC-1, AC-6 and AC-7, no `DOMAIN v_`
  in a temporal-level range (AC-22), all `preserved`; the verdict unchanged
  by stuttering.
- Step 5: the exact register, counter and history texts and the stated
  truth values; no pre-origin read for unbounded or `[a,*]` operators.
- Step 6: the terminal disjunct and its weak fairness; TLC `holds`,
  `holds`, `violated` and the Lean values true, true, false.
- Step 7: the exact `PInit_Hand`, send, receive, `CASE` and overflow texts;
  `preserved` in both targets; the binding by the activation relation.
- Step 8: TLA+ `conditional` with `real-valued-time`, Lean `preserved`;
  the exact timed monitor and Lean `holds_T`; `Braking` keeps its
  `unrepresented` record.
- Step 9: `unimplemented_capability`/`unimplemented-meaning` naming the
  construct, exit 21, no package; never exit 30.
- Step 10: no `theorem`, proof or `sorry`.
