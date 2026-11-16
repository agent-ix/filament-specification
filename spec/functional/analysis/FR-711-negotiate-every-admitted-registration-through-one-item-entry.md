---
id: FR-711
title: "Negotiate an item on every admitted registration through one item entry"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/AD-016
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-281
    type: traces_to
---
# FR-711: Negotiate an item on every admitted registration through one item entry

## Description

`filament-contract-codegen`'s `negotiate_*` is the single negotiation point
(FR-290, AD-016). It SHALL settle a request item on any admitted
registration, whatever kind of provider the registration describes: a linked
analysis engine, a process provider or a Kani backend. It SHALL expose one
public item entry that takes the request's manifest and one request item and
returns the item's disposition, and that entry SHALL require no member of the
item that its single candidate's arm does not read.

### Decision: one generic arm per registration, backend-specific checks on top

FR-290's candidate table settles an item `unknown-backend` on "a registered
backend with no negotiation arm". If arms existed only for the provider kinds
code generation emits harnesses for, every linked engine a caller registers,
such as an explicit-state model checker, would be refused as unknown although
its registration was admitted and it is the item's one candidate. Negotiation
compares what an item asks for with what a registration advertises (kind,
mode, domains, evidence kind, dynamics, fairness kinds, state-graph forms,
reductions, obligation forms), and every admitted registration advertises
those through the same FR-331 manifest. So every admitted registration has
the generic arm, which applies the rules that read the manifest alone, and a
backend whose IR forms need their own check (a Kani harness form, an SMT
obligation form) adds that check after the generic arm.

## Inputs

The request's manifest (its admitted registrations, FR-331) and one FR-331
request item with its `candidates` and extent.

## Outputs

One disposition: `supported` naming the candidate, `requires-bound`,
`unsupported` with its warning, or `invalid-request` with its cause.

## Behavior

- Every admitted registration SHALL have a negotiation arm. FR-290's
  "registered backend with no negotiation arm" row therefore never settles an
  item whose candidate is admitted; `unknown-backend` remains the settlement
  of the unknown-backend mark.
- The generic arm SHALL run, in order, FR-290's domain-kind check, its
  advertised-mode table, the evidence-kind rules, the hybrid-dynamics rule,
  the fairness-kind rule (FR-368), the state-graph form rule (FR-390), the
  reduction rule (FR-382) and the obligation-form rule (FR-349), each where
  the item carries the corresponding requirement. When every rule passes, its
  own disposition is `supported` naming the candidate.
- A backend-specific arm SHALL run only after the generic arm passes, and may
  settle only `supported`, `requires-bound` (FR-771), `unsupported` with a
  warning, or `invalid-request` naming the form it cannot take. It changes
  only the generic arm's `supported`, never a `requires-bound`, `unsupported`
  or `invalid-request` the generic arm settled, and it never turns any
  disposition into `supported`. It may turn the generic `supported` into
  `requires-bound` only as FR-771 states: when the backend's own form check
  finds a value domain in the item's IR form that the backend can decide only
  under a finite bound that the item does not carry.
- The item entry SHALL take the item without any backend-specific form when
  the single candidate's arm does not read one. An item whose single candidate
  is a backend whose arm needs such a form, and which lacks it, settles
  `invalid-request` with `invalid_capability`/`inconsistent-candidates`
  naming the candidate.
- The disposition of an item SHALL be a function of the manifest and the item
  alone: the same item over the same manifest settles the same whether the
  candidate is linked, a process or a Kani backend, except for what a
  backend-specific arm adds.
- A caller that settles items (the driver's `analyze` and `prove` verbs) SHALL
  settle each item through this entry; no caller settles an item by its own
  candidate rules.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-711-AC-1 | Over a manifest holding a linked engine `en-1` advertising (`temporal-satisfaction`, `unbounded`) and (`temporal-satisfaction`, `bounded`), the item entry settles a `bounded` and an `unbounded` item with candidate `en-1` `supported` naming `en-1`, and never `unknown-backend`. | Test (TC-1741) |
| FR-711-AC-2 | Over a linked engine advertising (`temporal-satisfaction`, `bounded`) only, an `unbounded` item settles `requires-bound` when a finite bound is available and `unsupported` `unbounded-extent` when none is, each row of FR-290's advertised-mode table settling its listed disposition for a linked candidate as for a process candidate with the same manifest. | Test (TC-1741) |
| FR-711-AC-3 | An item that carries no backend-specific form settles on a linked and on a process candidate; the same item whose single candidate is a Kani backend whose arm reads a harness form settles `invalid-request` `inconsistent-candidates` naming it. | Test (TC-1741) |
| FR-711-AC-4 | A state-graph item whose form a linked candidate's `state_graph_forms` lists settles `supported`; with the form absent from the list the candidate set is empty and the item settles `unsupported` `unsupported-requested-capability`. | Test (TC-1741) |
| FR-711-AC-5 | The driver's `analyze` settles every item through the item entry, and its dispositions equal the entry's for the same manifest and items. | Driver test (TC-1741): the driver calls the entry, so the driver code ticket adds the test in the driver repository. |

## Dependencies

- FR-290 owns the candidate set, the candidate table and the advertised-mode
  table; FR-331 owns the manifest and the request item.
- AD-016 places the negotiation point in `filament-contract-codegen`.
