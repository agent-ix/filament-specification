---
id: TC-2047
title: "An export outcome carries its refusal, and an output failure exits 30"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-271
    type: verifies
---

# TC-2047: An export outcome carries its refusal, and an output failure exits 30

## Description

Verify that a `refused` export outcome carries the closed `refusal` member in
its observer and below-floor shapes, that no other status carries it, and
that an export whose target file cannot be written is an output failure,
exit 30, with the catalogued code and cause `output-failure`. Scope:
FR-817-AC-14, FR-817-AC-15, FR-817-AC-16 to FR-817-AC-18 and FR-271-AC-8.

## Test Procedure

1. Export `SlowEnough` over `Braking` under `any` with `observer` `tlc` in an
   implementation with no TLC backend; then under the default floor; then an
   assembled and a limit-stopped export.
2. Export under `any` with `output` `{"file": "missing-directory/m.tla"}`;
   then with `m.tla` already present.
3. Export a clause that a protocol requires, in an implementation that does
   not lower protocol subjects; under `any`; under `{"export.nodes": 1}`.
4. Export a program whose checked package admission refuses
   `invalid_package`, and one with a missing import.
5. Export a clause required by `Hand` with a `protocols` binding, with the
   binding malformed, binding a whole-model role, without a binding for a
   model-type role, and without a universe for a replicated role.

## Expected Results

- Step 1: the observer `refusal` shape with exactly its six members, exit
  21; the below-floor shape with `failing` the failing record, exit 21; no
  `refusal` member on the other two.
- Step 2: exit 30, internal-failure, one `output-failure` diagnostic,
  `last_stage` the stage reached, nothing created; then `invalid-request`,
  exit 20.
- Step 3: the construct refusal with exactly its seven members, exit 21, no
  package or file, under every floor; `incomplete` under `export.nodes` `N - 1`.
- Step 4: each program or package refusal keeps its own code, cause and
  exit; none exits 30.
- Step 5: the fixed `over`, then the FR-520 refusal, `wrong-role-mapping` naming
  each role, and `unbounded-extent` naming the population.
