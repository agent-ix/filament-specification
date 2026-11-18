---
id: TC-2014
title: "A comparison separating witness record names the comparison, its operands and its path"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-351
    type: verifies
---

# TC-2014: A comparison separating witness record names the comparison, its operands and its path

## Description

Verify the comparison form of the separating witness record and its strict
reader. Scope: FR-351-AC-9, FR-351-AC-10 and FR-352-AC-21.

## Test Procedure

1. Settle the state clause of FR-351-AC-9 over the observation where
   `self.version` is 600 and its parent's is 0, and read its record.
2. Compare that record with one equal except for its right operand.
3. Hand the strict reader the record of step 1 and each defective variant
   FR-351-AC-10 lists.

## Expected Results

- Step 1: basis `decisive-counterexample`; the record names the `=`'s
  occurrence key, operands `[600, 0]`, the decision path `implies` then `=`,
  deciding element `false`, a value path rooted at `self` with empty steps,
  and no quantifier, index or trace position.
- Step 2: the records compare unequal.
- Step 3: the reader admits step 1's record and refuses every variant.
