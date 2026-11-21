---
id: FR-262
title: "Declared instance relationship"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/filament-specification/FR-111
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-213
    type: references
  - target: ix://agent-ix/filament-specification/FR-287
    type: references
---
# FR-262: Declared instance relationship

## Description

When an observation is associated with a workflow subject, the correlation service SHALL select only an explicit typed relationship carrying its domain package identity, exact relationship declaration key, and ordered authority-qualified endpoints.

FR-111 defines exactly `resolved`, `missing`, `conflicting` and `ambiguous`
correlation outcomes, and states that the three non-success outcomes replace
heuristic matching. It also states that the service refuses "a missing domain package identity, a
relationship declaration or kind outside the checked package, or a declaration
key naming a different domain package identity rather than reconstructing a
relationship from a foreign key, trace ID, provider name, or ambient
configuration".

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| relationship_type | String | 1..1 |  |
| domain_package_identity | String | 1..1 |  |
| relationship_declaration_key | String | 1..1 |  |
| related_instance_endpoints | AuthorityQualifiedRuntimeSubjectReference | 0..* |  |
| identity | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| relationship_type | type not stated in source; required key component (FR-262 Absence); see Field sources |
| domain_package_identity | type not stated in source; see Field sources |
| relationship_declaration_key | type not stated in source; see Field sources |
| related_instance_endpoints | ordered source and target, each one complete FR-287 key (FR-262-AC-5) |
| identity | Non-empty producer-supplied opaque bytes; when transported by FR-106, lowercase hexadecimal encoding (FR-181, FR-480). Required whenever FR-111 correlation reads the runtime instance. |


| Property | Source | Note |
| --- | --- | --- |
| Relationship type | FR-111 Description | "an explicit typed relationship". |
| Domain package identity | FR-111 Description | The domain package identity named by a checked package `model_root` node's owner identity; absence or mismatch refuses (FR-111-AC-4). |
| Relationship declaration key | FR-111 | Exact (domain package identity, IR node identity) key of the spec-declared relationship; a kind label does not replace it. |
| Related instance endpoints | FR-111 Behavior | Ordered source and target, each carrying one complete FR-287 key and satisfying the relationship declaration's endpoint type/direction. |

FR-111 Behavior states that "a telemetry trace ID or shared provider identity
does not establish either relationship", and FR-111-AC-1 that "A refund linked
to O1 is refused for O2 even when both orders share a provider or trace
infrastructure."

## Identity and comparison

- **Semantic identity** — *value*. A relationship has no semantic identity of
  its own; its semantic key is compared componentwise. It is not *derived*:
  FR-111 forbids reconstructing it "from a foreign key, trace ID, provider
  name, or ambient configuration".
- **Producer identity** — every supplied runtime relationship carries one
  non-empty producer-supplied opaque identity byte string. It is transport
  identity for detecting exact replay and identity rebinds, not a component of
  the semantic relationship key below. An exact replay with the same identity
  and facts is one value. Reusing an identity with different typed relationship
  facts refuses as `RelationshipIdentityContradiction` before slot evaluation.
- **Key** — the relationship type, domain package identity,
  exact relationship declaration key, and ordered
  source and target FR-287 endpoint keys.
  FR-111-AC-4 establishes the declaration key as
  key-participating: a missing, unknown or mismatched one refuses rather than
  resolves. Equality is componentwise
  over the key and nothing else; display name, source path, span, message text,
  locale and rendering never participate, and a trace ID, provider name, foreign
  key or ambient configuration is not a key component (FR-111 Behavior).
- **Key order** — lexicographic over the key in the declared sequence, using
  FR-287 endpoint order without sorting source and target. It serves canonical
  collection comparison only and carries no causal meaning: FR-113 states that
  "Arbitrary timestamp order SHALL NOT establish causality."
- **Causal order** — the declared relationship structure itself, and nothing
  else. It is never derived from key order, from a timestamp, from a telemetry
  trace ID or from a shared provider identity (FR-111 Behavior, FR-111-AC-1).
  A repeated transport fact adds no causal node: "A duplicate receipt SHALL NOT
  establish a second business effect" (FR-111 Behavior), and repeated delivery
  "cannot alone create a second attempt or effect" (FR-111-AC-3).
- **Absence** — every key component of an offered relationship is required;
  missing/empty, wrong-kind, foreign-authority, reversed or incompatible values
  refuse. Absence of any admitted candidate for a required slot is the distinct
  `missing` outcome with incomplete admission status, never a default
  relationship. A conflicting or
  ambiguous candidate refuses.

## Correlation outcomes and distinct subjects

