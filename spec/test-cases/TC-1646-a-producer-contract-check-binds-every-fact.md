---
id: TC-1646
title: "A producer's contract check binds every fact"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-661
    type: verifies
---

# TC-1646: A producer's contract check binds every fact

## Description

Verify the producer's contract check over its pinned inventory and binding document. Scope: FR-661-AC-3.

## Test Procedure

Run the producer's contract check over its committed binding document. Then run it over copies whose binding names a nonexistent or non-public accessor for one fact, omits one fact, and binds an id the inventory does not hold.

## Expected Results

- The committed bindings pass and the report lists every fact with its resolved public path.
- The three copies fail, naming respectively the fact and path, the unbound fact, and the unknown binding.
