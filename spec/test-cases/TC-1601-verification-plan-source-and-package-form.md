---
id: TC-1601
title: "Verification plan source form and package form"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-192
    type: verifies
---

# TC-1601: Verification plan source form and package form

## Description

Verify that a `verify` declaration checks to plan nodes bound to clause identities, its refusals, and its linked package form. Scope: FR-192-AC-8 to FR-192-AC-11.

## Test Procedure

1. Check the `Release` plan of FR-192 "Source form" and read its emitted `filament.verification-plan/v1` document.
2. Check each mutation FR-192-AC-9 and FR-192-AC-10 list, one at a time.
3. Compile the unit with and without the plan and compare checked package identities; execute the plan document against a package with another identity.

## Expected Results

- Step 1: three nodes, their clause identities, methods, bounds and the one `requires` edge, in the document.
- Step 2: each refusal with the cause and locus stated.
- Step 3: equal package identities; the mismatched execution refuses naming both identities.
