---
id: TC-1481
title: "Collection-typed fields name a package type, never rendered type text"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-154
    type: verifies
---

# TC-1481: Collection-typed fields name a package type, never rendered type text

## Description

Reads the published `acme/orders` domain package document and patched copies
of it through domain-package intake, checking that a collection-typed field
names its collection through a package type definition and that rendered type
text in a `typeRef` refuses.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| NT-01 | `proposals/checked-package-v2/domain-package-acme-orders.json` | Admitted; `Order.clock` has `typeRef` `ix://acme/orders/ClockMap`, a map type definition with `keys` `ix://filament/native/Text` and `values` `ix://filament/native/Clock`, read as `Map<Text, Clock>` | FR-154-AC-50 |
| NT-02 | NT-01 with `Order.clock`'s `typeRef` set to `Map<Text, ix://filament/native/Clock>` | `invalid_model_binding`/`malformed-declaration` at the field node and `Field.typeRef` | FR-154-AC-50 |
| NT-03 | NT-01 with a field's `typeRef` set to `Sequence<ix://filament/native/Integer>` | `invalid_model_binding`/`malformed-declaration` at the field node and `Field.typeRef` | FR-154-AC-50 |
| NT-04 | Every published domain package document under `proposals/` | No `typeRef`, `items`, `keys`, `values`, `inner`, `target` or `payloadType` holds rendered type text | FR-154-AC-50 |

## Expected Results

A `typeRef` is one reference of the three forms; collection types are named
package type definitions.
