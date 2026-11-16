---
id: TC-1662
title: "A replay recompiles libraries under the recorded clocks"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-323
    type: verifies
---

# TC-1662: A replay recompiles libraries under the recorded clocks

## Description

Check that a replay of a program whose library uses a clock of its own reads
that clock's configuration from the proved package's
`dependency_clock_selections`, and the refusals when the member is missing,
holds an unused entry or holds an entry of the wrong kind.

Scope: FR-323-AC-38.

## Test Procedure

1. Compile TC-1661 step 1's program and replay one of its functions from a
   request whose `package` is that package and whose `dependencies` entry
   names the library's sources.
2. Replay the same request with the package's
   `dependency_clock_selections` removed; with an added entry `spare`; with
   the `lib-ms` entry replaced by a `fixed_sample` configuration.
3. Compile a variant whose program also has a clause on `lib-ms`, remove the
   member from its package and replay it.

## Expected Results

- Step 1: the replay recompiles and gives the proved run's result.
- Step 2: `missing-clock-selection` naming `lib-ms` and the library at
  `/package/lock/dependency_clock_selections`; `unused-clock-selection` at
  the `spare` entry; `clock-kind-mismatch` at the `lib-ms` entry's
  `clock/kind`. None yields a result.
- Step 3: `missing-clock-selection` naming `lib-ms` and the library; the
  program's row does not serve the library.
