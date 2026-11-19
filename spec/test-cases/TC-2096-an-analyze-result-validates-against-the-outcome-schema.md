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

Verify that an analyze result validates against the published
`filament-outcome/1` schema `urn:filament:outcome:1`, is canonically
encoded, and does not depend on the request's member order. Scope:
FR-520-AC-8.

## Test Procedure

1. Run FR-520-AC-1's request and validate the result against
   `urn:filament:outcome:1`; re-encode it under RFC 8785.
2. Mutate the result three ways: remove `/items/0/request_index`; set
   `/items/0/category` to `"proven"`; remove `aggregate`. Validate each.
3. Run FR-520-AC-1's request with the members of every object reversed.

## Expected Results

- Step 1: the result validates and equals its re-encoding.
- Step 2: errors at `/items/0`, `/items/0/category` and the root pointer.
- Step 3: the result is byte-equal to step 1's.
