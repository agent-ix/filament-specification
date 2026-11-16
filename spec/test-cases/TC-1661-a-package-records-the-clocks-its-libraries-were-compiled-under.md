---
id: TC-1661
title: "A package records the clocks its libraries were compiled under"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1661: A package records the clocks its libraries were compiled under

## Description

Check the `lock.dependency_clock_selections` member: what a compile writes
into it, its absence when no library takes a request clock, and the reader's
shape and agreement refusals.

Scope: FR-322-AC-452.

## Test Procedure

1. Compile a program that imports a library whose timestamped-event clause is
   on clock `lib-ms`, with the request configuration
   `{"kind": "timestamped_event", "timestamp_unit": "ms"}` for `lib-ms`; read
   the lock. Compile it again with `timestamp_unit` `s`.
2. Compile a program whose library declares no clock-bearing declaration.
3. Read the package of step 1 with the member set to `[]`, with an entry
   carrying a third key, with two entries out of `clock_name` order, with a
   repeated `clock_name`, with an entry named `model-steps`, and, in a
   program whose own clause is also on `lib-ms`, with the entry's
   configuration changed.

## Expected Results

- Step 1: the member is the one entry for `lib-ms` with the request's
  configuration; the two packages are structurally unequal.
- Step 2: the member is absent.
- Step 3: schema failure, schema failure, `invalid_package`/`invalid-value`
  at the later entry, `invalid_package`/`conflicting-definition` with both
  entries, `invalid_package`/`invalid-value` at the `clock_name`, and
  `invalid_package`/`conflicting-definition` naming the entry and the row.
