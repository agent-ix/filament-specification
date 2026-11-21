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
row pass and the coverage pass of FR-831, and that a producer and a consumer meet
their obligations. Scope: FR-831-AC-1 to FR-831-AC-12, FR-322-AC-516 and
FR-466-AC-20. Every case names a complete package of `packages` (the base
`fixtures/protocol-node-base.json` plus RFC 6902 `ops`, or a complete fixture
file) and an `edit` that adds the member; no projection stands in for a package.

## Test Procedure

Steps 1, 2 and 7 are executed now by `tests/protocol_correlation_vectors.rs`, as
data and schema qualification: they run no reader, producer or consumer. Steps 3,
4, 5 and 6 are planned and run once the IR reader, the FSL producer and a
consumer exist; none is executed today, because the existing strict reader
refuses the member as `unknown_member`.

1. **Packages and schema (executed).** Build each package of `packages`: apply
   its `ops` (published slot form) to its `fixture`, or take the fixture file.
   Validate each, and every case edit of `positive`, `reader_refusals`, `budget`
   and `consumer`, against the root V2 `schema.json`; read each package without
   the member through the existing strict reader and expect admission (an
   out-of-repo receipt, recorded in the review report).
2. **Schema decisions (executed).** For each `schema_valid` case, apply its
   `edit` and validate; for each `schema_invalid` case, apply it, validate, and
   compare `malformed_wire` and the first schema error path with the case's
   `locus`.
3. **Reader (planned).** For each `positive` and `reader_refusals` case, apply the
   `edit` to its package, read it with the strict read, then run the carrier
   check. A case whose package names `dependencies` (`PC-ROOT`) is read once per
   named package against its own member and `source_map`. Compare the decision,
   the typed read (`complete` with its groups, each holding the whole
   `source_map` entry, or `unavailable_correlation_provenance`) or the refusal's
   code, cause and locus; for a coverage refusal compare the retained control,
   occurrence and first uncovered `canonical_index` of `expect.retained`.
4. **Budget (planned).** For each `budget.cases` case, measure the baseline
   consumption of the work and read-storage counters of the FR-358
   `NativeJoinBudget` immediately before the step on the same input, then run the
   check with the limit at the baseline plus the case's offset (`G` is the
   measured size of the typed-view block) and compare the admission or the
   `incomplete` result naming the counter, the first unpaid amount and the locus,
   with no partial typed view. `CO-B13` runs with earlier consumption on the same
   budget and expects no reset; `CO-B15` shows control bytes charged before the
   join refuses.
5. **Producer (planned).** Lower real checked source with an authored clause
   written once, written three times and written in the opposite order (the FSL
   producer, over the checked source inputs of its existing source-invariance
   tests where they already ground the canonical channels), and compare the
   packages: every member but `protocol_correlation_occurrences` equal, the
   member's rows as FR-831-AC-8 states. A static pair built from one package with
   two member edits (step 7) does not stand in for this step.
6. **Consumer (planned).** For each `consumer.cases` case, give a consumer the
   package and the control it needs and compare `refuse`/`proceed`, the refusal
   name and the clauses it receives. The existing FSL guard stays until the
   capability and pin land.
7. **Vector consistency (executed).** Compute from each package the eligible
   control occurrences and their `related` lengths, the full structural key order
   of the rows, the per-group ordinals, the `source_map` lookup by the full key and
   the first uncovered canonical position, and compare them with the positive
   rows, the group `source_map_entry` and clauses, the refusal rows and the
   `retained` data (and `CO-R17`'s two simultaneous defects); recompute the
   charges of each budget case from its package and rows and compare every
   expected counter, first unpaid amount and locus; compare the packages of each
   `invariance` case outside the member.

## Expected Results

- Steps 1, 2 and 7: every package and edit validates, every `schema_invalid` edit
  fails at its stated path, every recorded datum equals the value computed from
  its package, and a mutated key, ordinal, occurrence, package or source map
  fails the consistency check.
- Step 3: every admitted case returns its stated typed read; every refusal
  returns the first defect in FR-831 order and no partial view; an absent member
  is `unavailable_correlation_provenance` and the package is admitted.
- Step 4: the exact limit admits and a limit one short returns `incomplete` at the
  stated first unpaid charge.
- Step 5: only the member differs.
- Step 6: an absent member over a nonempty `related` is refused by name and no
  empty premises are used.

## Boundaries

- The packages are real: relationship `Order-order-Order` of `acme/orders`, ends
  `view` and `notified` typed `Reference<Order>`, canonical `related` records on
  `send`, `receive`, `attempt`, `effect`, a new `event` control and a new
  `recommend` control. A control has one declaration occurrence, so no
  two-occurrence package exists; `absent_by_standard` records the evidence and
  the join is exercised by a row naming occurrence ordinal 1 (`CO-R08`).
- Steps 3 to 6 are not executed anywhere, and the criteria they verify
  (FR-831-AC-1 to AC-5, AC-7 to AC-9 and AC-12) carry no trace tag until they are.
- The vectors check a reader's consistency checks. They do not prove that a
  producer recorded every authored clause, an omitted trailing duplicate, or the
  exact alias spelling, and they do not execute FSL FR-731's first-failing-clause
  replay, whose source-order oracle is unchanged.
