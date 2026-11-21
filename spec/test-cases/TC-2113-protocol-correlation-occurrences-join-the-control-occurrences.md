---
id: TC-2113
title: "Protocol correlation occurrences join the control occurrences"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-466
    type: verifies
---

# TC-2113: Protocol correlation occurrences join the control occurrences

## Description

Verify that a reader admits the optional `protocol_correlation_occurrences`
member of `protocol-correlation-vectors.json` over complete published packages
and refuses each mutation with the stated code, cause and locus, in the single
row pass and the coverage pass of FR-831. Scope: FR-831-AC-1 to FR-831-AC-9,
FR-322-AC-514 and FR-466-AC-20. Every case names a complete package of `packages`
(the base `fixtures/protocol-node-base.json` plus RFC 6902 `ops`, or a complete
fixture file) and an `edit` that adds the member; no projection stands in for a
package. No reader implements FR-831 yet (the existing strict reader refuses the
member as `unknown_member`), so the carrier decisions of `positive`,
`reader_refusals` and `budget` are the expected results of a reader that has the
carrier and are not executed; the complete packages, the schema decisions and the
consistency of the vectors are executed.

## Test Procedure

1. Build each package of `packages`: apply its `ops` (published slot form) to its
   `fixture`, or take the fixture file; read it with the strict read without the
   member and expect admission. Validate each against the root V2 `schema.json`.
2. For each `schema_valid` case, apply its `edit` and validate; for each
   `schema_invalid` case, apply it, validate, and compare `malformed_wire` and the
   first schema error path with the case's `locus`. Every `positive` and
   `reader_refusals` edit is schema-valid.
3. For each `positive` and `reader_refusals` case, apply the `edit` to its
   package, read the package with the strict read, then run the carrier check; a
   case with `dependencies` is read once per named package against its own member
   and `source_map`. Compare the decision, the typed read (`complete` with its
   groups, each holding the whole `source_map` entry, or
   `unavailable_correlation_provenance`) or the refusal's code, cause and locus.
4. For each `budget.cases` case, measure the baseline consumption of the work and
   read-storage counters immediately before the step on the same input, then run
   the check with the limit at the baseline plus the case's offset and compare the
   admission or the `incomplete` result naming the counter, the first unpaid
   amount and the locus, with no partial typed view. `CO-B13` runs with earlier
   consumption on the same budget and expects no reset.
5. For each `invariance` case, apply `editA` and `editB` to the package and assert
   that only the member differs.

## Expected Results

- Step 1: every package is admitted by the strict read without the member and is
  schema-valid.
- Step 2: every `schema_valid` edit validates; every `schema_invalid` edit fails
  at its stated path.
- Step 3: every admitted case returns its stated typed read; every refusal returns
  the first defect in FR-831 order and no partial view; an absent member is
  `unavailable_correlation_provenance` and the package is admitted.
- Step 4: the exact limit admits and a limit one short returns `incomplete` at the
  stated first unpaid charge.
- Step 5: the member alone differs.

## Boundaries

- The packages are real: relationship `Order-order-Order` of `acme/orders`, ends
  `view` and `notified` typed `Reference<Order>`, canonical `related` records on
  `send`, `receive`, `attempt`, `effect`, a new `event` control and a new
  `recommend` control. A control has one declaration occurrence, so no
  two-occurrence package exists; `absent_by_standard` records the evidence and
  the join is exercised by a row naming occurrence ordinal 1 (`CO-R08`).
- The vectors check a reader's consistency checks. They do not prove that a
  producer recorded every authored clause, an omitted trailing duplicate, or the
  exact alias spelling, and they do not execute FSL FR-731's first-failing-clause
  replay, whose source-order oracle is unchanged.
