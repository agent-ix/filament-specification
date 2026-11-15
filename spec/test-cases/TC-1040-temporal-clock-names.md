---
id: TC-1040
title: "Temporal clock names over model and trace subjects"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-090
    type: verifies
---
# TC-1040: Temporal clock names over model and trace subjects

## Description

Verifies that a clause's clock name selects a binding over a model subject
from exactly `model-steps` and `model-time`, names a clock role over a trace
subject, never changes what one interval tick counts, and is refused when it
spells a time unit over a trace subject (FR-090 "Clock names").

## Test Procedure

1. Over a model subject under `filament.temporal.infinite-trace/v1`, check
   `eventually[0,3] holds(p)` with `clock "model-steps"`, with no clock, and
   with `clock "ms"`, `clock "wall"` and `clock "steps"`.
2. Over a trace subject, check `eventually[0,3] holds(p)` with
   `clock "events"` under the infinite-trace and event-position profiles and
   with `clock "sample-clock"` under the fixed-sample profile, and evaluate the
   admitted clauses over a three-step trace.
3. Over a trace subject, check the same clause with `clock "ms"`,
   `clock "s"`, a role spelled as a time unit the unit declares,
   `clock "model-steps"` and `clock "model-time"`, under each trace profile.

## Expected Results

1. `model-steps` and the unwritten clock both bind one tick per step; each
   other name refuses `unsupported_construct` at the clock name, naming
   `model-steps` and `model-time`, before evaluation (FR-090-AC-11).
2. Each clause is admitted and `[0,3]` counts three positions or three samples
   (FR-090-AC-12).
3. Each refuses `ill_typed` at the clock name, naming the profile and what one
   tick counts under it (FR-090-AC-12).
