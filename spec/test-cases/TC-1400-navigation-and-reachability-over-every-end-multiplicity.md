---
id: TC-1400
title: "Navigation and reachability over every end multiplicity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-152
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1400: Navigation and reachability over every end multiplicity

## Description

Verify FR-152-AC-13 and FR-322-AC-57's destination-multiplicity step: unbounded and ordered ends navigate, ordered ends read instance positions, an end with no multiplicity refuses navigation as a typed semantic refusal, and reachability admits every multiplicity.

## Test Procedure

Use the model and observations named by FR-152-AC-13 through the real local Rust boundary; check and evaluate each navigation and reachability, and admit each malformed-position extent.

## Expected Results

Each result type, order, refusal and Boolean matches FR-152-AC-13; no case refuses `unsupported_construct`.
