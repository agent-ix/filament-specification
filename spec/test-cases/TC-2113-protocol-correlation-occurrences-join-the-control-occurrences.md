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
member of `protocol-correlation-vectors.json` and refuses each mutation with the
stated code, cause and locus, in the order of FR-831. Scope: FR-831-AC-1 to
FR-831-AC-8, FR-322-AC-514 and FR-466-AC-20. No reader or harness implements
FR-831 yet, so the vectors are inputs and expected results and this case is not
executed; no existing reader is qualified against it.

## Test Procedure

1. For each `schema_valid` case, validate its `value` as the member against the
   root V2 `schema.json`; for each `schema_invalid` case, validate it and compare
   `malformed_wire` and the first schema error path with the case's `locus`.
2. For each case of `absence`, `positive` and `reader_refusals` whose `edit` is
   not null, apply the `edit` (RFC 6902 `add`, published form) to
   `fixtures/protocol-node-base.json` and read the package with the strict read,
   then run the carrier check. A case whose `edit` is null is an admitted-graph
   projection: build the check's input from the case's `projection`, `controls`
   and `related_len` (each eligible control occurrence's control `NodeRef`,
   operator, declaration occurrences, admitted `source_map` entry of the full key
   and length of its emitted `related` array) and the case's `rows`. A case with
   `packages` is read once per named package, each against its own member and
   `source_map`. Compare the decision, the typed read (`complete` with its
   groups and `source_units`, or `unavailable_correlation_provenance`) or the
   refusal's code, cause and locus.
3. For each `budget.cases` case, run the carrier check with the case's
   `exact_limit` as the `work` limit and then with `one_less_limit`, and for each
   of `CO-B2` and `CO-B3` with its `limit_value`; compare admission or the
   `incomplete` result naming `work` at the stated locus, with no partial typed
   view.
4. For `invariance` cases, assert the listed members are byte-equal and the
   listed members differ between the two packages (`CO-I2` over the base is
   executable; `CO-I1` compares the canonical graph of two projections).

## Expected Results

- Step 1: every `schema_valid` value validates; every `schema_invalid` value
  fails at its stated path.
- Step 2: every admitted case returns its stated typed read; every refusal
  returns the first defect in FR-831 order and no partial view; an absent member
  is `unavailable_correlation_provenance` and the package is admitted.
- Step 3: the exact limit admits and the one-short limit returns `incomplete`
  naming `work` at the stated locus.
- Step 4: the member alone differs.

## Boundaries

- Projection cases for `event`, `recommend` and the two-occurrence send name
  controls the base package lacks; their full-package embodiment, and the
  full-package embodiment of every case with a nonempty `related`, is the
  producer's fixture and is not published here.
- The vectors check a reader's consistency checks. They do not prove that a
  producer recorded every authored clause, an omitted trailing duplicate, or the
  exact alias spelling, and they do not execute FSL FR-731's first-failing-clause
  replay, whose source-order oracle is unchanged.
