---
id: TC-1641
title: "Consumers read the standard through the pinned library"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1641: Consumers read the standard through the pinned library

## Description

Verify that a consumer of the standard's published files resolves the standard as a pinned Cargo dependency and locates it in no other way. Scope: FR-660-AC-2.

## Test Procedure

In a consumer repository, run `cargo metadata --locked --format-version 1` and find the package `filament-specification-qualification`. Search the consumer's Makefile, build scripts, sources and tests for an environment-variable or filesystem-path lookup of the standard. Run the consumer's vector tests with no checkout of the standard next to the consumer.

## Expected Results

- The package's source is a git source carrying a `tag`, or a registry.
- No lookup of the standard by environment variable or path exists.
- The vector tests run and read every published file through the library.
