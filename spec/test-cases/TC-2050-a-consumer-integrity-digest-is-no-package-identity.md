---
id: TC-2050
title: "A consumer integrity digest is no package identity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---
# TC-2050: A consumer integrity digest is no package identity

## Description

Verify that an integrity digest a consumer holds over a checked package's
bytes, for a foreign interface that requires one, never becomes a package
identity, version or sameness test, and is never written to a package, plan
or outcome document. Scope: FR-322-AC-470.

## Test Procedure

1. Compile one package, emit it, read it back and run a temporal evaluation
   over it whose evaluator admits the trace to filament-observation with the
   SHA-256 of the package bytes as the owner's package digest.
2. Inspect the emitted package, the run's plan and outcome documents.
3. Compare the package with a structurally equal copy and with a copy that
   differs in one node, with the consumer's digest present and with it
   removed from the evaluator's state.

## Expected Results

- Step 2: no package, plan or outcome document carries the digest.
- Step 3: the equal copy is the same package and the differing copy is not,
  in both cases, and the answers do not change when the digest is removed.
