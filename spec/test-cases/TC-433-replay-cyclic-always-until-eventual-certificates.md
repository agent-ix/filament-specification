---
id: TC-433
title: "Replay cyclic always, until, and eventual refutation certificates"
type: TC
relationships:
  - target: "ix://agent-ix/filament-specification/FR-412"
    type: verifies
  - target: "ix://agent-ix/filament-specification/FR-413"
    type: verifies
---
# TC-433: Replay cyclic always, until, and eventual refutation certificates

## Description

Verify FR-412-AC-25 and FR-413-AC-18 for both comparison directions and ensure
that a cyclic subject cannot be refuted with a path set for an unwindowed
`eventually`, `always`, or `until` claim.

## Test Procedure

Use a finite `INFINITE-TRACE` subject whose initial state `s` has a self-loop
of probability `1/2` and transitions with probability `1/4` each to accepting
and rejecting terminal states. Terminal states have only implicit terminal
stutter. This makes the original reachable graph cyclic and gives each
terminal bottom component probability `1/2`.

1. For `probability >= 3/4 [always P]`, make the accepting terminal satisfy
   `P` and the rejecting terminal violate `P`. Supply a `lower` certificate
   for the rejecting bottom-component union with initial bound `1/2`.
2. For `probability <= 1/4 [always P]`, use the same subject and supply a
   `lower` certificate for the accepting union with initial bound `1/2`.
3. For `A until B`, use an accepting terminal satisfying `B` and a rejecting
   terminal satisfying neither `A` nor `B`; repeat steps 1 and 2 for `>= 3/4`
   and `<= 1/4`.
4. For `probability <= 1/4 [eventually E]`, make only the accepting terminal
   satisfy `E` and supply a `lower` certificate for the accepting union with
   initial bound `1/2`.
5. For each claim, also supply a finite path set that crosses its bound. Try
   lower bounds equal to `1/4`, an incorrectly classified bottom component,
   and an invalid progress rank.

## Expected Results

For every claim, the provider does not begin path-set search and emits no
path-set evidence because the original reachable graph has a nonterminal
cycle. Replay independently derives the relevant bottom-component union,
accepts the valid certificate, and applies the strict comparison: `1/2 > 1/4`
settles `refuted`. The equal bound, wrong component, or broken rank does not
settle `refuted`; a supplied path set is not accepted as evidence for these
cyclic claims.
