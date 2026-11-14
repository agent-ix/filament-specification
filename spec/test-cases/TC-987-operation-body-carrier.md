---
id: TC-987
title: "Operation-body carrier and strict reader boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-987: Operation-body carrier and strict reader boundary

## Description

Verify that the checked package carries each model operation's authored body as
an `operation_bodies` row keyed by selected document, declaring node and original
operation name, naming the checked body function node or stating the typed
absence `no_body`; that no row or body is omitted; and that the reader refuses a
malformed, mis-keyed, mis-attributed, mis-owned or incomplete carrier with exact
codes and loci.

## Test Procedure

Read `proposals/checked-package-v2/operation-body-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Validate every row of
each positive case against `$defs.OperationBodyRow`, then read each case's rows
against its `documents` (types, their operations and optional `extends`), `graph_nodes` (each with form,
`owner` and `clause`; whether an operation has a body is derived from them, a document carries no `has_body`) and optional `dispatch_candidates` (each with `selected_operation`, `selection_key` and `operation_body`, joined to a row through its `selection_key`), requiring one row per
operation, keyed and ordered by `(document, declaring, operation)`. OBC-01 is a
body, OBC-02 `no_body`, OBC-03 two members of one object, OBC-04 the same member
name `settle` under a process and two object types with a subtype that inherits
it and has no row, OBC-05 a dispatch candidate that agrees with the row, OBC-06 the present empty member, and OBC-07 three candidates of one dispatch table (two types redefining the operation, one inheriting it) sharing one `selected_operation` and each agreeing with the row its `selection_key` joins.

For each `schema_refusals` and `reader_refusals` case, apply its RFC 6902 `patch`
to `{"operation_bodies": <rows of base>}`. A schema case fails
`$defs.OperationBodyRow` as `malformed_wire` at exactly the recorded locus. A
reader case passes the schema and the reader, run against the base case, refuses
with the recorded code, cause and locus, and no other. A reader case may replace
the base `dispatch_candidates`. Each case holds one defect.

## Expected Results

All positive cases read back. The 13 schema refusals (OBS-01 to OBS-13) are
`malformed_wire`. The 21 reader refusals (OBR-01 to OBR-20 and OBR-22; OBR-21 is retired) are: OBR-01
`missing_declaration`/`missing-selection`; OBR-03 and OBR-10
`missing_declaration`/`missing-name`; OBR-02, OBR-05, OBR-08, OBR-09, OBR-12,
OBR-14, OBR-15, OBR-16, OBR-17, OBR-18, OBR-19, OBR-20 and OBR-22
`invalid_package`/`invalid-value` (OBR-15 and OBR-16 at `/operation_bodies`;
OBR-17 to OBR-19 at `body/node` for a body of another operation, a precondition
clause function, and a node of another package; OBR-20 and OBR-22 at the row's `body`, a `no_body` row whose joined candidate carries a body and a candidate carrying another candidate's body); OBR-13
`invalid_package`/`conflicting-definition`; OBR-04, OBR-06, OBR-07 and OBR-11
`ill_typed`/`operator-ineligible`. The IR reader exposes the admitted body by
key and owns no mirror type; QSL owns emission.

OBR-21 (a body node whose owner identity is another document) is retired: the
owner join of every owner-bearing node precedes the carrier checks, so such a
node refuses `missing_declaration`/`missing-selection` at its `node_id` before
any carrier check and the carrier cannot record it. Each reader refusal holds one
defect and reaches exactly its recorded code and locus under the order FR-322
states: owner join, then the carrier, then `checked_term_data` validation.
