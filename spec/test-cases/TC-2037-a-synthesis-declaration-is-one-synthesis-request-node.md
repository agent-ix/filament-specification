---
id: TC-2037
title: "A synthesis declaration is one synthesis-request node"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-467
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-468
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-517
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-518
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-519
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-806-AC-6
    type: verifies
---

# TC-2037: A synthesis declaration is one synthesis-request node

## Description

Verify the `claim`/`synthesis_request` node a `synthesis` declaration
checks to, its `claim_names` row, and the reader's refusals of edited
bodies. Scope: FR-322-AC-467, FR-322-AC-468 and FR-322-AC-517 to FR-322-AC-519 and FR-806-AC-6.

## Test Procedure

1. Read the package of a unit declaring `type Digit = Int[0, 9];` and
   `synthesis Pick using p grammar Digit domain Digit satisfies { Pick * Pick = 49 };`.
2. Read the same unit with `minimize Pick`, and with a second declaration
   `Pick2` of the same grammar, domain and block.
3. Feed the reader each edit FR-322-AC-468 lists.
4. Select the admitted `example/config-version` domain package and read the
   package checked from this complete plan-synthesis source unit:

   ```fsl
   language "ix:native" edition "1-draft";
   profile p = "filament.model.complete/v1";
   model Config = "example/config-version";
   type Lengths = Int[0, 3];
   synthesis Reach using p grammar Config::ConfigVersion domain Lengths
     satisfies { true };
   ```

   Record the request's exact `grammar`, `domain` and `candidate`
   references, the candidate's `name`, `level` and `semantic_type`, and
   the referenced plan record's node, body, declaration and owner.
5. From step 4's otherwise admitted package, construct a negative fixture
   whose candidate has the same name `Reach` and level `0` but whose
   `semantic_type` references the actual `ConfigVersion` grammar node.
   Rebuild the parameter's structural key under the existing parameter
   identity rule (its type participates in that key), and replace every
   reference to its old key, including the request's candidate binding,
   dependencies and source-map keys. Maintain dependency sorting, node and
   operand-row canonical order, all required occurrences and source-map
   coverage, and unchanged grammar/domain joins, owners and `claim_names`.
   The true block and absent objective reference no candidate field, so
   this edit adds no earlier field-projection type defect. Feed this
   well-formed package to the reader. An earlier schema, structural-key,
   source-map, reference or application-type failure does not satisfy this
   test's refusal oracle.

6. Check the shared-arguments signatures in this table. `Server` has a
   finite live population; all other domains are explicitly bounded. No
   unbounded-domain defect may mask a signature conflict.

   | Operations and their declared parameter signatures | Expected boundary |
   | --- | --- |
   | `read(s: Int[0,1])`, `write(s: Int[0,1])`, both required | one required `s` field of that exact type |
   | same `read`, and `noop()` with no `s` | optional shared `s` of that exact type |
   | `drain(s: Reference<Server>)`, `resize(s: Int[0,1])`, both required | unsupported, naming `s`, `drain` and `resize` |
   | `read` and `maybeRead` both declaring `s: Int[0,1]`, required on `read` and optional on `maybeRead` | S3 support-set refusal, naming `s`, `read` and `maybeRead` |
   | `maybeRead` with optional `s: Int[0,1]`, `optionRead` with required `s: Option<Int[0,1]>` | admitted common `Option<Int[0,1]>` field type; native omission flags remain different |
   | `read(s: Int[0,1])`, `write(s: Int[0,1])`, `resize(s: Int[0,2])`, `other(s: Boolean)`, all required, in that declaration order | first conflict names `s`, `read` and `resize` |

   Presence is the existing model declaration property under FR-154.
   Compare FR-154/FR-151 normalized parameter types for record typing;
   native argument-omission flags remain distinct. For each conflict,
   inspect the S3 diagnostic and confirm no checked package or plan
   outcome was emitted and no plan request read. No per-operation union
   or coercion is inferred.
