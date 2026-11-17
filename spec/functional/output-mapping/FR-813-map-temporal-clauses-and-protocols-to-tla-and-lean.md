---
id: FR-813
title: "Map temporal clauses, fairness and protocols to TLA+ and Lean 4"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-812
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-425
    type: references
  - target: ix://agent-ix/filament-specification/FR-377
    type: references
---

# FR-813: Map temporal clauses, fairness and protocols to TLA+ and Lean 4

## Description

When the TLA+ or Lean 4 mapper exports a temporal clause, a fairness
constraint or a protocol, it SHALL use the encodings below, which state the
clause's FSL meaning over the exported behaviours exactly, so that the
untimed forms are `preserved`.

## Inputs

The obligation's temporal, fairness and protocol nodes and the state and
actions of FR-812.

## Outputs

The temporal, fairness, protocol and theorem regions.

## Behavior

### Temporal clauses in TLA+

- A clause built only from `always` and `eventually` over state formulas
  SHALL be emitted directly with `[]` and `<>`.
- Every other clause (`until`, `release`, past operators `once`,
  `historically`, `since`, `triggered`, interval operators under a
  step-counting profile) SHALL be emitted as its clause monitor composed with
  the specification: the monitor's states become auxiliary variables `m_<clause>`
  updated in every `Next` disjunct, past operators become history variables,
  and the clause becomes the acceptance condition over the monitor
  (`[]~reject` for a safety monitor, `[]<>accept` for a Büchi monitor). The
  monitor is the one FSL derives from the clause, so the encoding is exact;
  auxiliary variables add no behaviour to the original variables.
- Step-sensitive meaning (intervals counted in steps) SHALL be carried by
  the monitor, whose bounded step counters change only in operation
  actions and stay unchanged by stuttering, so TLA+'s finite stuttering
  does not shift a step count; no unbounded counter is added, so a finite
  model stays finite.
- The obligation's statement SHALL be `THEOREM T_<clause> == Spec =>
  <formula>`, written without a proof.

### Temporal clauses in Lean 4

Every clause SHALL be emitted as a `def` over a `Behaviour` (FR-812) and a
position, by direct recursion on the clause: `always p` at `i` as `∀ j ≥ i,
p b j`; `p until q` as `∃ k ≥ i, q b k ∧ ∀ j, i ≤ j → j < k → p b j`; past
operators over positions `≤ i`; intervals by bounds on `j - i` in steps.
The obligation's statement SHALL be a `def Ob_<clause> : Prop := ∀ b :
Behaviour, holds_<clause> b 0`. The file states obligations as propositions
and contains no `theorem`, no proof and no `sorry`.

### Fairness and terminal states

- Weak and strong fairness of an operation SHALL be `WF_vars(op_<op>)` and
  `SF_vars(op_<op>)` (with existential argument choice) in TLA+, and the
  corresponding `∀ i, (∃ j ≥ i, …)` premises in Lean.
- A terminal state stutters forever in both targets (FR-812 `Behaviour`),
  which is FSL's terminal stutter (FR-377).

### Clauses over supplied traces

A clause over a supplied, finite trace (observation and finite-window
clauses) SHALL be emitted as an operator (TLA+) or a `def` (Lean) over a
finite sequence of observation records, applied at the clause's activation
positions; it has no `Spec`.

### Protocols

A protocol SHALL be exported through its FR-425 protocol subject: each
role's control position and local state as variables, each channel as a
variable (a sequence for `fifo`, a bag for `unordered`, bounded by its
capacity in `TypeOK`/`WF`), and each send, receive, loss and timeout as an
action of `Next`. A symbolic capacity exports as a constant with its
declared constraint. Untimed channels are `preserved`; a delivery interval
in time units uses the dense-time encoding and is `conditional` (FR-651).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-813-AC-1 | `always holds(c.value <= 5)` exports to TLA+ as `Spec => [](...)` with no auxiliary variable, and to Lean as `Ob_<clause>` over `Behaviour`; both `preserved`. | Test (TC-1972) |
| FR-813-AC-2 | `holds(c.req) until holds(c.ack)` exports to TLA+ with a monitor variable and a `[]<>accept` condition, and to Lean as the direct `∃ k` definition; both `preserved`. | Test (TC-1972) |
| FR-813-AC-3 | `always[0, 3] holds(c.ok)` under a step-counting profile exports a monitor whose step counter is bounded by 3 in `TypeOK`, and an exported behaviour with two inserted stuttering steps keeps the same verdict. | Test (TC-1972) |
| FR-813-AC-4 | Weak fairness on `deliver` exports `WF_vars(op_deliver)`; a two-role protocol with one `fifo` channel of capacity 2 exports channel and role variables and send and receive actions, `preserved`. | Test (TC-1972) |
| FR-813-AC-5 | The Lean file of AC-1 and AC-2 contains no `theorem`, no proof term and no `sorry`. | Test (TC-1972) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-812](FR-812-map-state-actions-and-values-to-tla-and-lean.md);
  [FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md) owns
  protocol subjects; FR-377 the terminal stutter.
