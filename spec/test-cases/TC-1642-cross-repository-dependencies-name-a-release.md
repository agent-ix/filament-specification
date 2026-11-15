---
id: TC-1642
title: "Cross-repository dependencies name a release"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1642: Cross-repository dependencies name a release

## Description

Verify the pin rule on committed manifests and that no version record exists outside Cargo. Scope: FR-660-AC-3.

## Test Procedure

Run the consumer's pin check over its committed manifests, then over a scratch copy whose manifest adds a Filament git dependency with `branch = "main"`. Search every file other than `Cargo.toml` and `Cargo.lock` for a recorded expected release version, hash or digest of another repository.

## Expected Results

- The committed manifests pass.
- The scratch manifest fails, naming the `branch` dependency.
- No version, hash or digest record of another repository is found outside the Cargo files.
