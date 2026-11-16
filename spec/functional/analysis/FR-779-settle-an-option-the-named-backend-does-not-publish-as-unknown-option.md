---
id: FR-779
title: "Settle an option the named backend does not publish as unknown-option"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-290
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-272
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-775
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-967
    type: traces_to
---

# FR-779: Settle an option the named backend does not publish as unknown-option

## Description

An FR-331 request item that names a backend may carry `options`, a map of the
backend's option names to string values. A producer that builds a request item
SHALL settle an item whose `options` hold a name the named backend's manifest
`options` do not publish as the disposition `invalid-request` with
`invalid_capability`/`unknown-option` (FR-272), payload `request_index`,
`backend` and `option`, the least unpublished name in byte order.

A manifest's `options` are the backend's published capability, which the
registry snapshot carries. An option outside them asks for a capability the
backend does not offer, the same class of defect as `unknown-backend`, so the
cause is an `invalid_capability` one and the disposition is `invalid-request`.
No input value is wrong, so `invalid_runtime_input` does not apply. The rule
holds for every operation that routes items (`analyze`, `prove`, `monitor`,
`execute` with requirement settlement (FR-775), `replay`, `generate`).

## Inputs

The request item's named backend and `options`; the registry snapshot.

## Outputs

The item's disposition, or no change.

## Behavior

- An item whose named backend the registry does not hold SHALL settle
  `unknown-backend` and its options SHALL NOT be checked.
- The check SHALL run before candidates are computed; an item it settles is
  not negotiated, sent to a provider or run.
- The refusal settles only its own item; the request's other items and their
  request indices are unchanged.
- An item whose every option the backend publishes carries them to the
  backend unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-779-AC-1 | Over a registry whose backend `b` publishes the option `depth`, an item naming `b` with `options` `{"zeta": "1", "alpha": "2"}` settles `invalid-request` `invalid_capability`/`unknown-option` with `request_index` its index, `backend` `b` and `option` `alpha`, and is sent to no backend; an item naming `b` with `{"depth": "3"}` reaches `b` with that option; an item naming an unregistered backend with options settles `unknown-backend`; the native-diagnostics row and FR-272's row of `invalid_capability` each list `unknown-option` with the payload keys `request_index`, `backend` and `option`. | Test (TC-1891) |

## Dependencies

- FR-290: negotiation. FR-331: the request item and manifest. FR-272: the cause.
