---
id: TC-1564
title: "Required probability analysis items"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-641
    type: verifies
---

# TC-1564: Required probability analysis items

## Description

Verify that each required probability item class is accepted and returns its result members and evidence.

Scope: FR-641-AC-1 to FR-641-AC-3.

## Test Procedure

1. Run `P95` under `Steady` with `okamoto`, `sprt` and `clopper-pearson`, and `LongRun` with `regenerative`; repeat each with the same seed.
2. Run one exact item per class row under a workload and under every scheduler; check each certificate with a checker in a separate process without the provider, and replay each counterexample.
3. Check the unbounded exact claim and its statistical item, and the `[0, *]` spelling.

## Expected Results

- Step 1: `measured` results naming method and seed; equal results for equal seeds.
- Step 2: each proof's certificate accepted, each refutation replayed; kinds `exact`, `lower`, `upper` and `long-run` all appear.
- Step 3: the claim admitted; the statistical item `unsupported`, `exact-only-form`; `[eventually[0, *] holds(p.done)]` refuses `unbounded-event`.
