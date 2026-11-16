---
id: TC-1741
title: "Every admitted registration negotiates through the one item entry"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-711
    type: verifies
---

# TC-1741: Every admitted registration negotiates through the one item entry

## Description

Check that linked engines, process providers and Kani backends all settle
items through `negotiate_*`'s item entry with the generic arm, and that a
backend-specific form is required only where the candidate's arm reads it.
Scope: FR-711-AC-1 to FR-711-AC-5; AC-5 is a driver test.

## Test Procedure

1. Settle bounded and unbounded items on a linked `en-1` advertising both modes.
2. Settle each advertised-mode row on a bounded-only linked engine and on a process provider with the same manifest.
3. Settle an item without a backend-specific form on linked, process and Kani candidates.
4. Settle a state-graph item on a linked candidate with and without its form listed.
5. Through the driver, compare `analyze`'s dispositions with the entry's.

## Expected Results

- Step 1: `supported` naming `en-1`; never `unknown-backend`.
- Step 2: each row's listed disposition, equal for the two candidates.
- Step 3: settles on linked and process; `inconsistent-candidates` on the Kani candidate.
- Step 4: `supported`; then `unsupported` `unsupported-requested-capability`.
- Step 5: equal dispositions.
