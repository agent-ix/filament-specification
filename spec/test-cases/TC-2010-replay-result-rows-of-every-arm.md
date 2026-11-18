---
id: TC-2010
title: "A replay results row is spelled for every replay-result arm"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-323
    type: verifies
---

# TC-2010: A replay results row is spelled for every replay-result arm

## Description

Verify that each of the five FSL FR-072 replay-result arms writes its
`results` row with exactly the members FR-323 "Replay result rows" gives it,
and that a reader refuses an unknown arm, a missing member and an extra
member. Scope: FR-323-AC-39 to FR-323-AC-42.

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
