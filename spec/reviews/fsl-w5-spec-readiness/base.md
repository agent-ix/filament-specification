---
id: SR-2026111608544811
title: "Spec review of the FSL wave 5 standard-side hyper clock rows"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #662 branch fsl-spec-w5 at 73e40eb6b, git diff origin/main...HEAD (FR-322 Hyper clause rows and FR-322-AC-453, FR-402 body wrapper, TC-1680, proposals/checked-package-v2/schema.json)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: reviews
  - target: ix://agent-ix/filament-specification/FR-402
    type: reviews
---

## Summary

Ticket: QSL-2295 batch (the QSL-2277 standard side). Combined review with
agent-ix/filament-spec-language#2744. The change specifies one
`temporal_bindings` row per quantified trace of a hyper clause's body
wrapper. Each row carries a `trace` member, and the reader has refusals for
a missing, unknown, repeated or misplaced trace. The schema gains the optional
`trace` member. Nothing is narrowed. Ids: FR-322-AC-453 is the next AC, and
TC-1680 is in the TC-1680..1699 range and absent from origin/main. No FR was
minted. `quire validate` exits 3 only on
spec/objects/foundation/FR-200-filament-assessment-domain.md
(`semantic.feature-not-extractable` at lines 246, 248 and 250). That file is
byte-identical to origin/main, last changed in #657, so the failure is
pre-existing. No changed file reports an error.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new rows contradict FR-322's existing reader rules, which were not amended. The general reader paragraph refuses a repeated (`declaration`, `occurrence`) key `conflicting-definition`, orders rows by declaration and occurrence only, and "requires exactly one row for every" `claim` occurrence. FR-322-AC-294 ("exactly one `temporal_bindings` row for each of its `claim` occurrences") and FR-322-AC-364 ("a second binding row with one (`declaration`, `occurrence`) key refuses") say the same. A conforming hyper package with rows `s` and `n` for one occurrence is therefore refused by the existing text. Fix: amend the general paragraph, AC-294 and AC-364. The key is (`declaration`, `occurrence`) for rows that are not wrapper rows and (`declaration`, `occurrence`, `trace`) for wrapper rows. Order is by trace position within an occurrence. Coverage is one row per trace of each wrapper occurrence. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3871,3876,4971,5041 |

## Verdict

REQUEST CHANGES on FND-001.

## Dispositions

Round 2, verified at head 0e7b1ef35.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-322 key paragraph, the reader order, repeat and coverage paragraph, AC-294 and AC-364 now key wrapper rows by (`declaration`, `occurrence`, `trace`). They order wrapper rows by quantifier position, refuse an out-of-order trace at `/temporal_bindings/{i}/trace`, require one row per trace, and admit rows `s` and `n` of one occurrence. |
