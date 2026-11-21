---
id: TC-141
title: "Correlate related instances without heuristics"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-111
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-262
    type: verifies
---
## Description

Accepted FS04 portable OB01 correlation controls. No
executable standard coverage is claimed.

## Test Procedure

Create concurrent O1/O2 with a shared provider. Type their relationships by
relationship declarations in the checked package's domain package.
Link a refund to O1's effect, offer it for O2, remove/link-conflict the relation,
replay one exact relationship and offer a second distinct relationship for the
same required slot. Replay a candidate with both the same producer identity and
the same facts, and separately offer equal facts under a different producer
identity. For one required slot, also offer one exact match together
with one candidate asserting an incompatible endpoint; then offer
two distinct matching candidates with no incompatible candidate. Reuse one
producer relationship identity with different endpoint facts. Check a required
slot against an unlisted extent, a complete-empty extent, and `complete:
false` extents both without a candidate and containing an exact matching
candidate. Repeat an equal required
slot, and supply multiple missing slots in an order different from their
canonical key order. Also make membership incomplete and remove the selected
closure before slot evaluation, each with a selected required slot. Combine
missing closure with an identity rebind and with a reversed-endpoint candidate.
Reverse relationship endpoints and independently mutate each FR-287
endpoint member, relationship kind/declaration key, domain package identity. Offer a relationship declaration key that is not in the
checked package. Repeat a delivery receipt and substitute its message,
delivery or receive identity in each other position.

## Expected Results

O1 is selected only by its declared relation. Missing is incomplete;
conflicting and multiple distinct matches refuse; exact replay is idempotent.
A matching candidate together with an incompatible endpoint candidate is conflicting;
ambiguous requires two distinct matching candidates and no incompatible
candidate. An explicitly partial relationship extent contributes an
`IncompleteRelationshipExtent{required}` reason to the existing incomplete
admission result, even with a matching candidate unless a known conflict
refuses; it does not add a fifth correlation outcome. Reversed direction is
`InvalidReference` during reference construction, before resolver admission. An identity rebind submitted with
missing closure returns the earlier `MissingClosure` unchanged; the resolver
does not check the rebind. A reversed-endpoint construction failure submitted
with missing closure still refuses `InvalidReference` before the resolver.
Equal required slots are one set member; missing-slot reasons appear in the
canonical key order when slot evaluation completes, while a refusal is returned
alone and an incomplete result from earlier admission is returned unchanged before
slot evaluation and no relationship slot is evaluated. An unlisted or
complete-empty extent with no candidate is `missing`; an incomplete extent
adds `IncompleteRelationshipExtent{required}` within the existing incomplete
admission result, with or without a match unless a known conflict refuses.
Exact replay with the same identity and facts is one
candidate; identity rebind refuses, and identical facts with different
identities are distinct candidates.
A declaration outside the checked package, endpoint reversal/substitution, repeated
delivery and every declaration-selection mismatch refuse without
manufacturing an effect or reconstructed relationship. Receipt, delivery,
receive and message remain distinct.
