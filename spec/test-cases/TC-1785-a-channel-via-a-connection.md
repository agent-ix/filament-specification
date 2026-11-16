---
id: TC-1785
title: "A channel via a systems connection is admitted or refused by each rule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-738
    type: verifies
---

# TC-1785: A channel via a systems connection is admitted or refused by each rule

## Description

Catches a reader that accepts the wrong ports or direction.

Scope: FR-738-AC-1, FR-738-AC-2, FR-738-AC-3.

## Test Procedure

1. Read a channel node with a valid `connection`; each rule's violation; a channel with no `via`; a `related by` naming a connection.

## Expected Results

- Step 1: The valid channel is admitted with its dependency; each violation refuses with its code; the plain channel is byte-identical to its earlier form; the `related by` clause is refused at the name.
