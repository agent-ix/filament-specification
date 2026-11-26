---
id: SR-202611210022442313144108579
title: "Base checklist review of the plan refusal exits"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #864 at 54432e37db9ca98123bcb8e12a1af082034ee644; spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md, spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md, spec/test-cases/TC-1953-plan-policies.md, spec/test-cases/TC-1956-plan-refusals.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/filament-specification/FR-806, type: references }
---

## Summary

Ticket: QSL-3515. Checked the new exits against the criteria and test cases. Hand values are correct: complete:false gives incomplete_population/incomplete-scope, exit 22; counters omitted with Nowhere gives missing_declaration/missing-name, exit 20; counters omitted with NotFour gives incomplete_population/missing-required-artifact, exit 22. FR-806-AC-8 is in TC-1956 scope and step 7. quire validate on the five spec files reports one unrelated module warning; cargo test --test acceptance_criterion_ids passes 2 of 2. EARS analysis skipped: no requirement statement changed.

## Verdict

Changes needed: one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Step 4 requests 'an inadmissible initial snapshot' and the expected result is exit 20, but FR-806 now says an admission refusal exits as its own code and AC-4 fixes 20 only for the bound violation. A snapshot listing a population complete: false is inadmissible and exits 22, so a test built that way contradicts the expected result. Say 'a snapshot that violates the population bound'. | spec/test-cases/TC-1956-plan-refusals.md:22 |
