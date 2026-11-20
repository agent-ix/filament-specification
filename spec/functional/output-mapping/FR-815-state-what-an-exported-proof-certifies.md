---
id: FR-815
title: "State what a TLA+ or Lean result over an export certifies"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-354
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
---

# FR-815: State what a TLA+ or Lean result over an export certifies

## Description

When a target tool's result over an exported package is brought back into
FSL, the coordinator SHALL give it exactly the meaning the export's
disposition supports: for a `preserved` obligation, a target proof or a
complete target model check certifies the FSL obligation over the exported
model as `trusted` evidence; for a `conditional` one, only under its
retained conditions; for an `unrepresented` or `refused` one, nothing; and
a target counterexample counts only after native replay.

## Inputs

An FR-299 package; a target result (a TLC or Apalache run, a TLAPS proof, a
Lean proof of an `Ob_` proposition) that a caller submits as evidence against the package; an observer run (FR-651) is not this intake and settles nothing.

## Outputs

An FR-331 result entry for the obligation, or none.

## Behavior

### The preservation claim

For a `preserved` obligation the export SHALL satisfy: an FSL behaviour of
the model satisfies the obligation exactly when the corresponding target
behaviour (the same states, up to finite stuttering in TLA+) satisfies the
exported formula. The claim covers the model as exported: the package's
model, initial states, operations, fairness and the obligation. It does not
cover whether the model describes a running system or an implementation
(implementation guarantees are FR-488's), and it does not cover the
mapper's own correctness, which is part of the trust base.

### What a target result certifies

| Target result | Disposition | Meaning in FSL |
| --- | --- | --- |
| TLAPS proof of `Th_<clause>`, or a Lean term of type `Ob_<clause>` checked by the Lean kernel | `preserved` | `proved`, basis `decisive-witness`, certification `trusted`, extent `unbounded`; the trust base names the mapper, its version and the target tool and version |
| complete TLC or Apalache run over the whole finite state space, no state constraint, invariant or property holding | `preserved` | `proved`, certification `trusted`, extent `unbounded` for a finite model |
| run bounded by a depth, a state constraint or a `CONSTANT` restriction | `preserved` | `proved` with extent `bounded`, the bound recorded; never `unbounded` |
| any of the above | `conditional` | the same, with every retained condition listed as an assumption of the entry |
| any result | `unrepresented` or `refused` | no entry |
| a target counterexample trace | any | read back through the name map (FR-814) and replayed natively; it settles `refuted` only when native replay reproduces the violation, otherwise it is recorded as a non-reproducing observation and settles nothing |

- A target result SHALL never be `certified`: only the core's own
  certificate checkers certify (FR-354).
- A target result over a package SHALL apply only to that package's key: a
  changed model, package version, profile or mapper version makes it
  inapplicable.
- An observer's parse or elaboration acceptance is structural evidence and
  settles nothing (FR-651).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-815-AC-1 | A Lean term checked against `Ob_Low` of a `preserved` record yields `proved`, `trusted`, extent `unbounded`, with a trust base naming the mapper, its version and the Lean release; never `certified`. | Test (TC-1974) |
| FR-815-AC-2 | A TLC run with a state constraint yields extent `bounded` naming the constraint; a complete run over the finite counter yields extent `unbounded`. | Test (TC-1974) |
| FR-815-AC-3 | A proof over a `conditional` dense-time TLA+ record lists its `real-valued-time` condition, which states the non-Zeno premise, as an assumption; a result over an `unrepresented` record yields no entry. | Test (TC-1974) |
| FR-815-AC-4 | A TLC counterexample reads back to FSL states and settles `refuted` after native replay; a trace edited so it no longer replays settles nothing. | Test (TC-1974) |
| FR-815-AC-5 | A Lean proof recorded against a package whose model changed after export is inapplicable to the new package. | Test (TC-1974) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-354](../ir-proof/FR-354-record-proof-trust-basis.md) and
  [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md).
