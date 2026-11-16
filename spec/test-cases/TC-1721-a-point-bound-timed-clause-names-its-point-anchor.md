---
id: TC-1721
title: "A point-bound timed clause names its point anchor"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-342
    type: verifies
---

# TC-1721: A point-bound timed clause names its point anchor

## Description

A point-bound timed clause names its point anchor. Scope: FR-342-AC-13.

## Test Procedure

Read a package whose `reply` guard and reset carry `point_anchor` bindings to the `pre` and `post` anchors of `reply`; then read variants naming the no-point anchor, the wrong point, another operation's anchor and a frame, and a model holding a point-bound and a point-free guard of `reply`.

## Expected Results

- The point-bound guard and reset, and the published layouts without `point_anchor`, are admitted with no added `anchor` occurrence.
- Each variant refuses `invalid_package`/`timed-node-reference` at the binding's target, and the duplicate guard at the second guard.
