---
id: TC-1244
title: "Request-surface commands of the CLI"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-301
    type: verifies
---

# TC-1244: Request-surface commands of the CLI

## Description

Verify the request-surface commands FR-301 lists reach their features and refuse malformed input before any stage. Scope: FR-301-AC-5.

## Test Procedure

1. Run `version --format json`, `check --markdown` over a document of two native fences, and `compile` with a `handoff` member followed by `inspect --handoff` over the written directory.
2. Run `prove --provider-config` naming a document with no `providers`, and `check --extensions` naming a set document whose entry has an unknown member.

## Expected Results

- Step 1: the inventory document; one item per fence; the handoff directory is written and `inspect --handoff` exits `0`.
- Step 2: each exits `20` before any stage runs, empty stdout.
