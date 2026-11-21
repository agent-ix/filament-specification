---
id: TC-2037
title: "A synthesis declaration is one synthesis-request node"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-467
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-468
    type: verifies
---

# TC-2037: A synthesis declaration is one synthesis-request node

## Description

Verify the `claim`/`synthesis_request` node a `synthesis` declaration
checks to, its `claim_names` row, and the reader's refusals of edited
bodies. Scope: FR-322-AC-467 and FR-322-AC-468.

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
