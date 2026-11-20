---
id: TC-1973
title: "Canonical export text and injective names"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-814
    type: verifies
---

# TC-1973: Canonical export text and injective names

## Description

Verify keyword-safe and injective names, canonical layout, dependency order and the name map. Scope: FR-814-AC-1 to FR-814-AC-5 and FR-814-AC-7 to FR-814-AC-11.

## Test Procedure

1. Export operations named `MODULE`, `Next` and `theorem`.
2. Export names `a_b`, `a__b`, `Shop::Order` and one with `é`.
3. Export one request twice.
4. Export a function used by an invariant; reorder two independent declarations.
5. Read a TLC trace back through the name map.
6. Export a model whose field, operation parameter, function and function parameter are all named `total`.
7. Export an operation of `Item` whose declared parameter is spelled `self`.
8. Export an invariant whose binders are named `Next`, `instance`, `DOMAIN`, `num` and `tag` over fields `num` and `tag`, a `Decimal` field and an `Option` field.
9. Export a model with operations `inc(a: Integer)` and `dec(a: Integer)`, a function `f(a: Integer)` and two invariants that each quantify `k`; read every `p_a`, `a_a` and `x_k` name-map entry, and read a TLC step of `op_dec` naming `p_a` back through the map.

## Expected Results

- Step 1: prefixed names; both observers parse.
- Step 2: distinct mangled names, `_uE9_` for `é`.
- Step 3: byte-identical ASCII text with the stated layout.
- Step 4: the function first; only node order changes.
- Step 5: the FSL nodes.
- Step 6: `fld_total`, `p_total`, `f_total` and `a_total`, pairwise distinct, each mapped to its own node.
- Step 7: `r_self` and `p_self`, mapped to the operation and the parameter; SANY and Lean parse both files.
- Step 8: `x_Next`, `x_instance`, `x_DOMAIN`, `x_num`, `x_tag`, `fld_num` and `fld_tag`; SANY and Lean parse both files.
- Step 9: two `p_a` entries, each with the `binding` of its own operation and `node` its own parameter, ordered by `binding`; one `a_a` entry bound by `f`; two `x_k` entries, each bound by its invariant's quantifier; no `binding` on single-node names; the `op_dec` step reads back to `dec`'s parameter.
