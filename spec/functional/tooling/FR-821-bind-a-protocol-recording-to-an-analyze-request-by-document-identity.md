---
id: FR-821
title: "Bind a protocol recording to an analyze request by document identity"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-520
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-737
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-059
    type: references
  - target: ix://agent-ix/filament-specification/FR-247
    type: references
---

# FR-821: Bind a protocol recording to an analyze request by document identity

## Description

When an analyze request (FR-520) supplies an FR-737 protocol recording for a
`global-conformance` item, the reading tool SHALL provision the recording
under one `{authority, identity}` document identity: the identity the
recording declares in its own `identity` member, or the identity the
request's `documents` entry states for it, and SHALL refuse the request when
the two disagree, when either is malformed, when the recording has neither,
or when a second document takes the same identity. The item names the
recording by that identity in its `recording` member, and the identity is
recorded on the item's result.

## Inputs

- An FR-520 `native-analyze/1` request whose `documents` entries are
  `{file}` or `{file, identity}`.
- An FR-737 recording, with or without its own `identity` member.

## Outputs

The recording provisioned under one document identity and named by the
item, or a request refusal at the request stage.

## Behavior

### Where the identity comes from

- An FR-737 recording MAY carry `identity`, a closed object `{authority,
  identity}` of two non-empty strings, the same spelling a
  `filament.state.snapshot/v1` document uses for its own identity. The
  recorder of the deployed system writes it.
- An FR-520 `documents` entry MAY carry `identity` in the same spelling. It
  states the identity the caller binds the file to. A recorder that writes no
  identity is bound this way, so a recording needs no change to be supplied.
- The document's **provisioned identity** is its declared identity when it
  has one, otherwise the stated identity. When both are present they SHALL be
  equal componentwise; the stated identity then only confirms it.
- A snapshot or finite-trace document keeps FR-520's rule: it is provisioned
  under the identity it declares, and a stated identity on its entry SHALL
  equal it.

### How the item uses it

- A `global-conformance` item carries `recording`, the `{authority,
  identity}` of a provisioned recording, to be assessed in replay mode
  (FR-059 over one recorded history). An item of another kind that carries
  `recording` is refused.
- The item's request identity covers the provisioned identity and the
  recording's bytes, so two requests that bind the same bytes under different
  identities, or different bytes under one identity, are different requests.
- The item's result carries `recording`, the provisioned identity, beside
  its per-obligation results and closure record, so a reader of the result
  knows which recorded history was assessed. A counterexample over the
  recording is a prefix of that document.

### Document identity and closure-premise identity

The document identity names the recording as a whole. It is not part of any
closure-premise identity `{kind, instance, node[, role_instance]}` (FR-247),
whose components are authored protocol facts and the recording's FR-287
runtime keys. Premise identities are compared without the document identity,
so the same recording bound under two document identities yields equal
premise sets and equal per-obligation verdicts. Neither identity is derived
from the other: an observation's `instance` is an FR-287 runtime key, never
the document identity, and the document identity is never read from a
premise.

### Refusals

Each refusal below refuses the whole request at the request stage with
`invalid-request`, exit 20, naming the member by its RFC 6901 pointer, before
any item is assessed:

| Defect | Locus |
| --- | --- |
| A stated `identity` that is not a closed `{authority, identity}` object of two non-empty strings | `/documents/<i>/identity` |
| A declared `identity` in the recording that is not that object | `/documents/<i>`, naming the document's `/identity` |
| A stated identity that differs from the identity the document declares | `/documents/<i>/identity`, with the stated and declared identities |
| A recording with neither a declared nor a stated identity | `/documents/<i>` |
| A second document whose provisioned identity equals an earlier one's | `/documents/<i>` of the later entry |
| An item `recording` that is not that object, names no provisioned document, or names a document that is not an FR-737 recording | `/items/<j>/recording` |
| A `recording` on an item whose kind is not `global-conformance` | `/items/<j>/recording` |

A recording whose observations break FR-737's rules is not a request
refusal: the item is refused `invalid_runtime_input` at the first offending
member, as FR-737 states.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-821-AC-1 | A recording with no `identity` member, supplied as `{file, identity: {authority: "test", identity: "recording"}}` and named by a `global-conformance` item's `recording`, is assessed, and the item's result carries `recording` `{authority: "test", identity: "recording"}`; the same recording declaring that identity and supplied as `{file}` gives equal verdicts and result. | Test (TC-2001) |
| FR-821-AC-2 | The same recording bound under `{authority: "test", identity: "other"}` gives the same `closure.premises` identities and per-obligation verdicts, and a different request identity. | Test (TC-2001) |
| FR-821-AC-3 | A stated identity `{authority: "test"}`, a stated identity differing from the declared one, a recording with no identity supplied as `{file}`, and a recording provisioned under the identity of a supplied snapshot each refuse `invalid-request`, exit 20, at the pointer the table names, with no item assessed. | Test (TC-2001) |
| FR-821-AC-4 | An item `recording` naming no provisioned document, one naming a snapshot, and a `recording` on a `temporal-satisfaction` item each refuse `invalid-request` at `/items/0/recording`. | Test (TC-2001) |

## Dependencies

- [FR-520](FR-520-carry-analyze-requests-and-results-as-documents.md): the
  request's `documents` and items.
- [FR-737](../../objects/interfaces/FR-737-protocol-execution-recording.md):
  the recording document.
- [FR-059](../protocol/FR-059-assess-finite-global-conformance.md) and
  [FR-247](../../objects/protocol/FR-247-complete-global-conformance-closure-state.md):
  the assessment and its closure premises.
