---
id: TC-2096
title: "An analyze result validates against the outcome schema"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
---

# TC-2096: An analyze result validates against the outcome schema

## Description

Verify that an analyze result, conformance and simulation items included,
validates against the published `filament-outcome/1` schema
`urn:filament:outcome:1`, is canonically encoded, and does not depend on
the request's member order where its refusal does not. Scope: FR-520-AC-8.

## Test Procedure

1. Run FR-520-AC-1's request and validate the result against
   `urn:filament:outcome:1`; re-encode it under RFC 8785.
2. Mutate the result three ways: remove `/items/0/request_index`; set
   `/items/0/category` to `"proven"`; remove `aggregate`. Validate each.
3. Run FR-520-AC-1's request, and FR-520-AC-2's request whose item carries
   only `request_index`, each with the members of every object reversed.
4. Run an FSL FR-990-AC-1 `global-conformance` request, an FSL FR-990-AC-3
   request with a recording, and an FSL FR-634-AC-1 `simulation` request;
   validate each result.

## Expected Results

- Step 1: the result validates and equals its re-encoding.
- Step 2: errors at `/items/0`, `/items/0/category` and the root pointer.
- Step 3: each result is byte-equal to the result of the request as
  written.
- Step 4: every result validates.
