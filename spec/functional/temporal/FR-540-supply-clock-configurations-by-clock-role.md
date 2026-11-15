---
id: FR-540
title: "Supply trace-subject clock configurations by clock role"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-252
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
  - target: ix://agent-ix/filament-specification/FR-541
    type: references
---
## Description

When a caller compiles or runs a unit whose temporal declarations read a clock
the unit does not define, the frontend SHALL take one clock selection per clock
role, a map from the clock name the declarations write to one clock
configuration, and SHALL bind every declaration that writes that name to that
configuration. This is the clock binding FR-090 and FR-252 say the observation
interface supplies for a trace subject.

## Inputs

A checked unit; a clock selection: a set of `(role, configuration)` pairs, each
role a clock name and each configuration one of FR-322's `TemporalBinding`
`clock` alternatives: event-position (sequence authority), fixed-sample (exact
epoch, positive exact period, unit), timestamped-event (timestamp unit) or timed
(time unit).

## Outputs

One FR-322 `temporal_bindings` row per clock-bearing declaration occurrence,
carrying its clock name and the configuration selected for that name, or a
located refusal.

## Behavior

The selection SHALL be keyed by clock role, not by declaration. A role is the
name the source writes and stands for one clock; keying by role makes two
configurations for one clock unrepresentable and binds every declaration on a
clock with one entry.

A declaration over a trace subject under the event-position, fixed-sample or
timestamped-event profile, or under the timed profile, SHALL take the
configuration of its role, whose alternative SHALL be the one its profile
determines (FR-090). A declaration over a model subject SHALL take its binding
from the model (FR-252) and no selection entry; under the event-position profile
that binding's sequence authority is the model's step sequence. An
infinite-trace declaration over a trace subject takes no configuration (FR-090
"Clock names").

The frontend SHALL refuse, before any package is produced and naming the role:

- a declaration that needs a configuration whose role the selection lacks
  (`missing-clock-selection`, located at the declaration's clock name);
- a selection entry whose role no declaration that needs a configuration writes
  (`unused-clock-selection`);
- an entry whose alternative is not the one a declaration's profile determines
  (`clock-kind-mismatch`, located at that declaration's clock name).

The selection enters result identity only through the bindings it produces:
selections that differ in any configuration component produce different
binding identities (FR-090-AC-3), and the order of entries changes nothing.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-540-AC-1 | A unit with event-position, fixed-sample and timestamped-event clauses over a trace subject on roles `events`, `sample-clock` and `plant-ms`, and a selection with one entry per role, produces one binding row per clause holding its role and configuration; two clauses on one role get equal configurations. | Test (TC-1160) |
| FR-540-AC-2 | Omitting `plant-ms`, adding an entry `spare`, and selecting a timestamp unit for `events` each refuse before any package is produced with `missing-clock-selection`, `unused-clock-selection` and `clock-kind-mismatch` naming the role; an event-position clause over a model subject needs no entry. | Test (TC-1160) |
| FR-540-AC-3 | Changing the `events` sequence authority changes the binding identity; reordering the entries does not. | Test (TC-1160) |

## Dependencies

- [FR-090](FR-090-select-temporal-profile-and-clock.md) (profiles and clock
  names), [FR-252](../../objects/temporal/FR-252-temporal-clock-binding.md)
  (clock bindings), [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  (binding rows, including the timed alternative).
