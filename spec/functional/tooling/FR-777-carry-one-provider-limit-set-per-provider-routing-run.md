---
id: FR-777
title: "Carry one provider limit set per provider-routing run"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-462
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-300
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-561
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-964
    type: traces_to
---

# FR-777: Carry one provider limit set per provider-routing run

## Description

Every operation that routes items to providers (`analyze`, `prove`,
`monitor`, `execute` with requirement settlement (FR-775), `replay` and
`generate`) SHALL take the FR-462 provider limits as one caller-supplied set
for the run, with the same members and defaults for every operation, and SHALL
apply that set to every provider the run uses.

The caller sets the limits once per invocation (FR-561 keeps them out of the
provider-configuration document), so a set per provider would carry values no
invocation can state. One set shared by every operation also means the same
limits give the same requests whichever operation routed the item.

## Inputs

The caller's FR-462 limit set: the provider deadline and the transport
limits, each with its published default (FR-460).

## Outputs

Every FR-331 request of the run carries the set's `deadline_ms`; every process
provider of the run is held to its transport limits.

## Behavior

- An operation SHALL read no provider limit from a provider's registration,
  a configuration document, an environment variable or a file.
- A linked provider SHALL receive `deadline_ms` and ignore the transport
  limits.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-777-AC-1 | An `analyze` run and an `execute` run with requirement settlement, each given one limit set with `deadline_ms` 250 and two process providers, send every request `deadline_ms` 250 and hold both providers to the set's frame limit; given no set, both send FR-462's default deadline. | Test (TC-1882) |

## Dependencies

- FR-462: the limits. FR-300: the operations. FR-561: the configuration that carries no limit.
