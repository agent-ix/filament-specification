---
id: TC-1701
title: "A model-trace clause-run record carries the limits it ran under"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
---

# TC-1701: A model-trace clause-run record carries the limits it ran under

## Description

Check the clause-run record's `limits` member: present with every effective
evaluation limit, defaults explicit, exactly for a run over a `model` trace,
so two runs differing only in a limit write unequal records.

Scope: FR-352-AC-19.

## Test Procedure

1. Write the records of two clause runs over one `model` trace, one with no `max_candidates` in the request and one with `max_candidates` 1, and one record over a `finite` trace.
2. Read a `model`-trace record with `limits` removed, the `finite`-trace record with `limits` `{"expansion.max_candidates": "1000000"}` added, and `model`-trace records whose `limits` has an unknown setting name or a value `"0"`, `"01"` or `1`.

## Expected Results

- Step 1: `limits` is `{"expansion.max_candidates": "1000000"}` and `{"expansion.max_candidates": "1"}`; the two records are unequal; the `finite`-trace record has no `limits`.
- Step 2: each variant is refused by the reader.
