---
id: TC-1644
title: "The resolved standard dependency carries a version promise"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1644: The resolved standard dependency carries a version promise

## Description

Verify, offline, that a consumer resolves the standard to a release whose version Cargo's semantic-version rules apply to. Scope: FR-660-AC-5.

## Test Procedure

Run `cargo metadata --locked --offline --format-version 1` in a consumer and read the source and version of `filament-specification-qualification`. Repeat on a scratch consumer whose manifest names the standard with `branch = "main"`.

## Expected Results

- The consumer's source is a git source with `tag=vX.Y.Z`, or a registry, and the version is `X.Y.Z`, equal to the tag without its `v`.
- The scratch consumer's `branch` source fails, naming the dependency.
