---
id: TC-989
title: "Native fence unowned identity and whole-document failure outcomes"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-563
    type: verifies
---

# TC-989: Native fence unowned identity and whole-document failure outcomes

## Description

Verify headless native-fence body identity and whole-document failure
serialization for the Markdown `check` command.

## Test Procedure

1. Check a Markdown document whose opening `ix:native` fence is on line 3 and
   precedes every heading, followed by an owned native fence. Check a second
   document with the headless fence moved to another line.
2. Inject, independently, whole-document extraction refusal, a reached
   extraction/check limit, cancellation, and an internal fault.

## Expected Results

The first headless body is `doc#unowned-3`, with no clause; moving the fence
changes its line-derived identity and its reported source location, while the
semantic outcome is otherwise the same. Each whole-document failure has
operation `check`, no items and one operation-level diagnostic. Extraction
refusal is category `refusal`, no stage, exit 20; limit is `incomplete`, names
its catalog code, cause and reached stage, exit 22; cancellation is
`incomplete`, no stage, names its cancellation code and cause, exit 22; fault
is `internal_failure`, no stage, names its fault catalog code, exit 30.
