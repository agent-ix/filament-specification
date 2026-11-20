---
id: SR-20261120013403001
title: "Spec review: QSL-2240 FR-364 task-trace step alphabet"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #763 head c6c4511032ada43622adb105ad509914e2300d84; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md"
review_set: base
relationships:
  - { target: "ix://agent-ix/filament-specification/FR-364", type: references }
---

## Summary

Ticket: QSL-2240. Base spec review of the FR-364 additions "Step alphabet",
"Synchronized source moves", "Comparison with the re-derived step", and
FR-364-AC-23 and AC-24, against the Linear arch ruling on QSL-2240, FR-331's
`TransitionIdentity`/`TaskTransitionIdentity`/`ModelStep` schema
(`proposals/backend-provider-v1/schema.json`), FR-419, FR-482, FR-425 and the
existing FR-364-AC-17 to AC-22. Integrity and consistency checks were folded
into this base pass; no separate analysis lens was run.

## Verdict

**PASS (mergeable).** The text matches the ruling: no new wire, the subject
fixes the alphabet, wrong-alphabet steps refuse
`invalid_runtime_input`/`invalid-value` at `/transition/type`, the activation
group is in FR-482 task-name order, and the member comparison order equals the
schema's `TaskTransitionIdentity` member order. The new rules agree with the
existing AC-20 and AC-21 pointers (static `hi`-task check gives AC-21's
`/prefix/13/transition/task`; the policy-order fallback gives its
`/prefix/11/transition/job`). AC-23/AC-24 are new ids with no collision on
`origin/main` or open PRs. `quire validate` passes. No high findings; the
medium and low findings below are clarifications.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium | The comparison rule derives each task step "at the step's delayed pre-state", but for the second and later `activate` steps of one synchronized move the preceding step already carries the whole move's post-state, where that activation is no longer enabled; the text does not say those steps are derived from the move's pre-state as the move's next activation. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:316-319 |
| FND-002 | medium | AC-23's `tick` model is not defined enough to build: the task set, task parameters, policy and the refuted claim and continuation are not given, and "with the second removed it refuses at `/prefix/1/transition`" needs a step at index 1 after removal. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:526 |
| FND-003 | low | Delay checks (allowance, group delay 0, urgent `budget-change`) refuse at `/delay` but are not placed in the four-check order, so a step with both a delay and a member fault has no stated first refusal. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:316-334 |
| FND-004 | low | AC-23's source-projection case says only "is refused" with no pointer; FSL FR-237-AC-11 states it as the first such step's `/transition/type`. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:526 |
| FND-005 | low | AC-23/AC-24 hard-code `/prefix/0` as `activate t1#0` released by `M::Call::fire_t1`; FSL FR-249-AC-8/9 names neither the operations nor the order of the three time-0 activations (the names exist only in the coder's test fixture). | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:526-527 |
| FND-006 | low | The new text cites FR-425 but the frontmatter has no relationship to it. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:285 |
