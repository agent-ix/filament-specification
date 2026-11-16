---
id: TC-1811
title: "A package that writes no node refuses at check"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-746
    type: verifies
---

# TC-1811: A package that writes no node refuses at check

## Description

Verify that check and compile refuse a package with no node in the same way, naming the rule. Scope: FR-746-AC-2.

## Test Procedure

Check, then compile, three units under the TC-1810 header: one whose only declaration is `type I = Integer;`, one of `type I = Integer;` and `type J = Boolean;`, and one with only its header.

## Expected Results

- Each check refuses `missing_declaration`/`empty-package`, located at the unit's header, retaining the package name and the unit's source identity, with no checked package.
- Each compile refuses with the same code, cause and locus, and with no wire refusal.
- The refusal message names the rule (a checked package needs at least one declaration that has a node; a type alias has none) and does not say that a node cannot be written to the wire.
