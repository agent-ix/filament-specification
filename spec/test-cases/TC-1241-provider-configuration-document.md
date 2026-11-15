---
id: TC-1241
title: "Provider-configuration document"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-561
    type: verifies
---

# TC-1241: Provider-configuration document

## Description

Verify the provider-configuration document and its refusals. Scope: FR-561-AC-1 to FR-561-AC-3.

## Test Procedure

1. Run `prove --provider-config` naming a fixture plugin by a relative path; then with `providers` `[]`.
2. Give a document with no `providers`, an entry with member `env`, an entry with no `path`, `format` `filament.provider-config/v2`, and a nonexistent document path.
3. Run `parse --provider-config`.

## Expected Results

- Step 1: the plugin starts, registers and receives only negotiated items; `[]` runs with linked providers only.
- Step 2: each refuses as FR-561 Behavior states, exit `20`, empty stdout, no plugin started; the missing `providers` refusal names the file and `providers`.
- Step 3: unknown option, exit `20`.
