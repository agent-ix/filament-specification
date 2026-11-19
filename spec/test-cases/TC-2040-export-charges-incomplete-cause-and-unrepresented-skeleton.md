---
id: TC-2040
title: "Export charges, the incomplete cause and the skeleton of an unrepresented obligation"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-817
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-811
    type: verifies
---

# TC-2040: Export charges, the incomplete cause and the skeleton of an unrepresented obligation

## Description

Verify the `limit-reached` cause of an incomplete export, that shared closure nodes are charged once, that an unrepresented obligation's representable nodes are still written, and the `null` locus of an unlocated node. Scope: FR-817-AC-11, FR-817-AC-12, FR-811-AC-7 and FR-811-AC-8.

## Test Procedure

1. Export the counter's `Bounded` under `export.work_units` 1, then under `export.nodes` 1 and `export.work_units` 1.
2. For two invariants over one state model, find the least `export.nodes` under which each alone and both together export without stopping `incomplete`.
3. Export `SlowEnough` over `Braking` under `minimum_fidelity` `any`; read its record, text and `name_map`.
4. Read every `target-unrepresentable` cause's `locus` from step 3.

## Expected Results

- Step 1: `incomplete`, `records` `[]`, `cause` `limit-reached{limit: work_units, value: 1, setting: export.work_units}`, exit 22; then `limit-reached{limit: nodes, value: 1, setting: export.nodes}`.
- Step 2: the value for both is less than the sum of the two single values and at least the greater of them.
- Step 3: the record claims no region; the text holds an `Init` naming `Moving` and the jump to `Stopped`, each with a `name_map` entry; each closure node is named by a cause or a `name_map` entry.
- Step 4: each `locus` is `null` or a region with `start < end`, and `null` exactly for a node with no source-map region.
