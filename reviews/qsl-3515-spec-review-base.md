---
id: SR-202611210022442313144108575
title: "Base checklist review of the policy instance criteria"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #863 at 4e8b52206ba1625382619bc384452895778090a4; spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md, spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md, spec/test-cases/TC-1953-plan-policies.md, spec/test-cases/TC-1955-plan-request-and-outcome-wire.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/filament-specification/FR-803, type: references }
---

## Summary

Ticket: QSL-3515. Checked ids, criteria-to-test mapping, hand values and coverage of the new statements. TC-1953 and TC-1955 scopes list the new criteria. No hash or pin antipattern added.

## Verdict

Changes needed: one medium and one low coverage finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No criterion has a clause with two or more instances, or a key only a step creates. An implementation that monitors only the first admitted key passes AC-1 to AC-7 and reports optimal for a plan that breaks the policy at the second object. Add a two-object criterion (FSL FR-1012-AC-8 has one: a at 0, b at 4, always value != 4, goal true, domain Int[0, 0], infeasible) and its TC-1953 step. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:54-64 |
| FND-002 | low | 'It SHALL decide this after every admission refusal' has no criterion. With the counters population empty and policy ['NotFour', 'Nowhere'], an implementation that settles no-instance at the first clause, never refusing missing-name at /policy/1, passes every criterion. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:67-69 |
