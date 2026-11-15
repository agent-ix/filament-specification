---
id: TC-1644
title: "Release versions follow the bump rules"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1644: Release versions follow the bump rules

## Description

Inspect releases against the breaking and compatible classes. Scope: FR-660-AC-5.

## Test Procedure

For each release, list its changes since the previous release and classify each as breaking or compatible under FR-660. Compare the release's version position and its notes.

## Expected Results

- A release with any breaking change carries a breaking version and its notes name each breaking change with the requirement, case or fact it changes.
- A release with only additions carries a compatible version.
