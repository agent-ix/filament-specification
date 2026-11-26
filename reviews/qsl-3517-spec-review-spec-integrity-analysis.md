---
id: SR-20261126060257459121766
title: "Integrity analysis: PR 860 State locus prose"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-393-state-graph-evidence.md, spec/objects/interfaces/FR-400-hyper-counterexample.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/objects/interfaces/FR-413-probabilistic-counterexample.md, spec/functional/temporal/FR-364-replay-temporal-counterexamples.md, proposals/backend-provider-v1/schema.json"
review_set: subset
---

## Summary

Ticket: QSL-3517 (PR agent-ix/filament-specification#860, head 09fd42d2a3abaa6f62f3f16f1f06b4a5f12c1769). Consistency of the changed wording with the provider schema and FR-331. The State locus wording agrees. One older ambiguity remains in the rewritten HyperStep paragraph.

## Verdict

**CONDITIONAL** - one low finding, older than this PR.

Observations outside the changed files, with no failing run observed there, so not findings:

- FR-364 "A model-arm step is `{transition, post_state}`" calls `post_state` "the state key of the step's post-state"; FR-433 says the same for protocol steps; FR-331 calls the hyper tuple's `pre_state` and `post_state` "the actual model StateKeys". The schema types each as State. None shows a bare string.
- FR-413 gives a scheduler entry a "product state key". `ProbabilisticCounterexample.scheduler.entries[].state` is `$ref State`, whose description is the FR-181 model `StateKey` octets. The `{key}` shape can carry either; which octets it holds is stated two ways. `Where.product_state` reuses the State shape with a description that names the product key; the scheduler entry has no such description.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | HyperStep text can be read as a step whose `transition` is `{"type":"terminal-stutter"}` with a `post_state`; the schema rejects that and admits only `{"type":"terminal-stutter","mark"?}` as the whole step. Older than this PR. | spec/objects/interfaces/FR-400-hyper-counterexample.md:37-44 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 7a25c8b227c678c4f40499831d763d3b578189ac the terminal stutter step is stated as the whole object, with no transition or post_state member, matching HyperStep. |
