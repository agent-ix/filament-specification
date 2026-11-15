---
id: FR-550
title: "Carry native-runtime arguments and per-item results as typed canonical values"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-323
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-144
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-204
    type: references
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-650
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-651
    type: traces_to
---

# FR-550: Carry native-runtime arguments and per-item results as typed canonical values

## Description

A native runtime request (FR-323) supplies function arguments and receives
one disposition per selected item. Every supplied argument value and every
completed value in a disposition SHALL be the FR-181 typed canonical form of
its declared type, for every value kind FR-181 defines. No value kind is
excluded from a request or a result, so any function a producer can check, a
runtime can run, whatever its parameter and result types.

One value form for arguments and results lets a completed result be supplied
unchanged as an argument of a later request, and lets two conforming
implementations compare their per-item results componentwise (FR-323
"Identity and validation").

## Inputs

- An FR-323 envelope whose `state_environment` supplies argument values,
  object populations and frames, and whose `selection` names one or more
  items.

## Outputs

- One disposition per selected item in `results`, in `selection` order,
  each completed value in its FR-181 typed canonical form.

## Behavior

- The runtime SHALL read each argument value as the FR-181 typed canonical form of its
  parameter's declared type: an integer as its exact decimal string at any
  magnitude, a Boolean, a float by its bits, a rational normalized, a
  decimal with its retained coefficient and scale, a text, a string, a UUID,
  a Timestamp, Bytes, a JsonObject, an option, a sequence, set, bag or
  ordered set (admitted by FR-144's "Supplied collection values"), a record,
  a tuple, an enumeration or union member, a quantity, or a reference.
- An argument whose declared type is a population SHALL name a population
  of `state_environment` by its declaration node; a population has no FR-181
  form, so the argument names a binding the envelope supplies and carries no
  members.
- A reference argument SHALL resolve in the complete object world of
  `state_environment`; one that resolves to no object refuses
  `dangling_reference`/`absent-target-in-complete-population` naming the
  parameter.
- An argument that is not a value of its parameter's declared type refuses
  `invalid_runtime_input` with the FR-144 or FR-181 cause (`wrong-value-kind`,
  `missing-member`, `unknown-member`, `invalid-value`, `duplicate-member`)
  or `cardinality_out_of_bound`, naming the parameter and the location
  inside the value, category refusal.
- A declared parameter or result type with no FR-181 form (a collection
  whose element type contains a population, FR-144) refuses
  `unknown_required_feature`/`unsupported-feature` naming the parameter or
  the function, category unsupported. No other type refuses.
- The runtime SHALL write a completed value in the FR-181 typed canonical form of
  the function's declared result type; a reference with its own universe,
  most-specific type and object identity (FR-204); a population-typed result
  as the `set` of its members' references.
- The runtime SHALL settle each selected item independently against the same
  checked package: a refused, undefined, incomplete or violated item is that
  item's disposition and SHALL NOT change, skip or reorder another item's.
  An evaluator invariant failure remains FR-323-AC-1's failed disposition of
  the whole request.
- The envelope's overall category SHALL be the most severe item category
  under FR-301's exit-code ordering (`30`, `20`, `21`, `22`, `10`, `11`,
  `0`).

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-550-AC-1 | A request selecting an identity function of each FR-181 kind (integer above 2^64, Boolean, float64, rational, decimal, text, string, option, sequence, set, record, tuple, enum member, reference) with an argument in that kind's typed canonical form yields a completed disposition whose value is the same form, normalized where FR-181 normalizes (a rational `4/6` as `2/3`, a set's elements in canonical order). | Test (TC-1200) |
| FR-550-AC-2 | An argument of another kind, a record missing a field, a set with a repeated element and a sequence above its bound each refuse `invalid_runtime_input` (`wrong-value-kind`, `missing-member`, `duplicate-member`) or `cardinality_out_of_bound`, naming the parameter and the location in the value; a reference resolving to no object refuses `dangling_reference`/`absent-target-in-complete-population`; a population argument naming a supplied population binds it; a result type with no FR-181 form refuses `unknown_required_feature`/`unsupported-feature` naming the function. | Test (TC-1201) |
| FR-550-AC-3 | A request selecting three items, the second of which refuses, yields three dispositions in selection order whose first and third equal those of the same items selected alone, and an overall category of refusal; a request with a violated item and an incomplete item has overall category incomplete. | Test (TC-1202) |

## Dependencies

- FR-323: the envelope, its `state_environment`, `selection` and `results`.
- FR-181: the typed canonical forms; FR-144: supplied collection values;
  FR-204: object identity.
- FR-301: the exit-code ordering.
- FSL FR-650 and FR-651 realize these values and items in FSL's run
  documents, in FSL's native spelling of the same forms.
