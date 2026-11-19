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
- The check SHALL run after FR-290's kind admission (`absent-kind`,
  `unknown-kind`) and its `inapplicable-kind` rule, and before candidates are
  computed, so each item has exactly one outcome: an item those rules settle
  is not checked for options, and an item the check settles is not
  negotiated, sent to a provider or run.
- The refusal settles only its own item; the request's other items and their
  request indices are unchanged.
- An item whose every option the backend publishes carries them to the
  backend unchanged.

### Option values

- A backend publishes its option names and defaults, not their value
  domains: a domain such as "a positive rational number of seconds" is not
  a closed set a manifest can list. A backend that holds a domain for a
  published option SHALL check the item's option values before any solving
  step, and a value outside its domain SHALL settle the item
  `invalid-request` with `invalid_capability`/`invalid-option-value`
  (FR-272), payload `request_index`, `backend`, `option` (the least refused
  option name in byte order) and `value` (that option's value as the item
  wrote it). The cause is an `invalid_capability` one for the same reason
  as `unknown-option`: the item asks the backend for a setting it does not
  offer.
- The value check SHALL run after the name check above, so an item with
  both an unpublished name and a bad value settles `unknown-option`.
- An item the value check settles SHALL NOT run a solving step, and its
  refusal settles only that item.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-779-AC-1 | Over a registry whose backend `b` publishes the option `depth`, an item naming `b` with `options` `{"zeta": "1", "alpha": "2"}` settles `invalid-request` `invalid_capability`/`unknown-option` with `request_index` its index, `backend` `b` and `option` `alpha`, and is sent to no backend; an item naming `b` with `{"depth": "3"}` reaches `b` with that option; an item naming an unregistered backend with options settles `unknown-backend`; the native-diagnostics row and FR-272's row of `invalid_capability` each list `unknown-option` with the payload keys `request_index`, `backend` and `option`. | Test (TC-1891) |
| FR-779-AC-2 | Over a registry whose backend `h` publishes the option `step` and holds its domain "a positive exact rational", an item naming `h` with `options` `{"step": "0"}` settles `invalid-request` `invalid_capability`/`invalid-option-value` with `request_index` its index, `backend` `h`, `option` `step` and `value` `0`, and the backend runs no solving step; `{"step": "1/4"}` reaches the backend; `{"colour": "blue", "step": "0"}` settles `unknown-option` with `option` `colour`; the native-diagnostics row and FR-272's row of `invalid_capability` each list `invalid-option-value` with the payload keys `request_index`, `backend`, `option` and `value`. | Test (TC-1891) |

## Dependencies

- FR-290: negotiation. FR-331: the request item and manifest. FR-272: the cause.
