---
id: TC-1740
title: "An interrupt or termination signal cancels a running CLI verb"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-710
    type: verifies
---

# TC-1740: An interrupt or termination signal cancels a running CLI verb

## Description

Check that the first signal yields the verb's cancelled outcome with cause
`requested` and exit `22`, a second signal ends the process, and an earlier
deadline keeps its cause. Scope: FR-710-AC-1 to FR-710-AC-5, driver tests.

## Test Procedure

1. Send `SIGINT` to `analyze` after its first item settles.
2. Send `SIGTERM` to a long `run`.
3. Send `SIGINT` to `monitor` after two records with standard input held open.
4. Send two `SIGINT`s to a verb before its operation reaches its next charge.
5. Send `SIGINT` to a run already cancelled by its deadline, and after an outcome is written.

## Expected Results

- Step 1: one document; first item unchanged, second `incomplete` cause `requested`; exit `22`.
- Step 2: `cancelled`/`requested`, exit `22`.
- Step 3: summary within one second, open clauses `incomplete`, exit `22`.
- Step 4: the process ends by `SIGINT`'s default action.
- Step 5: cause `deadline`; the late signal changes nothing.
