---
id: FR-765
title: "Default the clock binding of a model claim by its selected profile"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-252
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-250
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
---

# FR-765: Default the clock binding of a model claim by its selected profile

## Description

When a temporal claim over a model subject writes no `clock` member, the
frontend SHALL bind the default clock binding of the profile the claim's
`using` alias selects: `model-time` under `filament.temporal.timed/v1`, and
`model-steps` under every other profile. The model's `time` member decides
only whether the chosen binding can be built, never which binding is
chosen.

### Decision: the profile chooses the default

A claim always names its profile, and the profile already fixes what one
interval tick counts: the timed profile counts time units of the model's time
source, and every other profile counts positions (FR-250). A default chosen
from the model alone would, over a timed model under a step-counting profile,
bind `model-time`, which selects the timed profile (FR-252) and so
contradicts the alias the claim wrote; and over an untimed model under the
timed profile it would bind `model-steps`, which the timed profile does not
read. Choosing the default from the profile gives every claim with no `clock`
member exactly one binding that agrees with its profile. The only claim left
without a binding is a timed-profile claim over a model that has no time to
count, and it refuses as an explicit `model-time` claim over that model does.
No claim that checked under the model-chosen default changes binding: under
the timed profile over a timed model and under a step-counting profile over
an untimed model the two rules choose the same binding.

## Inputs

- A checked temporal claim over a model subject with no `clock` member.
- The profile its `using` alias selects (FR-090).
- The model's time source, or none (FR-415).

## Outputs

The claim's clock binding (FR-252), or a typed refusal.

## Behavior

- Under `filament.temporal.timed/v1`, the default SHALL be `model-time`. Over a
  model with a time source the binding SHALL be built from that source as
  FR-252 states. Over a model with no time source the frontend SHALL refuse
  `invalid_timed_model`/`model-time-on-untimed-model` at the claim's `using`
  alias, naming the model and the timed profile, since the claim has no clock
  member to locate the refusal at.
- Under every other profile the default SHALL be `model-steps`, over a timed
  and over an untimed model alike. Its interval bounds are step counts, so a
  bound with a time unit refuses `ill_typed`/`type-mismatch` at its span as
  FR-252 states for an explicit `model-steps` claim.
- The default binding SHALL give the same binding and obligation identity as
  the same claim with that binding written.
- The default rule SHALL NOT apply to a claim over a trace subject, whose clock
  name is a role the request configures.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-765-AC-1 | Over a timed model with `time dense unit ms`, a claim with no `clock` member under `filament.temporal.timed/v1` binds `model-time` and equals the same claim with `clock "model-time"`; the same claim under `filament.temporal.event-position.false-extension/v1` with `eventually[0, 3]` binds `model-steps` and equals the same claim with `clock "model-steps"`, and with `eventually[0 ms, 3 ms]` refuses `ill_typed`/`type-mismatch` at the interval. | Test (TC-1840) |
| FR-765-AC-2 | Over an untimed model, a claim with no `clock` member under `filament.temporal.timed/v1` refuses `invalid_timed_model`/`model-time-on-untimed-model` at its `using` alias, naming the model; under `filament.temporal.infinite-trace/v1` it binds `model-steps`. | Test (TC-1840) |

## Dependencies

- FR-252 owns the `model-steps` and `model-time` bindings and their keys; this
  requirement states which one a claim with no `clock` member binds.
- FR-250 and FR-090 own the profiles; FR-415 owns a model's time source.