The closed outcome set is `resolved`, `missing`, `conflicting`, `ambiguous`.
`missing` means no admitted candidate exists for a required slot and is
incomplete. An explicitly partial relationship extent contributes an
`IncompleteRelationshipExtent{required}` reason to the existing incomplete
admission result; it is not another correlation outcome. It contributes that
reason even if one or more matching candidates are present, unless a known
incompatible candidate establishes `conflicting`. `conflicting` means a
candidate with the required declaration,
producer authority and interface version, and the required source endpoint,
asserts a different target endpoint; it refuses. Reversing the declared
source/target direction fails relationship-reference construction and is an
`InvalidReference` before resolver admission. `ambiguous` means at least two
distinct admitted candidates match one required slot and refuses. For one
required slot, any conflicting candidate makes the result `conflicting`, even
when another candidate matches; conflict takes precedence over match count.
A relationship declaration or kind outside the checked package or a foreign
authority is an admission refusal, not another correlation outcome.

An unlisted relationship extent is not supplied. A complete empty extent is
known empty; when it leaves a required slot without a candidate, that slot is
`missing`. A `complete: false` extent is incomplete. When a required slot uses
an incomplete extent and no incompatible candidate already proves a conflict,
the resolver returns an `IncompleteRelationshipExtent` reason for that slot,
even if one or more matching candidates are present: unseen instances could
change resolution or ambiguity. A known incompatible candidate still makes the
slot `conflicting`.

Workflow instances, messages, receive occurrences, deliveries, attempts,
effects and receipt observations remain distinct. A receipt is an observation
record referring to a delivery and never aliases that delivery, the message or
the receive occurrence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-262-AC-1 | A relationship declared between O1 and its refund does not resolve for O2 sharing a provider or trace infrastructure. | Analysis |
| FR-262-AC-2 | A missing, foreign or identity-or-version-mismatched relationship declaration key does not resolve and is not reconstructed from a foreign key, trace ID, provider name or ambient configuration. | Analysis |
| FR-262-AC-3 | `resolved`, `missing`, `conflicting` and `ambiguous` are the complete unequal outcome set; missing is incomplete, conflicting/ambiguous refuse, and a declaration outside the checked package is an admission refusal rather than another relationship outcome. `IncompleteRelationshipExtent{required}` is a reason in the existing incomplete admission result, not a fifth correlation outcome. | Test (TC-141) |
| FR-262-AC-4 | Exact replay of one complete relationship value is idempotent and adds no causal edge; two distinct admitted values satisfying one required slot are ambiguous. | Test (TC-141) |
| FR-262-AC-5 | Source and target each retain a complete FR-287 key in authored order; empty, wrong-kind, foreign, reversed or statically incompatible endpoints refuse. | Test (TC-141) |
| FR-262-AC-6 | Message, receive occurrence, delivery and receipt observation remain non-interchangeable even when transport or display data match. | Test (TC-141) |
| FR-262-AC-7 | Withdrawn: it existed only for the declaration digest and its domain, which the relationship no longer carries. | Withdrawn |
| FR-262-AC-8 | For one required slot, a matching candidate together with a candidate that asserts an incompatible endpoint returns `conflicting`; two distinct matching candidates with no incompatible candidate return `ambiguous`. Reversed direction is `InvalidReference` during reference construction, before resolver admission. | Test (TC-141, `filament-observation/tests/admission.rs::tc005_relationship_conflict_precedes_match_count`, `filament-observation/tests/admission.rs::tc005_relationship_reversal_is_invalid_reference`) |
| FR-262-AC-9 | Exact replay of one producer identity and complete relationship value is one candidate; reusing one producer identity with different relationship facts refuses as `RelationshipIdentityContradiction` when resolver identity validation is reached, before slot evaluation; equal facts with distinct producer identities are distinct candidates. An incomplete result reached earlier returns unchanged without identity-rebind checking. | Test (TC-141, `filament-observation/tests/admission.rs::tc005_same_relationship_identity_cannot_rebind`, `filament-observation/tests/admission.rs::tc005_distinct_identities_are_distinct_candidates`, `filament-observation/tests/admission.rs::tc005_pre_slot_incomplete_precedes_identity_rebind`) |
| FR-262-AC-10 | An unlisted or complete-empty extent with no candidate yields `missing`; an explicitly partial extent yields the existing incomplete admission result with an `IncompleteRelationshipExtent{required}` reason, with or without a matching candidate, unless an incompatible candidate establishes `conflicting`. The reason does not add a correlation outcome. | Test (TC-141, `filament-observation/tests/admission.rs::tc005_relationship_extent_completeness_controls_resolution`) |

## Dependencies

- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  establishes the explicit typed relationship and its refusal cases.
- [FR-213](../protocol/FR-213-workflow-instance.md) supplies the related
  instance subject and records that its key is unstated.
- [FR-200](../foundation/FR-200-filament-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](../protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  supplies the ordered source and target keys.

## Status

Proposed closure for specification issue #31. Relationship declarations,
endpoint types and direction are spec-declared and checked by S3
([AD-006](../../assurance/AD-006-model-graph-state-contracts.md)). Observation
currently uses unqualified endpoint identities, ignores relationship identity
when matching and treats exact replay as ambiguity; that implementation is
partial and incompatible. Runtime adoption remains planned under TC-141 and
the existing Observation/Protocol owner tickets.
