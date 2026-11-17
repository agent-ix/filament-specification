---
id: TC-1901
title: "Window-aggregate threshold delimitation and type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-784
    type: verifies
---

# TC-1901: Window-aggregate threshold delimitation and type

## Description

Verify that a window-aggregate threshold ends at `and`, `or` and `implies`,
that an aggregate atom may stand in any temporal operand position but not
inside `holds(...)`, and that a parenthesized threshold is typed against the
aggregate's value. Scope: FR-784-AC-1, FR-784-AC-2, FR-784-AC-3, FR-784-AC-4.

## Test Procedure

1. Parse the `or` and `implies` formulas of FR-784-AC-1, and the two formulas
   with a bare value operand after the connective.
2. Parse the middle-operand conjunction and the `always` operand of
   FR-784-AC-2.
3. Check the two parenthesized thresholds of FR-784-AC-3.
4. Parse `always holds(count where holds(s.ok) over past[0,1] >= 1)`.

## Expected Results

- Step 1: one disjunction and one implication with the atom on the left; the
  other two refuse `invalid_syntax`/`unexpected-token` at the `s` after the
  connective.
- Step 2: both parse with the atom where written.
- Step 3: `(s.k + 1)` checks; `(s.ok and s.done)` refuses
  `ill_typed`/`type-mismatch` at the parenthesized threshold.
- Step 4: `invalid_syntax`/`unexpected-token` at `where`.
