---
id: TC-1322
title: "Read a tuple position with a literal index"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-143-AC-22
    type: verifies
---

# TC-1322: Read a tuple position with a literal index

## Description

Verify the FR-143 tuple position read `p[n]` and the refusal of `p.0`.

## Test Procedure

Select `filament.value.complete/v1`. Declare `tuple Pair(Boolean, Integer);` and `tuple Nest(Pair, Integer);`; check functions reading `p[0]`, `p[1]` and `q[0][1]`, read the emitted package, then check `p[2]`, `p[i]` and `p.0`.

## Expected Results

`p[0]` is `Boolean`, `p[1]` and `q[0][1]` are `Integer`, each one `filament.op.tuple.project` with the position member; `p[2]` and `p[i]` are `ill_typed`/`operator-ineligible` at the index; `p.0` is `invalid_syntax`/`unexpected-token` at `0`.
