---
id: FR-111
title: "Correlate related workflow instances only through declared relationships"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-009
    type: implements
  - target: ix://agent-ix/filament-specification/FR-262
    type: references
  - target: ix://agent-ix/filament-specification/FR-287
    type: references
---
## Description

When associating an observation with a workflow subject, the correlation service SHALL select only an explicit typed relationship whose
relationship declaration is an effective domain declaration of the checked
package, with its exact declaration key and ordered endpoints.
The correlation outcomes are exactly `resolved`, `missing`, `conflicting` and
`ambiguous`; the latter three are typed non-success outcomes rather than
heuristic matches. An explicitly partial relationship extent contributes an
`IncompleteRelationshipExtent{required}` reason to the existing incomplete
admission result; it does not add a fifth correlation outcome.

## Behavior

The caller SHALL supply required relationship slots as a list that the
correlation service interprets as a set. `max_required_relationships` SHALL
bound the raw list length before equal entries are collapsed. Equal required
slots identify one slot.
The service SHALL evaluate distinct slots in canonical order of the complete
required-slot key: declaration identity, producer authority key
(`bundle_identity`, `bundle_revision`, `digest`), `interface_version`, source
FR-287 key, then target FR-287 key. Each key component uses its exact identity
comparison, and source precedes target. When admission reaches slot evaluation,
an unlisted or complete-empty extent with no candidate adds one
`MissingRelationship{required}` reason, while an explicitly partial extent
adds one `IncompleteRelationshipExtent{required}` reason even when a matching
candidate is present, unless an incompatible candidate establishes
`conflicting`. Both reasons follow canonical slot order. A refusal returns
alone and drops accumulated incomplete reasons.
An incomplete result reached before slot evaluation is returned unchanged and
the resolver performs no identity-rebind or slot checks after that point.
Reference construction/validation precedes resolver admission; its
`InvalidReference` refusal therefore precedes any pre-slot incomplete result.

Workflow instances, messages, receive occurrences, deliveries, attempts,
effects and receipt observations SHALL remain distinct. A receipt refers to a delivery;
it is not the message, receive occurrence or delivery itself. A duplicate receipt SHALL NOT establish a second business effect. A refund is admitted only with its
compensated effect and order relationship; a telemetry trace ID or shared
provider identity does not establish either relationship.
The correlation service SHALL refuse a missing domain package identity, a
relationship declaration or kind outside the checked package, or a declaration key
that does not name a declaration of the selected package rather than reconstructing a relationship
from a foreign key, trace ID, provider name, or ambient configuration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-111-AC-1 | A refund linked to O1 is refused for O2 even when both orders share a provider or trace infrastructure. | Test (TC-141) |
| FR-111-AC-2 | Missing, conflicting and ambiguous required relationships yield three distinct non-success correlation outcomes; missing is incomplete, while conflicting and ambiguous refuse. `unknown` is not a relationship outcome. `IncompleteRelationshipExtent{required}` is a reason in the existing incomplete admission result, not a fifth correlation outcome. | Test (TC-141) |
| FR-111-AC-3 | Repeated delivery is retained as a delivery and cannot alone create a second attempt or effect. | Test (TC-141) |
| FR-111-AC-4 | A missing relationship declaration key, a declaration outside the checked package, or a declaration key naming a different domain package identity is refused without reconstructing a relationship. | Test (TC-141) |
| FR-111-AC-5 | Each endpoint retains its complete FR-287 key in authored source/target order; reversing, sorting, emptying, foreign-qualifying or substituting an endpoint refuses. | Test (TC-141) |
| FR-111-AC-6 | Exact replay of one complete relationship value is idempotent, while multiple distinct admitted values satisfying one required slot are ambiguous. | Test (TC-141) |
| FR-111-AC-7 | A receipt observation, delivery, receive occurrence and message remain non-interchangeable even when they share transport data. | Test (TC-141) |
| FR-111-AC-8 | The raw required-slot list is bounded by `max_required_relationships` before duplicates collapse; equal slots are one set member; distinct slots are evaluated in canonical key order (declaration identity, producer authority key, interface version, source FR-287 key, target FR-287 key). When slot evaluation completes without refusal, an unlisted or complete-empty extent without a candidate contributes `MissingRelationship{required}`, while an explicitly partial extent contributes `IncompleteRelationshipExtent{required}` even with a matching candidate unless an incompatible candidate establishes `conflicting`; reasons follow canonical slot order. A later refusal returns alone and discards those reasons, and an incomplete result from earlier admission returns unchanged without slot evaluation or identity-rebind checks. Reference-construction refusals precede resolver admission. The extent reason is not a fifth correlation outcome. | Test (TC-141, `filament-observation/tests/admission.rs::tc005_required_slot_limit_counts_raw_entries`, `filament-observation/tests/admission.rs::tc005_required_slots_are_a_canonical_set`, `filament-observation/tests/admission.rs::tc005_slot_refusal_discards_incomplete_reasons`, `filament-observation/tests/admission.rs::tc005_pre_slot_incomplete_skips_relationship_slots`, `filament-observation/tests/admission.rs::tc005_pre_slot_incomplete_precedes_identity_rebind`, `filament-observation/tests/admission.rs::tc005_reference_construction_precedes_pre_slot_incomplete`, `filament-observation/tests/admission.rs::tc005_relationship_extent_completeness_controls_resolution`) |

## Dependencies

- [Shared foundation](../../../proposals/filament-v1/shared-foundation.md).
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md): relationship declarations are spec-declared and keyed by domain package identity and IR node identity (the package selection carries identity and version); B owns protocol subject meaning.
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  supplies each ordered endpoint key.

## Status

Accepted by FS04 for explicit relationship correlation and ambiguity/refusal.
Observation #18/#13 retain executable declaration-key/topology adoption.
