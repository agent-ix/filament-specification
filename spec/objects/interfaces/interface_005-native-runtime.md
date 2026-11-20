---
id: interface_005
title: "I05 native runtime input and result"
type: interface
relationships:
  - target: ix://agent-ix/filament-specification/FR-323
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-100
    type: references
  - target: ix://agent-ix/filament-specification/FR-153
    type: references
---

# [interface_005] I05 native runtime input and result

## Features

| Feature | Kind |
| --- | --- |
| execute | operation |
| replay | operation |

## Contract

```yaml
name: NativeRuntime
associated_types: [RuntimeRequest, StateEnvironment, RuntimeResult, ResourceLimit]
operations:
  - { name: execute, inputs: [CheckedPackage, typed request, explicit inputs, limits], output: ordered per-clause results, semantics: reference native value state and graph evaluation }
  - { name: replay, inputs: [CheckedPackage, canonical counterexample, limits], output: verdict and parity metadata, semantics: uses the same execute semantics }
invariants:
  - invalid unsupported incomplete undefined failed and false remain distinct
  - exhaustion never truncates meaning into a completed value
```

## Package selection

A request that selects an existing checked package by path carries FR-323's
`package` reference as one closed selection (FSL FR-100, FR-1039):

```yaml
PackageSelection:
  oneOf:
    - { type: object, required: [file], properties: { file: string, equality: string, source_package: SourcePackageRef }, additionalProperties: false }
    - { type: object, required: [directory], properties: { directory: string, source_package: SourcePackageRef }, additionalProperties: false }
SourcePackageRef: { type: object, required: [identity, version], properties: { identity: { type: string, minLength: 1 }, version: { type: string, minLength: 1 } }, additionalProperties: false }
```

`equality` is the path of the package's equality witness file and is
admitted on the file form only; FR-323 states its reading and refusals.

## FSL source Value-call binding

For the FSL source-compiled Value function boundary, I05's typed values SHALL
be the shared `filament_exact::Value` carriers. Its explicit model/object
context SHALL be the borrowed
`fsl_semantics::model::object_environment::ObjectEnvironment`, with an
actually admitted shared object closure and registered FR-153 population
bindings. FSL [FR-100](ix://agent-ix/filament-spec-language/FR-100) owns the
single public `fsl_replay::spine::run` facade and its replacement signature;
the driver consumes it. I05 adds no second executor, evaluator export,
private environment copy or locally minted model/value semantics.

A Population value SHALL carry only the owning admission's PopulationId;
its binding, selected domain package and declared maximum remain in the
model context. A Reference SHALL retain the admitted universe,
most-specific effective type and exact object identity. The executor SHALL
check those correspondences before evaluation, including for unused
parameters. The object world can contain a Reference that is absent from a
particular closed query population; such an admitted Reference reaches the
lookup's declared absence mode. A reference absent from the complete
object world instead retains the typed dangling-reference admission
failure. An unregistered population, wrong maximum, wrong kind or foreign
model/universe retains its owning admission failure and actual fields.

The executor SHALL use FSL FR-100's settled admission order: all name checks
first, then declared parameter order, checking value shape, actual selected
model/universe correspondence, selected-model conformance and complete-world
resolution. Its Reference parameter domain is the real admitted selected
model's shared FSV `TypeEnvironment::admits`/`conforms` domain: a
most-specific `S` is admitted to `Reference<T>` when both types are admitted
there and `S` equals or has a declared supertype path to `T`. The original
Reference triple SHALL remain unchanged, without retagging or constructing
an upcast. A same-model unrelated type refuses `WrongValueKind`; a foreign
model or universe retains its actual owning cause before type conformance.
If universe and type both fail, then the executor SHALL preserve
`foreign_reference` / `foreign-universe` with the actual required/supplied
identities, not substitute wrong-kind or dangling-reference refusal.
The same rules apply to unused parameters. Population registration and
declared-maximum guards remain FR-100/FR-089's, not kernel Population
admission.

Kernel `filament_exact::ValueType::admits` remains exact-type admission for
References; it does not itself admit proper subtypes. The model-aware FSV
predicate is the owning conformance rule here, not a change to that kernel
API or permission for consumers to select different admission semantics.
TC-792's genuine subtype/unrelated/foreign/combined controls discriminate
these rules through the single public call boundary.

When a Value function completes with a Reference, the executor SHALL retain
the actual returned shared reference in the typed result, including its
universe, most-specific effective type and exact object identity. The
returned reference need not equal an input; input equality is specific to
TC-792's present lookup control. When serialized as a function value it
SHALL use the
existing canonical typed Reference shape
`{kind: "reference", universe, type, object_identity}`: two actual
64-character lowercase hexadecimal universe/type identities and the exact
nonempty UTF-8 object identity. This is a terminal identity, not object
state or a Population binding. The original Boolean/Integer results and
typed refusal, undefined, incomplete, cancellation and fault partitions
remain distinct. A lookup refusal SHALL retain its StateModel cause in
the enclosing Value-family result, not become an admission failure or a
Boolean false claim.

This binding fixes the in-process semantic carriers; it does not add a
Population JSON value to the native-run/1 file request or amend FR-323's
closed envelope. The applicable strict reader must admit actual selected
model/object inputs through its existing owner contracts. No adapter may
infer a missing payload, register a caller-asserted population identity,
or treat shape decoding as semantic admission. File-based typed-input
support requires that consumer's separate qualification.

Population admission, supplied helper admission/conversion and evaluation
SHALL retain their own actual caller limits and original Cancel, including
Requested versus Deadline. The QSL-681 proposed FR-323 supplied-phase
amendment remains its owner's work: A/H defaults, native cause mapping,
public helper and consumer qualification are not selected or established
here. Pre-call failures have zero evaluation consumption and no fabricated
FunctionCall charge. Existing accounting, identity, schema and strict-reader
owners retain their contracts.

This amendment is proposed under QSL-665 for independent review. The
absent/present source lookup, unused-parameter admission controls, scalar
regressions, cancellation/accounting and driver adoption are the existing
[TC-792](ix://agent-ix/filament-spec-language/TC-792) controls, all UNRUN here.
No source-only contract change claims runtime or consumer qualification.
