---
id: SR-2026111610291805
title: "Integrity review of the FSL wave 6 spec readiness batch, standard side"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #663 branch fsl-spec-w6 at bb47382e3aaf5a4a1d790e3007b8f9c371c46435, git diff origin/main...HEAD (FR-352 limits member and FR-352-AC-19, FR-364 sample steps and FR-364-AC-16, FR-200 Ubiquitous Language table, TC-1700, TC-1701)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: reviews
  - target: ix://agent-ix/filament-specification/FR-364
    type: reviews
  - target: ix://agent-ix/filament-specification/FR-200
    type: reviews
---

## Summary

Ticket: QSL-2296 (wave 6 batch; standard side of QSL-2296 item 1, QSL-2303 and QSL-2297). One combined review with filament-spec-language#2747.

- **Ids.** TC-1700 and TC-1701 are new and do not collide with origin/main (aae8a4a84). No FR in FR-690..FR-699 was needed.
- **Validation.** `quire validate --scope . 'spec/**/*.md'` exits 0, so the FR-200 fix works. Its Ubiquitous Language bullets became a table with all six terms and definitions kept word for word.
- **FR-352 `limits`.** This agrees with language FR-267 and FR-641.
- **FR-364 `{sample}`.** This agrees with language FR-330.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The undefined-evaluation `where` table gains a `sample` key, but no AC covers an observed-arm envelope that carries `sample`. FR-364-AC-10 covers only `document`, and AC-16 covers only steps. Fix: extend FR-364-AC-16 with an envelope whose `where` carries `position`, `sample` and `locus` (admitted), and one carrying both `sample` and `document` (refused). | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:254; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:395 |
| FND-002 | low | The `limits` presence rule says "present exactly when at least one request-settable limit bounds the run's evaluation, today a run over a `model` trace". The reader's refusals, two sentences later, are keyed to the `model` trace alone. "today" makes a wire rule depend on the current set of limits, and the two sentences disagree once another limit is added. Fix: state it as "present exactly for a run over a `model` trace" and drop "today". | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:145-147 |

## Verdict

PASS WITH TWO LOWS. Nothing was dropped or narrowed, and the FR-200 edit is format-only.
