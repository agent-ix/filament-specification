---
id: SR-202611210022442313144108574
title: "Integrity review of per-instance policy monitors"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #863 at 4e8b52206ba1625382619bc384452895778090a4; spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md, spec/functional/analysis/FR-801-bound-and-meter-the-plan-search.md, spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/filament-specification/FR-803, type: references }
---

## Summary

Ticket: QSL-3515. Checked the new per-instance monitor rule against FR-801 (search node, charge order), FR-650 (certificate key), FR-804 and the six planning fixtures. O1 KeepServing has five instances and O5 Arrive has two.

## Verdict

Changes needed: one medium finding. Statuses, cause and exit tables agree across FR-650, FR-803 and FR-805.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Each instance has its own monitor or automaton', but FR-801 still keys a node by one monitor state per named clause and charges one max_work per 'policy monitor or automaton update, in the request's policy order'. With two counters under NotFour, or O5's Arrive over two robots, one implementation charges two units per step and another one (FSL plan_search.rs charges one per clause), and the instance order and early stop at a rejecting monitor are unstated. explored.work then differs, and under a max_work limit one reports incomplete where the other reports optimal. Say in FR-801 or here whether the charge is per clause or per instance, and in what instance order. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:63 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | 'in FR-361's instance order' is not defined for a plan: FR-361 orders by first occurrence in each population's universe list, and this round removed the sentence making the snapshot's listed keys that universe. With b listed before a in the snapshot, one implementation reads b first and another a (key order), so the early stop charges the evaluation meter differently and an accounting limit can stop one run and not the other. State the order of a population's keys for a plan. | spec/functional/analysis/FR-803-hold-plans-to-policy-clauses.md:80-83 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | a4b3e72a: FR-801 charges one max_work per policy clause and states the node's instance states and the initial reading; matches FSL plan_search.rs. |
| FND-002 | fixed | 4f36fe84: each population's universe is the snapshot's listed keys in list order, so FR-361's instance order is defined for a plan. |
