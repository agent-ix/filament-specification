---
id: TC-059
title: "Assess finite global conformance"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-059
    type: verifies
---
## Description

Verify global conformance across permitted concurrency and adverse finite histories.

## Test Procedure

Replay every causally valid S1/S2 interleaving with payment and refund. Inject
unexpected, missing, misordered, duplicate-effect and cross-workflow events.
Compare open/incomplete histories and settled batch versus incremental inputs.
For a global-conformance claim, retain the exact required execution, branch and
workflow closure-premise set and exercise `closed`, `open`, `incomplete` and
`contradicted` complete-global-conformance closure states. Independently omit,
duplicate, cross-wire, change scope on and change identity on every required
premise; substitute decision-scope closure, surrounding-execution closure and
completed assessment execution for the global closure record in turn. Keep one
independently settled per-obligation truth fixed through every mutation.

Assess a recording of a protocol with a fan-out node, with no activated
replicated-role population stated and no obligation violated, under a `closed`
and an open closure record; then state the population, once with a complete
`closed` record and once with an obligation pending (FR-059-AC-6).

## Expected Results

Valid interleavings conform. Every adverse event names its obligation;
open/incomplete inputs never report complete success; settled batch and
incremental semantic results agree exactly. Complete global success occurs only
with the exact `closed` complete-global-conformance closure record and full
required premise set. Every omitted, duplicate, cross-wired, wrong-scope or
wrong-identity premise prevents complete success or refuses the record, while
the independently settled per-obligation truth remains unchanged. The fan-out recording
without a population settles `inconclusive`, cause `incomplete-closure-scope`,
under either closure state; with the population it settles `proved`, and
`inconclusive`, cause `open-history`, with an obligation pending.
