---
id: SR-2026112113535901
title: "Base specification review — native invocation outcomes, receivers, shared arguments and hybrid step types"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #786 at d932bae2bea85d8d2e16ed50ce4337cb4ca9857b against base dd10962a01fe7314c95c0cba510fe2f029aa3b96; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md, spec/functional/analysis/FR-801-bound-and-meter-the-plan-search.md, spec/functional/analysis/FR-804-order-plans-deterministically.md, spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md, spec/objects/interfaces/FR-322-checked-package-artifact.md (Plan-record type; FR-322-AC-513..515), spec/test-cases/TC-1950, TC-1951, TC-1954, TC-1955, TC-2037"
review_set: base
---

## Summary

Ticket: QSL-1978. Initial review of PR #786 (its own review; not a fix round
of PR #783). The PR settles PR #783's two open disposition items, shared
argument collisions (former FND-006) and hybrid step field types (former
FND-007). It also defines native invocation identity: one work unit per
operation/receiver/arguments invocation, a canonical `(result, post-state)`
outcome order with zero-based `outcome_ordinal`, a typed receiver carried
through a generated `Reference<T>` union, and Float32/Float64 `totalOrder`
result ordering.

The accounting arithmetic is internally consistent. FR-801-AC-9 `{states: 4,
work: 5}`, AC-10 `{1, 1}`, TC-1951 step 9 `{1, 2}` and the unchanged
FR-801-AC-8 counter counts all follow the new charge order. The Float32/Float64
`totalOrder` vectors agree with IEEE 754 totalOrder and with FR-181's
bit-preserving float encoding. The shared-arguments rule follows the
architecture decision (equal static type and presence, otherwise refuse
before search) and the per-operation union stays out of scope.

Several rejection criteria are not testable as written. Part of the
generated typing is still unpinned, and the PR does not reconcile some
existing owners.

## Verdict

**FAIL — not mergeable as written.** There is one high finding, five medium
and two low. The high finding: the receiver/ordinal rejection criteria depend
on an "existing replay and certificate verification entry surface" for
supplied step data, and the specification defines no such surface.

What is right: the work-unit charge position comes before the precondition,
so disabled invocations are charged. Rejected raw trials charge evaluation
accounting only. No ordinal is assigned from a partial outcome list.
Invocation order is operation, then receiver, then arguments; the outcome
order is result, then post-state key bytes. Ordinals are present only for a
multi-outcome invocation, and the singleton/out-of-range refusals are stated.
The receiver union reuses FR-440's existing form and adds no new tag. The
hybrid mode enum is in declaration order with `ordered: false`, and
continuous trajectories are explicitly not enumerated.

## Review notes

