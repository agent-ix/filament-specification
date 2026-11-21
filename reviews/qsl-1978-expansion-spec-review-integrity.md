---
id: SR-2026112113535902
title: "Integrity analysis — native invocation outcome criteria and test-case trace"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #786 at d932bae2bea85d8d2e16ed50ce4337cb4ca9857b against base dd10962a01fe7314c95c0cba510fe2f029aa3b96; new criteria FR-800-AC-6, FR-801-AC-9, FR-801-AC-10, FR-804-AC-5, FR-804-AC-6, FR-805-AC-9, FR-805-AC-10, FR-322-AC-513, FR-322-AC-514, FR-322-AC-515; TC-1950 steps 6-7, TC-1951 fixture and steps 8-9, TC-1954 steps 5-7, TC-1955 steps 9-10, TC-2037 steps 6-8 and relationships"
review_set: subset
---

## Summary

Ticket: QSL-1978. This is the integrity pass over the criteria and test cases
added by PR #786, covering completeness, consistency, atomicity and the
criterion-to-test trace. Every new criterion names a test case that holds a
matching procedure step and expected result. TC scope lines were updated to
the new criterion ranges. TC-2037 gains AC-level `verifies` edges for
FR-322-AC-513..515, matching its existing AC-level convention. TC-1950, 1951,
1954 and 1955 verify at FR level, as they did before.

## Verdict

**PASS with low findings.** There are three low findings and no medium or high
ones. The substantive soundness defects are recorded in the base review,
SR-2026112113535901, and are not repeated here.

Clean units: FR-801-AC-9 and AC-10 counts reconcile with the TC-1951 fixture
(the length domain `Int[1,1]` makes no initial goal evaluation; one charge for
`blocked` and one for `choose`; three admitted depth-1 nodes keyed by state
and step count). FR-804-AC-5 and TC-1954 step 5 agree on the 8-tuple order.
The FR-804-AC-6 float vectors match TC-1954 step 7's table. FR-322-AC-514
matches TC-2037 step 7.

## Review notes

The method is integrity analysis scoped to the diff's criteria, test cases and
relationship edges. It is spec text only. `quire validate --scope .` over the
ten changed files exited 0, with no diagnostics on changed lines.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | FR-322-AC-515's Verification cell cites `Test (TC-2037, TC-1955)`. TC-1955's relationships verify only FR-805, and its Description scope lists only FR-805-AC-1..10, so the TC-1955 half of the trace is not declared on the test side. Add the edge and scope entry, or drop TC-1955 from the cell. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6684; spec/test-cases/TC-1955-plan-request-and-outcome-wire.md:1-14 | correct-requirement-no-evidence |
| FND-002 | low | The evaluation-accounting half of FR-801-AC-10 and TC-1951 step 9 does not pin which `accounting.<counter>` is set, its bound, or the exact `limit-reached{limit, value, setting}` cause. The oracle is "names the denied accounting setting" with a self-calibrated budget, so any accounting counter satisfies it. That is weaker than the exact cause FR-801-AC-2 pins for the same mechanism. | spec/functional/analysis/FR-801-bound-and-meter-the-plan-search.md:214; spec/test-cases/TC-1951-plan-search-bounds-meters-and-completeness.md:43-47, :64-66 | wrong-requirement |
| FND-003 | low | Several fixtures are underspecified. TC-1951's invocation fixture never names the model, the grammar `G` or whether `v` is a direct attribute of `G`, yet TC-1950 step 6's goal `Reach.final.v = 2` reads `v` as a direct field. Under FR-322's state record, a population attribute is a sequence-element field, not a direct field. TC-1950 step 7's "receiverless invocation with one unchanged-state outcome" names no model or operation. | spec/test-cases/TC-1951-plan-search-bounds-meters-and-completeness.md:16-28; spec/test-cases/TC-1950-plan-steps-arguments-and-goal.md:23-31 | wrong-requirement |

## New findings (disposition pass 1)

Found while re-checking the fix commit at `2975a1885c64cd1b90d091dc0c432a226f32e5f5` (fix commit `2975a188` over the main merge `c6fb1499`). Numbering continues the `## Findings` sequence.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-004 | low | The nondeterministic-rate half of FR-819-AC-6 and TC-1966 step 6 has no definite oracle. 'Cannot be validated from a representative point' and 'not validated as feasible' name no status, cause or certificate. With `h` in [0,1], rate in [1,2] and `dwell_grid` 1, the guard `h >= 8` needs a dwell of at least 8, while the invariant `h <= 10` allows at most 4.5, so no schedule exists. The AC still admits `infeasible` with a closure certificate, `incomplete`, or any non-feasible answer. Pin the expected status and its certificate or cause. | spec/functional/analysis/FR-819-plan-over-hybrid-models.md:170; spec/test-cases/TC-1966-hybrid-plans.md:23-29, :38-45 | wrong-requirement |

## Dispositions

Disposition pass 1 at frozen head `2975a1885c64cd1b90d091dc0c432a226f32e5f5`. The fixes were inspected in fix commit `2975a188` (`git diff c6fb1499 2975a188`); `c6fb1499` merges main `d1ebfc72`. Each `fixed` row's after-excerpt is in the round-1 Linear dispositions comment.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | round 2 (fix commit 53eb40c5) |

Disposition pass 2 at frozen head `53eb40c5ea191641197071b641c749429f2a2d3b`. The fixes were inspected in fix commit `53eb40c5` (`git diff a39e303c 53eb40c5`); `a39e303c` merges main `9f5b30a5`. Round-2 rows are the rows whose reason begins `round 2`. Each `fixed` row's after-excerpt is in the round-2 Linear dispositions comment.
