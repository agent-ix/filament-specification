---
id: TC-1043
title: "Late-data cutoff rules and their separation from the history cutoff"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-094
    type: verifies
---
# TC-1043: Late-data cutoff rules and their separation from the history cutoff

## Description

Verifies the allowed-lateness and fixed-instant late-data cutoff rules, the
delayed establishment of a window under a cutoff, the exclusion and reporting
of late observations, and the independence of the late-data and history
cutoffs (FR-094 "Late-data cutoff").

## Test Procedure

1. Under `filament.temporal.late-data.allowed-lateness/v1` with `d` 2, evaluate
   a clause whose window has deadline 5 at watermarks 6 and 7, admitting an
   observation stamped 4 at arrival watermark 6 and one stamped 3 at arrival
   watermark 6, and one with interval stamp `[2, 4]` at arrival watermark 6.
2. Under `filament.temporal.late-data.fixed-instant/v1` with `C` 5, admit
   observations stamped 3 and 6 at arrival watermark 6.
3. Select a late-data cutoff and a history cutoff together and change each.
4. Recompute by revisioned repair admitting the late observation.
5. Repeat the late arrival of step 1 with no late-data cutoff into a window
   asserted complete.

## Expected Results

1. The window is not established at watermark 6 and is established at 7; the
   observations stamped 4 and `[2, 4]` are `on-time` and enter the evaluation;
   the one stamped 3 is `late`, is excluded, is reported beside the result with
   its identity, and the result is unchanged by it (FR-094-AC-12).
2. Stamp 3 is `late` and stamp 6 is `on-time` (FR-094-AC-12).
3. Each change alters only its own key and the result identity; no late
   observation makes history missing and the history cutoff classifies nothing
   `late` (FR-094-AC-13).
4. A new result links the prior result and names the late observation
   (FR-094-AC-13).
5. A completeness contradiction, with no `late` classification
   (FR-094-AC-13).
