---
id: TC-432
title: "Check and replay an event-avoidance lower certificate"
type: TC
relationships:
  - target: "ix://agent-ix/filament-specification/FR-412"
    type: verifies
  - target: "ix://agent-ix/filament-specification/FR-413"
    type: verifies
---
# TC-432: Check and replay an event-avoidance lower certificate

## Description

Verify FR-412-AC-24 and FR-413-AC-16: replay derives event-free bottom
components and checks a lower certificate for the probability of never
reaching an event.

## Test Procedure

Use a finite induced chain where initial state `s` transitions to event state
`e` with probability `1/2` and event-free state `a` with probability `1/2`,
and `a` self-loops. Check a `lower` certificate with values `x(s)=1/2`,
`x(a)=1`, `x(e)=0`, and ranks 1 at `s`, 0 at `a`, and 0 at `e` for the claim
`probability >= 3/4 [eventually E]`. Then add an event state to the claimed
event-free component and repeat with lower bound `1/4`.

## Expected Results

The checker derives `{a}` as the event-free bottom target without trusting a
producer component list, accepts the lower certificate, and replay confirms
`1/2 > 1 − 3/4`; the item settles `refuted`. The event-containing component
does not qualify. A lower bound equal to `1/4` does not refute because the
comparison is strict.
