---
id: TC-1910
title: "A clause-run ledger carries record-sample fingerprints"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
---

# TC-1910: A clause-run ledger carries record-sample fingerprints

## Description

Check the `fingerprint` member of a clause-run ledger's covered entry: present
exactly on record-sample entries, of the `sha256:` form, restored without loss,
and no sample text or value in the ledger.

Scope: FR-352-AC-20.

## Test Procedure

1. Write the record of a clause run over a `progressed` record-sample trace with a completeness assertion, and one over a snapshot trace.
2. Read both records back.
3. Read variants: a record-sample entry without `fingerprint`, a snapshot entry with one, a `fingerprint` with uppercase digits, one with 63 digits, and a covered entry with an extra member.

## Expected Results

- Step 1: each record-sample covered entry is `{identity, stamp, fingerprint}` with the index as `identity`; the snapshot entries carry no `fingerprint`; no sample text or value appears.
- Step 2: both records restore without loss.
- Step 3: each variant is refused by the reader, naming the member.
