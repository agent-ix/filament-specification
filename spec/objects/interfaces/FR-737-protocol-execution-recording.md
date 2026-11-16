---
id: FR-737
title: "Protocol execution recording document"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-059
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-035
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-287
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-247
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-288
    type: depends_on
---
# FR-737: Protocol execution recording document

## Description

A recorded execution of a deployed system, assessed for global conformance
(FR-059), SHALL be supplied as one `filament.protocol-recording/1` document.
A conformance evaluator SHALL read the document as below and SHALL refuse a
document that breaks a rule with `invalid_runtime_input`, naming the JSON
Pointer of the first offending member.

## Members

| Member | Value |
| --- | --- |
| `format` | the text `filament.protocol-recording/1` |
| `protocol` | the FR-288 qualified definition key of the protocol |
| `observations` | an array, each entry one observation (below) |
| `local_order` | per role, the array of observation indices in the order that role performed or observed them |
| `correlations` | an array of `{send, receive}` pairs of observation indices, each pairing a receive with the send of the message it received |
| `closure` | an FR-247 complete-global-conformance closure record, with its state and premise identities |

An observation is a closed object:

| Member | Value |
| --- | --- |
| `kind` | one of `send`, `receive`, `attempt`, `effect`, `event`, `commit`, `compensation-attempt`, `recovery`, `activate`, `spawn`, `retire`, `finish` |
| `node` | the qualified name of the protocol node it occurs at |
| `role` | the role name, and for a replicated role an `instance` object reference |
| `instance` | the FR-287 workflow-instance reference |
| `keys` | the FR-287 runtime references the kind requires: a message key and a delivery key for a send or a receive, an attempt key, a business-effect key, a compensation-attempt key and its registration |
| `record` | the bound record, in the canonical value form of the node's record type |
| `outcome` | for an attempt or a compensation attempt, an FR-236 per-attempt label |
| `clocks` | an optional object mapping clock names to exact rational readings |

## Rules

- **Causal order.** The recording's causal order is the transitive closure
  of `local_order` and `correlations`. No timestamp, clock reading, array
  position or key order supplies causal order (FR-226 "Causal order").
- **Absence.** An absent observation is never defaulted. An absent
  `clocks` reading leaves a window undecided, never satisfied.
- **Identity.** One complete key re-observed with equal facts is replay of
  one subject. The same key with different facts is an identity
  contradiction, and the document is refused (FR-058).
- **Closure.** Complete global success needs `closure` in state `closed`
  with every required premise (FR-059-AC-5).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-737-AC-1 | A document with each member well formed is read. A missing `protocol`, an observation `kind` outside the list, a `local_order` index with no observation, and a correlation pairing two sends are each refused at their pointer. | Test (TC-1784) |
| FR-737-AC-2 | Reordering `observations` while renumbering `local_order` and `correlations` consistently yields the same causal order, and changing only clock readings changes no causal edge. | Test (TC-1784) |
| FR-737-AC-3 | One message key with two delivery keys reads as two deliveries of one message; one business-effect key repeated with equal facts reads as one effect; the same key with a different record is refused as an identity contradiction. | Test (TC-1784) |

## Dependencies

- [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md):
  the assessment that reads the recording.
- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md),
  [FR-287](../protocol/FR-287-authority-qualified-runtime-subject-reference.md),
  [FR-247](../protocol/FR-247-complete-global-conformance-closure-state.md)
  and [FR-288](../protocol/FR-288-qualified-protocol-definition-identity.md).
