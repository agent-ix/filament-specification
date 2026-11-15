---
id: TC-1041
title: "Past operators before the origin and derived previous, next and weak until"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-092
    type: verifies
---
# TC-1041: Past operators before the origin and derived previous, next and weak until

## Description

Verifies the pre-origin reading of closed past intervals under every profile
with an authoritative origin, the origin-bounded reading of unbounded and
lower-bounded past operators, and that previous, next and weak until are
written only as their derived forms (FR-092 "Derived forms", "Positions before
the origin").

## Test Procedure

1. On a trace where atomic `p` holds at every position, under the
   event-position false-extension, infinite-trace and timestamped
   finite-window (`execution-origin`) profiles, evaluate `once[1,1] p` and
   `once[1,1] not p` at position 0, `historically[0,2] p` and
   `historically[0,2] true` at position 1, and `true since[1,1] not p` at
   position 0.
2. Under the infinite-trace profile evaluate the unbounded `historically p` and
   `once[2,*] not p` at position 1.
3. Repeat step 1 with a `history-cutoff` boundary in place of the origin.
4. Parse `previous holds(p)`, `next holds(p)` and
   `holds(p) weak until holds(q)`.
5. Compare `once[1,1] p`, `eventually[1,1] p` and
   `(p until[a,b] q) or always[a,b] p` against an independent oracle of
   strong previous, next and bounded weak until over all traces of length up
   to 5 and all intervals with `b <= 3`.

## Expected Results

1. False, true, false, true and true under each profile (FR-092-AC-9).
2. True and false (FR-092-AC-9).
3. Each closed-interval case reports missing history (FR-092-AC-9).
4. Each is a syntax error at `previous`, `next` or `weak`, with a message
   naming `once[1,1]`, `eventually[1,1]` and `(p until q) or always p`
   (FR-092-AC-10).
5. Every derived form agrees with its oracle at every position
   (FR-092-AC-10).
