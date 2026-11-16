---
id: SR-2026111610291807
title: "Criterion strength review of the FSL wave 6 spec readiness batch, standard side"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/filament-specification; PR #663 branch fsl-spec-w6 at bb47382e3aaf5a4a1d790e3007b8f9c371c46435: FR-352-AC-19, FR-364-AC-16"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: reviews
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
---

## Summary

Ticket: QSL-2296 (wave 6 batch). Jev is not installed on this host (`jev not found`), so the Jev-calibrated judgement was skipped and the criteria were judged by hand.

- **FR-352-AC-19** fails if:
  - the default is omitted;
  - the value is not echoed;
  - `limits` appears on a `finite` record;
  - the reader admits any malformed form: `limits` missing on a `model` record, an unknown name, `"0"`, `"01"` or a number.
- **FR-364-AC-16** fails if:
  - equal samples collapse to one step;
  - a mixed, numeric, non-canonical or out-of-range step is admitted;
  - a loop off the request's loop positions is admitted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS.
