---
id: TC-2005
title: "An analyze subject carries a protocol binding"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
---

# TC-2005: An analyze subject carries a protocol binding

## Description

Verify that a `native-analyze/1` subject's `protocol` member selects a
protocol subject, that the request writer carries it into the FR-331 item, and
that the binding is closed. Scope: FR-520-AC-6.

## Test Procedure

1. Read a request whose `subject.protocol` binds `SB` with `memory` selecting
   `SB::Both` as `tso`; inspect the written FR-331 request item.
2. Read the same request with an unknown binding member.
3. Read the claim with no `protocol` member.

## Expected Results

- Step 1: `temporal.subject.protocol` with the defaulted `over` and the
  effective model.
- Step 2: `invalid-request` naming the member.
- Step 3: a model-subject item.
