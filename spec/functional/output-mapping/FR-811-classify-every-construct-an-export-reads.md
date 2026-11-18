---
id: FR-811
title: "Classify every construct a TLA+ or Lean export reads, and refuse below the fidelity floor"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-121
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-298
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-269
    type: depends_on
---

# FR-811: Classify every construct a TLA+ or Lean export reads, and refuse below the fidelity floor

## Description

When the `filament.output.tla2/v1` or `filament.output.lean4/v1` mapper
exports an obligation, it SHALL account for every checked-package node in
the obligation's dependency closure, either by an output region that
represents it or by a typed cause on the obligation's record, so that no
construct is dropped silently; and when the request's fidelity floor is not
met, the coordinator SHALL refuse the whole request with every failing
record's located cause and write no target bytes.

## Inputs

An admitted FR-120 request for one of the two profiles, with its
`minimum_fidelity` member; the checked package.

## Outputs

One FR-298 record per obligation, each with its causes; one FR-299 package,
or a whole-request refusal.

## Behavior

### Dependency closure

The *dependency closure* of an obligation SHALL be the set of package nodes
its meaning reads: the clause, every expression, type, function, model,
object type, population, operation, frame, clock, fairness and protocol node
reachable from it through the package's references. Each node in it SHALL be
either represented by an output region (FR-812, FR-813) or named by a cause.

### Coverage

| Construct family | `filament.output.tla2/v1` | `filament.output.lean4/v1` |
| --- | --- | --- |
| state models, populations, operations, frames, state clauses, invariants | `preserved` (FR-812) | `preserved` (FR-812) |
| scalar, record, tuple, union, enum, option, collection and map values; exact integer, rational, decimal and quantity arithmetic; division laws; binary floating point; text | `preserved` (FR-812) | `preserved` (FR-812) |
| Unicode normalization and case-mapping operations on text | `unrepresented` | `unrepresented` |
| future and past temporal operators, intervals under a step-counting profile, fairness, terminal stutter | `preserved` (FR-813), or `conditional` for a clause that needs a nondeterministic monitor | `preserved` (FR-813) |
| clauses over supplied traces (observation, finite window) | `preserved` as a predicate over a finite trace (FR-813) | `preserved` (FR-813) |
| protocols with untimed channels | `preserved` (FR-813) | `preserved` (FR-813) |
| dense time, timed channels | `conditional` (FR-651) | `conditional` (FR-651) |
| hyperproperties, hybrid, probabilistic | as FR-651 "Fidelity" states | as FR-651 "Fidelity" states |

A construct the table marks `unrepresented` SHALL make its obligation
`unrepresented`. Its cause is `target-unrepresentable{target, construct,
node, locus}`: `target` the family, `construct` the construct's
`node_tag`/`semantic_form`, `node` its node id, and `locus` its FR-203
source region. An obligation whose closure holds several such nodes lists
one cause per node, in node order.

### Decision: one refusal form, and only for what the target cannot state

`target-unrepresentable` SHALL be the only cause by which a construct makes
its obligation `unrepresented`, and a mapper SHALL write it only for a
construct that the table above or FR-651 "Fidelity" marks `unrepresented`
for the selected target. Every other construct of the closure SHALL be
lowered by FR-812 or FR-813 to the disposition the table gives it. No
disposition, cause or exit stands for a construct that the target can state
but that the mapper does not lower: a mapper that meets such a construct
stops the export as a mapper failure (FR-817, exit `30`), with no package
and no target bytes.

The fidelity floor exists to measure what the target can say about the
source. A "not lowered" record would let an incomplete mapper assemble a
package under `minimum_fidelity` `any` that reads as a faithful export, and
two conforming mappers would then give different records for one request,
which FR-125 forbids. A missing lowering is a defect of the mapper, so it
fails loudly, while a construct the target truly cannot state is a property
of the model and is reported as a record.

### The fidelity floor

- The request member `minimum_fidelity` SHALL be `preserved`, `conditional`
  or `any`, default `conditional`.
- Under `preserved`, any record that is not `preserved` fails the floor;
  under `conditional`, any `unrepresented` or `refused` record fails it;
  under `any`, nothing fails it.
- When a record fails the floor, the coordinator SHALL refuse the request
  `unsupported_projection`/`below-fidelity-floor`, listing every failing
  record's obligation, disposition and causes in record order, and SHALL
  assemble no package and write no target bytes (FR-299-AC-4).
- Under `any`, the package is assembled with every record, and an
  `unrepresented` or `refused` record claims no region (FR-298).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-811-AC-1 | An invariant calling a function that normalizes text exports `unrepresented` with `target-unrepresentable{target: tla, construct: expression/call, node, locus}` naming the normalization call's node and source region. | Test (TC-1970) |
| FR-811-AC-2 | Under the default floor `conditional`, a request holding that obligation and a `preserved` one refuses `unsupported_projection`/`below-fidelity-floor`, names the failing obligation and its cause, and no target file or package exists. | Test (TC-1970) |
| FR-811-AC-3 | Under `minimum_fidelity` `any` the same request assembles a package in which the failing record claims no region and the other record is `preserved`. | Test (TC-1970) |
| FR-811-AC-4 | Under `minimum_fidelity` `preserved` a dense-time clause (`conditional`) fails the floor; under `conditional` it passes. | Test (TC-1970) |
| FR-811-AC-5 | For every obligation of the export conformance corpus (FR-818), every node of its dependency closure is either covered by an output region of its record or named by one of its causes. | Test (TC-1970) |
| FR-811-AC-6 | The counter's invariant `value <= 5` exports `preserved` under each profile with no cause; `SlowEnough` over the hybrid `Braking` exports `unrepresented` with one `target-unrepresentable` cause per flow-equation node of `Braking`, in node order, each with `target` the family, `construct` that node's `node_tag`/`semantic_form`, `node` its node id and `locus` its source region, and no record of either export carries any other cause for a construct of its closure. | Test (TC-2029) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md) owns the profiles and the
  fidelity rule; [FR-121](FR-121-account-for-every-output-obligation.md),
  [FR-298](../../objects/output-mapping/FR-298-per-obligation-output-mapping-record.md)
  and [FR-269](../../objects/observation/FR-269-consumer-mapping-loss.md) own the
  records and dispositions.
