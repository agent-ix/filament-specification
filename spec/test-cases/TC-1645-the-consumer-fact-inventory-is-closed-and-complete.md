---
id: TC-1645
title: "The consumer-fact inventory is closed and complete"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-661
    type: verifies
---

# TC-1645: The consumer-fact inventory is closed and complete

## Description

Verify the inventory's closed shape and the facts it must list. Scope: FR-661-AC-1, FR-661-AC-2.

## Test Procedure

Read `proposals/consumer-facades/consumer-facts.json`. Check the `inventory` tag, the consumer order, id codes and ordinals, id uniqueness, and that each `requirement` names an existing requirement artifact in its repository. Check the facts listed by FR-661-AC-2 are present. Then check copies with an extra member, a missing consumer, two consumers swapped, a repeated id and a requirement that names no artifact.

## Expected Results

- The published inventory is admitted and lists every fact FR-661-AC-2 names.
- Each mutated copy is refused, naming its defect.
