---
id: TC-1974
title: "Meaning of target results over an export"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-815
    type: verifies
---

# TC-1974: Meaning of target results over an export

## Description

Verify the evidence a target proof, model check or counterexample gives, by disposition. Scope: FR-815-AC-1 to FR-815-AC-5.

## Test Procedure

1. Record a Lean proof of `Ob_Low`.
2. Record a constrained and a complete TLC run.
3. Record a proof over a conditional and over an unrepresented record.
4. Read back a TLC counterexample and an edited one.
5. Apply a Lean proof to a changed package.

## Expected Results

- Step 1: `proved`, `trusted`, `unbounded`, never `certified`.
- Step 2: `bounded` naming the constraint; `unbounded`.
- Step 3: the condition as an assumption; no entry.
- Step 4: `refuted` after replay; nothing for the edited trace.
- Step 5: inapplicable.
