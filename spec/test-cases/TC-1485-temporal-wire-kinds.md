---
id: TC-1485
title: "The sixteen temporal disposition kinds of native-run-result/2"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
---

# TC-1485: The sixteen temporal disposition kinds of native-run-result/2

## Description

Reads `native-run-result/2` documents carrying each `disposition.temporal`
kind, with emphasis on the pending categories and the finite-window and
progress rows, and refuses unregistered kinds and misplaced categories.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| TK-01 | One evaluate-stage document per kind of the sixteen rows | Each reads with exactly its row's members, `category`, `truth`, `basis` and `outcome`, and reads back equal | FR-352-AC-10 |
| TK-02 | A `pending` document | Category `pending`, no `truth`, `basis` `unsettled` | FR-352-AC-11 |
| TK-03 | `pending-window` (position 0, deadline 5); `pending-next-event` (position 3); `indeterminate` with each reason kind; `activation-incomplete` at position 1; `progress-limit-reached` (`node count`, 4, `progress.covered_positions`); watermark `progress-contradicted` (100 retained, 60 asserted) | Each reads with its row's category, basis and outcome and reads back equal | FR-352-AC-16 |
| TK-04 | Independently: kind `pending-forever` and `held`; category `pending` on `incomplete`; `indeterminate` on `violated`; `inconclusive` on `pending`; `pending-window` without `deadline`; `missing-valuation` with `instance` and no `also_held_by`; an `outcome` on `activation-incomplete` and on `progress-limit-reached`; `setting` `progress.positions`; contradiction `fields` differing from members | Each refuses with a located wire diagnostic naming the member | FR-352-AC-17 |

## Expected Results

A strict reader admits exactly the sixteen registered kinds, each with its
row's category.
