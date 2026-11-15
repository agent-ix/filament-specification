---
id: TC-1442
title: "Source object member types follow the record field rule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-610
    type: verifies
---

# TC-1442: Source object member types follow the record field rule

## Description

Verify that every type a native record field admits is admitted for an
object field, parameter and result with the same package form, and that a
type a record field refuses is refused for an object member with the same
code. Scope: FR-610-AC-5 and FR-610-AC-6.

## Test Procedure

For each member type of FR-610-AC-5, check a unit holding `record R { f: T; }`
and `object O { f: T; operation g(p: T): T; }` (with the enum, record,
union, tuple, alias, dimension and unit declarations the type needs), and
emit both. For FR-610-AC-6, check the same pair with `T` = `Population<O>[3]`
and with `T` the name of a source function.

## Expected Results

For each FR-610-AC-5 type, both units check, and `O`'s binding `f` names the
same type node as `R`'s binding `f`. For each FR-610-AC-6 type, the object unit
is refused at the type with the code and cause the record unit gets, and the
function name refuses `missing_declaration`/`missing-name` at the name.
