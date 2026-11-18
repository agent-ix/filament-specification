---
id: TC-2010
title: "A replay results row is spelled for every replay-result arm"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-323
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-433
    type: verifies
---

# TC-2010: A replay results row is spelled for every replay-result arm

## Description

Verify that each of the five FSL FR-072 replay-result arms writes its
`results` row with exactly the members FR-323 "Replay result rows" gives it,
and that a reader refuses an unknown arm, a missing member and an extra
member. Scope: FR-323-AC-39 to FR-323-AC-46 and FR-433-AC-9.

## Test Procedure

1. Write the row of an agreeing `input`-arm function replay and of a
   disagreeing `witness`-arm one; read rows with `arm` `other`, with no
   `settlement`, and with an extra member.
2. Write the row of an observed-temporal replay of a `violated` packet at
   position 2, and of the same replay against a packet claiming position 1.
3. Write the row of a reproduced state-graph trap replay, and of one whose
   closure stops at `max_states` 1.
4. Write the row of a reproduced protocol-analysis replay, and of one whose
   walk leaves its case.
5. Write the row of a protocol-analysis replay whose walk takes a step that
   is not a successor of node N's `enter` port, and of one at its `exit`
   port; read a `step-not-successor` disagreement without `port`, a port
   reference with `port` `inside`, with `node` removed or with an extra
   member, and a `wrong-branch` disagreement carrying `port`.
6. Write the row of an observed-temporal replay of an `undefined` packet at
   position 3 whose atom at formula node 7 divides by zero; read it with
   `reason` removed and with `undefined_locus` `"7"`.
7. Write the row of an observed-temporal replay stopped by `max_window_values`
   2, and the `max_states` 1 row of step 3; read each with `value` a JSON
   number and with `setting` removed.
8. Write the row of an observed-temporal replay that observes formula node 4
   undefined where the packet expected a formula verdict; read it with
   `actual_locus` `"4"` and with `outcome.position` `"3"`.

## Expected Results

- Step 1: the `input` row with `record` `null` and no `disagreement`; the
  `witness` row `inconclusive` with `disagreement.kind` `verdicts`; the three
  reads refused.
- Step 2: `reproduced-with-evaluated-witness`, `outcome.kind` `violated`;
  then `inconclusive` with `parity`.
- Step 3: `reproduced-with-evaluated-witness` with its predicates; then
  `incomplete` with `limit` `max_states`.
- Step 4: `reproduced-with-evaluated-witness`, `category` `success`; then
  `inconclusive` with `disagreement.reason` `wrong-branch`.
- Step 5: `inconclusive` with `disagreement` `{"kind": "walk", "reason":
  "step-not-successor", "port": {"node": <N>, "port": "enter"}}`, then the
  same with `"exit"`; the five reads refused.
- Step 6: `outcome` `{"kind": "undefined", "position": 3, "reason": R}`, R
  the packet's undefined-evaluation `cause`, no `counterexample`,
  `undefined_locus` `7`; both reads refused.
- Step 7: `outcome` `{"kind": "window-limit-reached", "limit":
  "max_window_values", "value": "2", "setting": "monitor.max_window_values"}`
  and `limit` `{"limit": "max_states", "value": "1", "setting":
  "model_check.max_states"}`; the four reads refused.
- Step 8: `parity.actual_locus` `4`, `undefined_locus` `4` and
  `outcome.position` a JSON number; both reads refused.