The method is the base spec-review over the PR diff only, with the cited
owners (FR-181, FR-440, FR-481, FR-650, FR-806, FR-819, FR-151, FR-154) read
for context. This is normative spec evidence only. No build, runtime test or
full gate was run, and none is claimed. `quire validate --scope .` over the
ten changed files exited 0. Its only diagnostics were EARS advisories on
FR-804 line 19 and FR-322 line 112, Description statements outside the diff
hunks, so they are pre-existing and were not pursued.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The receiver/ordinal rejection criteria rely on an "existing replay and certificate verification entry surface" that accepts supplied, edited step data. No such surface exists. FR-650 defines only two library seams: plan substitution, which takes a caller-given sequence of invocations, and certificate tamper, which removes one successor. FR-650 forbids reaching either seam from a request, and the wire `certificate` is a summary (`type`, `entries`, `outcome`). Nothing defines where to supply a receiver, an `outcome_ordinal` of `-1`, `0.5` or `"0"`, or a retained post-state. Nothing says what reads JSON-kind defects in a step, what the locus is, or what the failure is (replay method failure exit 30, certificate rejection rule/entry, or a refusal). FR-800-AC-6, FR-805-AC-9/10, FR-322-AC-515, TC-1950 step 6, TC-1955 step 10 and TC-2037 step 8 therefore have no executable oracle. Extend FR-650's plan-substitution seam, and the checker inputs, to carry the receiver, the ordinal and the recorded post-state, and state the exact failure contract per defect. | spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md:163-180, :257-258; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md:132; spec/objects/interfaces/FR-322-checked-package-artifact.md:6684; spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md:141-158 | wrong-requirement |
| FND-002 | medium | The shared-arguments disagreement "reports typed `unsupported` before search" but names no code, cause, carrier, stage or locus. FR-806 owns the plan refusals and gains no row. FR-805's `unsupported` outcome status carries only FR-193's fixed `unsupported_projection`/`unsupported-requested-capability` cause, with no member that names a parameter and two operations. Two readings therefore diverge: a check-time S3 refusal (exit 20, no plan request read, FR-806 table) or an outcome document with `status: unsupported` (exit 21). It is also unstated whether the checked package, whose synthesis candidate must reference a generated plan-record type, is produced at all. Which "both operations" are named when three or more operations conflict is undefined. FR-322-AC-513 and TC-2037 step 6 cannot distinguish the readings. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6115-6122, :6682; spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md:36-49; spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md:204-206 | wrong-requirement |
| FND-003 | medium | The counter fixture's operations are receiver-bearing: `pre CanInc ... on M::Counter::inc { self.value < 20 }`, with one live counter `c`. The new FR-805 rule says a step "carries `receiver` exactly when its operation has a receiver", but FR-805's normative outcome document example still writes `{"index": 0, "operation": "inc", "arguments": []}` with no `receiver`. The receiver-union step field also now changes the generated plan-record type of every existing receiver-bearing fixture (the counter, FR-800-AC-1's `drain`), and no existing criterion or example was updated. Either update the example and state that the counter steps carry `receiver` `c`, or state which operations count as receiver-bearing. | spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md:118-130, :159-162; proposals/planning-v1/fixtures/counter/unit.native:5-8 | wrong-requirement |
| FND-004 | medium | Outcome identity includes the operation result, but no step record, wire step or FR-181 successor identity carries the result. FR-181 makes two successors with equal transition identity and equal post-state key one successor. FR-804 keeps `(0, S0)` and `(1, S0)` as two outcomes with two ordinals, and TC-1954 step 6 builds exactly that pair. For two outcomes that share a post-state, editing the ordinal between them passes the stated check ("bind the selected ordinal to the recorded post-state"). That contradicts FR-804 ("changing a receiver or an ordinal cannot silently select another edge") and FR-800-AC-6. It also leaves native replay, which runs through FR-181's runtime, unable to rederive the planner's outcome list. Reconcile with FR-181, or carry or bind the result. | spec/functional/analysis/FR-804-order-plans-deterministically.md:60-87; spec/functional/runtime/FR-181-simulate-finite-executions.md:296-300; spec/test-cases/TC-1954-canonical-plan-order.md:28-31 | wrong-requirement |
| FND-005 | medium | Former FND-007 is only partly resolved. Hybrid `delay` and plan `elapsed` are typed "the exact model-time-unit quantity type", but no such type node exists for a hybrid model. FR-481's hybrid nodes declare no time unit, its continuous variables are `scalar_type`/`rational`, and FR-805-AC-8's `dwell_grid` is a unitless `rational`. One producer will emit `scalar_type`/`rational`, another a `Quantity<u>` node with an invented unit, and FR-322-AC-514's "share its exact model-unit quantity type" cannot be checked. Name the exact node. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6163-6170, :6683; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:48-56; spec/functional/analysis/FR-819-plan-over-hybrid-models.md:49-58 | wrong-requirement |
| FND-006 | medium | Hybrid `initial`, `before`, `after` and `final` are typed as one point-valued hybrid-state record (the mode plus one value per variable). FR-819, however, plans from an initial set, and its execution is every trajectory from that set under every value of a nondeterministic parameter. With an interval initial set or a nondeterministic parameter, a state after a dwell is an enclosure, not a value, so the record cannot hold it. Yet the same paragraph says these records "represent schedules and enclosures". State which value the point fields hold, or restrict this typing to point-initial deterministic models. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6163-6175; spec/functional/analysis/FR-819-plan-over-hybrid-models.md:32-34, :56-58 | missing-requirement |
| FND-007 | low | The shared-arguments rule requires both "static type and declared presence" to agree. FR-154-AC-52 and FR-151, however, make an `optional` parameter of `X` and a `required` parameter of an option value type over `X` "one type" (`Option<X>`). Two operations spelling one type these two ways would be refused, against the existing equivalence. TC-2037 step 6 also says it introduces "not a new source spelling for an optional operation parameter", although FR-154 already defines `presence: optional`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6115-6118; spec/functional/type-model/FR-154-admit-domain-package-model.md:987; spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:92 | wrong-requirement |
| FND-008 | low | Certificate-side accounting is not reconciled with multi-outcome invocations. FR-650's successor records "nam[e] the step" without the receiver or ordinal now required for step identity. `max_check_work` still charges "one per successor it re-derives", while search work moved to one per invocation, so it is ambiguous whether the checker charges per invocation or per outcome. No criterion pins the check work. | spec/functional/analysis/FR-801-bound-and-meter-the-plan-search.md:59; spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md:104-107 | missing-requirement |
