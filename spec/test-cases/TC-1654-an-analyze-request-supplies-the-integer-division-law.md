---
id: TC-1654
title: "An analyze request supplies the integer-division law"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
---
# TC-1654: An analyze request supplies the integer-division law

## Description

A `native-analyze/1` request's `integer_division` member supplies the law its
program's `div` needs, and its absence or an unknown name refuses the whole
request.

Scope: FR-520-AC-5.

## Test Procedure

Over a program declaring `a div 2` with no law header and one claim, analyze the request with `"integer_division": "floor"`, without the member, and with `"integer_division": "ceiling"`.

## Expected Results

- `floor`: the program compiles and the claim's item settles.
- No member: the request refuses `missing_declaration`/`missing-selection` at the `div`; no item settles.
- `ceiling`: `unknown_profile`/`unsupported-selection`; no source is read.
