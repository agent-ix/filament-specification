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

Verify FR-412-AC-25, FR-412-AC-28 and FR-413-AC-18 for both comparison directions and ensure
that a cyclic subject cannot be refuted with a path set for an unwindowed
`eventually`, `always`, or `until` claim.

## Test Procedure

Use finite `INFINITE-TRACE` subjects whose initial state `s` has a self-loop
of probability `1/2` and transitions with probability `1/4` each to an
accepting and a rejecting state. For `always P`, make the accepting state
satisfy `P` and the rejecting state violate `P`; both are terminal with
implicit stutter. For `eventually E`, make only the accepting state satisfy
`E`; both are terminal with implicit stutter. These models have a cyclic
original reachable graph and give the accepting and rejecting components
probability `1/2` each. For `A until B`, make the accepting state satisfy `B`
and the rejecting state satisfy neither `A` nor `B`, and give both a
deterministic transition back to `s`. The original graph for this case is one
cyclic component; the claim monitor decides at either state and absorbs that
product state with only a monitor-preserving self-loop. Its accepting and
rejecting bottom components each have probability `1/2` from `s`.

1. For `probability >= 3/4 [always P]`, make the accepting state satisfy
   `P` and the rejecting state violate `P`. Supply a `lower` certificate
   for the rejecting bottom-component union with initial bound `1/2`.
2. For `probability <= 1/4 [always P]`, use the same subject and supply a
   `lower` certificate for the accepting union with initial bound `1/2`.
3. For `A until B`, use an accepting state satisfying `B` and a rejecting
   terminal satisfying neither `A` nor `B`; repeat steps 1 and 2 for `>= 3/4`
   and `<= 1/4`.
4. For `probability <= 1/4 [eventually E]`, make only the accepting state
   satisfy `E` and supply a `lower` certificate for the accepting union with
   initial bound `1/2`.
5. For each claim, also supply a finite path set that crosses its bound. Try
   lower bounds equal to `1/4`, an incorrectly classified bottom component,
   and an invalid progress rank.
6. Make the initial state of an `always P` subject violate `P` (and, for `eventually E`, give `E` an empty cylinder) while a repeatable operation keeps a reachable cycle in the raw graph, and replay the refutation (FR-412-AC-28).

## Expected Results

For every claim, the provider does not begin path-set search and emits no
path-set evidence because the original reachable graph has a nonterminal
cycle. Replay absorbs decided monitor states before SCC decomposition and
independently derives the relevant bottom-component union,
accepts the valid certificate, and applies the strict comparison: `1/2 > 1/4`
settles `refuted`. The equal bound, wrong component, or broken rank does not
settle `refuted`; a supplied path set is not accepted as evidence for these
cyclic claims.

For step 6, the initial verdict is decided before any graph ordering and the replay settles `refuted` without refusing for the cycle; a part that needs a ranking over the cyclic graph is refused naming that part, and the initial verdict is unchanged.
