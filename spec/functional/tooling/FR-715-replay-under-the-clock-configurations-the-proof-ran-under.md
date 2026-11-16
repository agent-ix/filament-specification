---
id: FR-715
title: "Replay under the clock configurations the proof ran under"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-323
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-252
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-336
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-862
    type: references
---

# FR-715: Replay under the clock configurations the proof ran under

## Description

When a user replays a proved package with the `replay` command (FR-301),
the replay SHALL recompile the package's sources under exactly the clock
configurations the package was compiled under (FR-252), read from the
package itself. A replay request SHALL NOT carry a clock configuration, and
the `replay` command SHALL refuse a request that carries one before any
replay.

A replay checks that a counterexample or input reproduces the proved
verdict. A clock configuration changes the meaning of a temporal clause (its
unit, its sequence authority, its sampling), so a replay under another
configuration would check a different claim and could reproduce or miss a
verdict the proof never stated. Reading the configuration from the package
makes the replay's configuration the proof's by construction.

## Inputs

A replay request file (FR-323 `NativeRuntimeEnvelope` with its `replay`
member, encoded as FR-336 "Replay request file" maps it) and the files its
byte provision names.

## Outputs

The replay result (FR-323 `results`), or a refusal envelope on stderr.

## Behavior

- The recompile SHALL take each clock configuration of the proving program
  from the package's temporal clock bindings, and each library's from the
  package lock's dependency clock selections (FR-252), never from the
  request or from a default.
- The `replay` command SHALL refuse a replay request with a `clocks`
  member at stage `request`, `invalid-request`, exit 20, naming `clocks`, with no result.
- The recompile SHALL refuse a package whose bindings lack a configuration
  it needs with its own refusal (`missing-clock-selection`),
  never completed from another binding of the same clock name.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-715-AC-1 | `replay` over a request whose package was compiled with `clocks` `{"plant": {"kind": "timed", "unit": "ms"}}` recompiles under that configuration and reproduces the proved verdict; the same request with a `clocks` member added refuses at stage `request`, `invalid-request`, exit 20, naming `clocks`, and writes no result. | Test (TC-1750) |
| FR-715-AC-2 | A replay request whose package lock omits the dependency clock selection a library's clause needs refuses `missing-clock-selection` naming the clock and the library, and is not completed from the program's binding of the same clock name. | Test (TC-1750) |

## Dependencies

- FR-301 (the `replay` command), FR-323 (the replay request), FR-252 (clock
  bindings in the package), FR-336 (the replay request file encoding).
