---
id: TC-1651
title: "The CLI supplies the integer-division law"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-301
    type: verifies
---
# TC-1651: The CLI supplies the integer-division law

## Description

`check --integer-division <law>` and the `integer_division` request member
supply the law for `div` and `rem`; there is no default.

Scope: FR-301-AC-6.

## Test Procedure

The CLI binary owns option parsing, so this test lives in the driver repository.

Over a complete-value unit declaring `function q(a: Int[-7, 7], b: Int[1, 3]): Int[-7, 7] { a div b }` and no law header: run `check --integer-division floor`, `check` with no option and `check --integer-division ceiling`; run a `run` request with `"integer_division": "floor"` calling `q(-7, 3)`; run `compile`, `run`, `analyze`, `prove` and `monitor` with `--integer-division floor` over a request.

## Expected Results

- `check --integer-division floor`: exit `0`.
- No option: exit `20`, `missing_declaration`/`missing-selection` at the `div`.
- `ceiling`: exit `20`, `unknown_profile`/`unsupported-selection` naming the option and the value; the source is not read.
- The `run` request: exit `0`, value `-3`.
- `compile`, `run`, `analyze`, `prove` and `monitor` with the option: exit `20` as an unknown option.