7. Read the generated plan types for FR-819-AC-1's hybrid tank, whose
   modes are `Fill`, then `Drain`, and whose declared state variable is
   `h`. Compare exact type references through the plan and step records,
   the selected hybrid model's mode and variable references, and the
   builtin Rational node used by `dwell_grid`. Do not enumerate continuous
   trajectories or invent native invocation ordinals.
8. Read generated plan types for an operation with a `Server` receiver,
   one with a `Worker` receiver, and one receiverless operation, with
   finite live populations for both types. Resolve the generated receiver
   union's payload references, not its generated case labels. Construct
   typed source step values and wire steps for both receiver-bearing
   operations, then supply the other operation's receiver case and a
   receiver on the receiverless operation to FR-650's plan-substitution
   and internal certificate-tamper library seams, respectively.

## Expected Results

- Step 1: one node, path `["Pick"]`, one `declaration` occurrence, body
  bindings `grammar`, `domain`, `candidate`, `objective` (empty),
  `satisfies`, `symmetry` `identity`; `dependencies` `Digit`, the candidate
  and the block root; one `claim_names` row named `Pick`.
- Step 2: `objective` is `minimize`, the candidate node and the text `Pick`;
  `Pick2` is a second node.
- Step 3: each refusal at the code and locus FR-322-AC-468 states.
- Step 4: admission; `grammar` is the actual `ConfigVersion`
  `model`/`object_type` node and `domain` the `Lengths`
  `bounded_domain`/`integer_range` node with bounds `[0, 3]`. The candidate
  is a `value`/`parameter` named `Reach` at level `0`; its `semantic_type`
  resolves to the exact generated `composite_type`/`record` node emitted
  for the FR-800 plan value, with the field-name/type constraints and
  generated-record identity/ownership/source mapping in FR-322
  "Plan-record type". Read-back returns those exact candidate and plan-type
  references, grammar and domain references, no objective, and the true
  Boolean block. It does not return the grammar node as the candidate type
  or substitute another record by name or matching field shape.
- Step 5: preceding reader stages admit the fixture; synthesis-request
  reader step 3 refuses `invalid_package`/`invalid-value` at the
  `candidate` binding. No source compilation or runtime search is used as
  evidence for this edited-package refusal.
- Step 6: admitted rows have the tabled normalized field type/presence.
  Conflicts give S3 `unsupported_construct`/`not-yet-implemented`, exit
  `21`, at the synthesis's `grammar` name, retaining the existing
  feature/owning-declaration/profile cause context and a diagnostic message
  naming the tabled parameter and operation pair. The model itself remains
  admitted but no checked package or plan outcome is emitted. Required
  `Option<X>` still requires a native argument even though optional `X`
  has the same normalized shared-field type.
  The exact unbuilt feature strings are:
  `parameter s has conflicting types or declared presence in operations drain and resize`,
  `parameter s has conflicting types or declared presence in operations read and maybeRead`,
  and, for the four-operation row,
  `parameter s has conflicting types or declared presence in operations read and resize`.
  Compare those byte for byte in FR-272's retained feature context and as
  the diagnostic message suffix. Changing this feature string while
  retaining the same code/cause tag is not an equivalent cause.
- Step 7: the generated mode enum has exactly `Fill`, `Drain` in that
  order. Step `operation` and state `mode` reference it exactly; step
  `index` is `Integer`, `arguments` is the empty record, `before`/`after`
  reference the same state record as plan `initial`/`final`, that record's
  `h` has the hybrid declaration's variable type, and `delay`/`elapsed`
  reference the exact builtin `scalar_type`/`rational` node. Receiver and native
  invocation ordinal are absent. The schedule retains FR-819's enclosure
  validation semantics.
- Step 8: one case carries each of the two distinct `Reference<T>` types
  in the existing union form, with no new tag or key. The receiver field
  is optional because one operation is receiverless. Both legitimate
  receiver types are supported, and operation/case mismatches or an
  unexpected receiver produces a method failure through plan substitution
  (exit `30`, first failing step/member, no outcome document), or a
  `closure` rejection at the internal certificate entry through tamper.
