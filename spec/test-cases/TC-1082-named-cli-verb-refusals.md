---
id: TC-1082
title: "Named CLI verb refusals"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-522
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-301
    type: verifies
---

# TC-1082: Named CLI verb refusals

## Description

Invokes each verb a build accepts without routing it. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| NR-01 | Each unrouted verb in machine and text mode | Outcome document with that operation, `unsupported`, `unsupported_operation`/`operation-not-routed` naming the verb, exit `21`; the named text line | FR-522-AC-1, FR-301-AC-4 |
| NR-02 | Every verb ending non-zero with an open stdout | Machine output is never empty | FR-522-AC-2, FR-301-AC-4 |

## Expected Results

No verb exits without a named outcome.
