---
id: TC-1784
title: "Protocol recording documents are read, ordered causally and keyed by identity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-737
    type: verifies
---

# TC-1784: Protocol recording documents are read, ordered causally and keyed by identity

## Description

Catches causal order taken from array position or timestamps.

Scope: FR-737-AC-1, FR-737-AC-2, FR-737-AC-3.

## Test Procedure

1. Read a well-formed recording, then each malformed variant; reorder its observations consistently; change only its clock readings; read message, delivery and effect keys with repeats and contradictions.

## Expected Results

- Step 1: The well-formed document reads and each defect is refused at its pointer; causal order is unchanged by reordering and by clock changes; two deliveries of one message, one replayed effect, and a refused contradiction.
