---
id: SR-20261119181013
title: "spec review of QSL-2512 model-based test entry and Text work bound (FSL #3069, FSpec #748)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-spec-language; FR-752 Entry, Text charge bullet, FR-752-AC-10, FR-752-AC-11, FR-281 SimulationLimits, TC-3068 at bfc728b1c; agent-ix/filament-specification; FR-190 Resource bound, FR-190-AC-22, TC-1600 step 10/11 at 913a57b99"
review_set: subset
---

## Summary

Ticket: QSL-2512. The Text work bound fails the 2^40 fixture before allocation under both profiles and uses FR-461-AC-5's limit-reached shape, and the record/refusal split matches FSpec FR-190 "Refusals" and "Result". Four defects block merge: the entry signature cannot run a value clause, a setting name is invented, AC-11 contradicts FR-190 partition construction, and the normalizing-profile charge is not a pinned quantity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `run_model_based_test` takes a pre-built `&ModelSystem`, which exists only for a state model with initial snapshots (ModelSimulationRequest) and whose expansion limits and meter are fixed at construction; value clauses (AC-9, AC-10 `Huge`, `Square`) cannot be run through it, and the entry cannot give the system size-bounded domains as Behavior requires | spec/functional/FR-752-generate-model-based-tests-and-shrink-counterexamples.md:46, :111 |
| FND-002 | high | `simulation.max_steps` is not a setting in FR-255's closed setting table (FR-255-AC-21 refuses unknown settings), so a stop on it cannot name a raisable setting; FR-255:244-245 also limits explore.* to FR-101 exploration and simulation | spec/functional/FR-281-analyze-claims-with-in-process-engines.md:350, :429 |
| FND-003 | high | FR-752-AC-11 requires equal `cases` under `explore.states` 1 and the defaults, but FSpec FR-190 Partitions generates no partition case when the coverage exploration stops on a limit, so the by=6 precondition-false case disappears and seeded trace indices shift | spec/functional/FR-752-generate-model-based-tests-and-shrink-counterexamples.md:210 |
| FND-004 | high | The normalizing-profile charge is "one per length and transfer state", but FR-190 leaves the transfer-state count to the implementation; whether `Text[0, 4; nfc]` is `tested` under work_units 1000000 (FR-752-AC-10, FR-190-AC-22) depends on it, so charges are not deterministic across implementations | spec/functional/analysis/FR-190-generate-model-based-tests.md:282 |
| FND-005 | medium | `ModelBasedTestRequest.max_trace_length` has no stated type or absence rule; value-clause requests carry none and FR-753 PlanInputs uses `Option<u64>` | spec/functional/FR-752-generate-model-based-tests-and-shrink-counterexamples.md:53-54 |
| FND-006 | medium | TC-1600 Expected Results has `Step 11` with no procedure step 11; the AC-22 run was appended to step 10, which also lost its `(FR-190-AC-21)` reference and gained a stray " ." | spec/test-cases/TC-1600-model-based-test-traces-and-1-minimal-shrinking.md:31, :45 |
| FND-007 | low | AC-11 says `undetermined` "naming `explore.states`"; per FR-255-AC-13 the limit is `max_states` and the setting `explore.states`; AC-11 also does not name which of AC-7's clauses the plan step runs | spec/functional/FR-752-generate-model-based-tests-and-shrink-counterexamples.md:210 |

## Verdict

FSL #3069 not mergeable (FND-001 to FND-004). FSpec #748 not mergeable (FND-004, FND-006). Clean: the 2^40 fixture charge (2^40+1 cells or more against 10^6) fails before allocation with the stated counter and setting; the limit-reached shape matches FR-461-AC-5/AC-8; the never-narrow and never-unsupported rules match the QSL-2512 ruling class D; `AnalyzeLimits.simulation` explore.* defaults match FR-255; `admit_arguments` exists at fsl-eval/src/value/expression/mod.rs:1199. quire validate exits 0 in both worktrees. No id collisions with open PRs; #3067 touches FR-281 in a different hunk.
