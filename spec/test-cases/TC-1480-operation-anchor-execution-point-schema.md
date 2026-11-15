---
id: TC-1480
title: "Operation anchor execution point against the V2 schema"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-342
    type: verifies
---

# TC-1480: Operation anchor execution point against the V2 schema

## Description

Validates `state`/`operation_anchor` bodies with and without the optional
`execution_point` binding against `proposals/checked-package-v2/schema.json`
and reads the schema-valid ones through the strict reader, so that the schema
and FR-342's body contract agree.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| EP-01 | `fixtures/positive-timed-clause-nodes.json` unchanged, and with CL-03's patch (`execution_point` `pre{operation: "size"}` added to node 70) | Both validate against the schema | FR-342-AC-12 |
| EP-02 | EP-01's anchor with `post{operation: "size"}` | Validates against the schema | FR-342-AC-12 |
| EP-03 | EP-01's anchor with `{"kind": "initialization", "name": "start"}` and with `{"kind": "handler", "name": "h"}` | Each validates against the schema; the strict reader refuses `invalid_model_binding`/`malformed-declaration` at the anchor | FR-342-AC-12 |
| EP-04 | Independently: a fourth binding named `point`; a fifth binding after `execution_point`; `execution_point` placed before `frame`; a point `{"kind": "mid", "operation": "size"}`; a point `{"kind": "pre", "operation": "size", "extra": 1}` | Each fails the schema | FR-342-AC-12 |

## Expected Results

The schema admits exactly the three-binding body and the four-binding body
ending in a closed `ExecutionPoint`; point applicability is a reader refusal.
