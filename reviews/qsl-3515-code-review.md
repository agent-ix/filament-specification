---
id: SR-202611210022442313144108573
title: "Code review of the plan policy no-instance outcome"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; PR #863 at 4e8b52206ba1625382619bc384452895778090a4; spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md, spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md, spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md, spec/test-cases/TC-1953-plan-policies.md, spec/test-cases/TC-1955-plan-request-and-outcome-wire.md"
relationships:
  - { target: ix://agent-ix/filament-specification/FR-803, type: references }
---

## Summary

Ticket: QSL-3515. Checked the FSpec text against FR-361, FR-399, FR-650, FR-805, FR-819, FR-823, FR-824, FR-301, FR-331, the planning fixtures, and against FSL origin/main feaadbf807 (FR-125, FR-120, FR-1010, FR-1012, plan_search.rs, plan_source.rs, plan_request.rs, plan_synthesis.rs). No schema or vector file reads the plan status. The hand values of FR-803-AC-6, AC-7 and FR-805-AC-11 are correct. quire validate on the five files reports one unrelated module warning; cargo test --test acceptance_criterion_ids passes 2 of 2. FSL's exit_code and status_name matches are exhaustive, so a new status variant is a compile error there until handled.

## Verdict

Changes needed: one medium finding. The status, cause spelling, category and exit agree with FR-301 and FR-331.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The instance rule takes 'the keys it lists for each population' as that universe, and says nothing of a population overlapping T that the initial snapshot omits or lists with complete false. FR-361 makes a population with no universe requires-bound, never no-instance, and FSL FR-120 gives an omitted or incomplete row no universe. Such a snapshot with a policy over T can be read as no-instance (empty list), as instances from a partial list, or as requires-bound, which has no plan status; FSL's instances_with_meter returns RequiresBound there, and plan_source policy_stop turns that into a fault (read, not run). State the outcome, and that it precedes no-instance across all named clauses as FSL FR-125 orders it. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:59-64 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | spawn needs a key to create under, and nothing in AC-9, TC-1953 step 9, the plan request or the snapshot supplies one: elsewhere a created key comes from the subject's universe (FR-480), and FR-803 now reads only the keys of objects the snapshot holds. A planner whose universe is those keys, as FSL's is, gives spawn no outcome, so with domain Int[1, 1] it settles infeasible where AC-9 says optimal. State where a plan's creatable keys come from, and how many there are, since several give several outcomes and ordinals. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:137 |
| FND-003 | low | The two incomplete_population refusals depend on the policy list, yet FR-806, whose request-time table FR-650 calls the complete refusal set, gains no row and no order. With counters listed complete: false and policy ['NotFour', 'Nowhere'], one implementation refuses at /initial and another missing-name at /policy/1; both conform. Add the rows to FR-806 and say they follow the policy name and type refusals (or precede them). | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:70-79 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 9507f34f: a listed complete:false population and an omitted one each refuse at /initial before no-instance (AC-11, AC-12). |
