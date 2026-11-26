---
id: SR-202611210022442313144108580
title: "Integrity review of plan refusal categories and exits"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #864 at 54432e37db9ca98123bcb8e12a1af082034ee644; spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md, spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md, spec/functional/tooling/FR-301-expose-complete-cli.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/filament-specification/FR-806, type: references }
---

## Summary

Ticket: QSL-3515. Checked one-category-per-code across FR-806, FR-805, FR-301, FR-823, FR-824, TC-413, TC-2037, FR-339 and FSL origin/main 2e960ceeed (FR-285, Code::category, read not run). The two incomplete_population rows, FR-805's table, FR-803-AC-11/AC-12 and both test cases agree on 22. FSL note, not a finding on this PR: FSL exits an unbounded operation parameter 20 (FR-663, TC-1746) while its code map gives an unbounded domain 21 (read, not run).

## Verdict

Changes needed: one high finding, which needs an arch decision, and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | 'The other codes these tables name are refusals, exit 20' includes unsupported_projection and unsupported_construct, whose category is unsupported, exit 21: TC-413 SV-04 and SV-10 give unsupported_construct/not-yet-implemented exit 21 and FR-806-AC-6 gives the same code and cause exit 20; FR-339-AC-7 and TC-2040 also use 21; FSL's code catalogue maps both codes to 21 (FSL FR-285, read). So one code keeps two exits by route, which the ruling removes. A planner that exits by code gives 21 for TC-2037's conflict and for an unbounded domain and fails AC-6 and TC-1956 step 1. The ruling as relayed also says other located refusals keep 20, so arch must say which holds: either state unsupported, exit 21 for the two codes in FR-806, AC-6, TC-1956 and TC-2037, or record why these routes are refusals. | spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md:37-41 |
| FND-002 | low | FR-806 points to FR-301 for a code's category, but FR-301 maps no code and its incomplete row does not cover an unestablished population. Add incomplete_population (and unavailable_observation) to the row or say where a code's category is owned. | spec/functional/tooling/FR-301-expose-complete-cli.md:67 |
