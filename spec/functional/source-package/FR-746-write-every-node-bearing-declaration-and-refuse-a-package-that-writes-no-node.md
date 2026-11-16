---
id: FR-746
title: "Write every node-bearing declaration and refuse a package that writes no node"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-142
    type: references
  - target: ix://agent-ix/filament-specification/FR-151
    type: references
  - target: ix://agent-ix/filament-specification/FR-500
    type: references
---

# FR-746: Write every node-bearing declaration and refuse a package that writes no node

## Description

When a package's source units are checked, the checker SHALL give each
declaration that FR-322 represents as a semantic graph node that node in the
checked package, whether or not another declaration names it. When no
declaration of any unit of the package has a node, the checker SHALL refuse
the package at check as `missing_declaration`/`empty-package`, so that check
and compile give the same verdict.

## Rationale

A unit that declares only dimensions, units or enums is a complete library of
declarations: another unit or package names them, and an author checks it on
its own before anything uses it. Writing those nodes only when something
references them would make such a unit check and then fail to compile.

A type alias has no node (FR-322 "Derived type forms and forwarding"): every
use of an alias names the resolved type's node, so that an alias cannot create
a second type identity. A package whose units declare only aliases, or only
`profile`, `import` and `model` header declarations with no selected model,
therefore has nothing FR-322 can write, and FR-322 admits no empty semantic
graph (its `nodes` array has at least one member). Allowing an empty graph
would be a breaking change to the wire for a package that no consumer can read
anything from: an alias is resolved away before the wire, so a dependent
package could not import it either. Refusing at check names the actual rule at
the stage the author runs first, instead of passing check and failing compile
with a refusal about the wire.

An alias-only unit stays fully usable inside a package of several units
(FR-151): its aliases resolve in the other units, and the package has the
nodes those units declare. Only a package with no node at all refuses.

## Inputs

The ordered source units of one package, after their declarations are
checked.

## Outputs

A checked package that holds a node for every node-bearing declaration, or the
refusal `missing_declaration`/`empty-package`.

## Behavior

1. **Node-bearing declarations are written.** Each declaration that FR-322
   gives a node, among them a `dimension` (`scalar_type`/`dimension`), a `unit`
   (`scalar_type`/`unit`), an `enum`, a `record`, a `tuple`, a `union`, a
   `function`, a `predicate` and a selected `model` (its `model`/`model_root`
   node), is a node of the checked package and of its emitted graph, with its
   FR-322 declaration and occurrences. A node that no other declaration names
   is written like any other; reachability from a claim or a function is not a
   condition for writing it.
2. **Aliases introduce no node.** A `type` alias introduces no node, wherever
   it is declared. Its uses name the resolved type's node (FR-322).
3. **An empty package refuses at check.** After every unit of the package is
   checked, if no declaration of any unit has a node, the checker refuses the
   package as `missing_declaration` with cause `empty-package`, at the stage
   that checks declarations, located at the header of the first unit of the
   package. The refusal retains the package name and the source identity of
   each unit. Its message names the rule: a checked package needs at least one
   declaration that has a node, and a type alias has none. No checked package
   is produced.
4. **Check and compile agree.** Compile runs check first, so a package refused
   under item 3 refuses compile with the same code, cause and locus before any
   emission, and compile never reports a refusal about the wire for it. A
   package that check admits compiles, and the strict reader admits its bytes.
5. **Running a unit.** A run of a function or clause first compiles the unit
   under items 3 and 4. A run that selects a function or clause the unit does
   not declare then refuses as that run's own selection rule states.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-746-AC-1 | Under the header `language "ix:native" edition "1-draft";` with `profile v = "filament.value.complete/v1";`, a unit whose only declaration is `dimension Length;` checks and compiles to a package whose graph holds exactly one source `scalar_type`/`dimension` node `Length` with `base: true`; a unit of `dimension Length;` and `unit m : Length = rational(1, 1);` compiles to a package holding that dimension node and one `scalar_type`/`unit` node `m` whose `dimension_node_id` is the dimension node; and the strict reader admits both packages. | Test (TC-1810) |
| FR-746-AC-2 | A unit whose only declaration is `type I = Integer;`, a unit of `type I = Integer;` and `type J = Boolean;`, and a unit with only its header each refuse at check as `missing_declaration`/`empty-package`, located at the unit's header, retaining the package name and the unit's source identity, with no checked package; compiling each refuses with the same code, cause and locus and with no wire refusal; and the refusal message names the rule and does not say that a node cannot be written to the wire. | Test (TC-1811) |
| FR-746-AC-3 | A unit of `type I = Integer;` and `dimension Length;` compiles to a package holding the `Length` dimension node and no node for `I`; a unit of `type I = Integer;` and `function f(x: I): I pure { x }` compiles to a package whose function node `f` has parameter and result type the node of `Integer` and which holds no node for `I`. | Test (TC-1812) |
| FR-746-AC-4 | A package of two units, the first holding only `type Port = Integer;` and the second only `function f(p: Port): Port pure { p }`, checks and compiles in either unit order, and `f`'s parameter and result type is the node of `Integer`; the first unit alone refuses as AC-2 states. | Test (TC-1812) |

## Dependencies

- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns the
  node forms, the rule that an alias has no node and the non-empty graph.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) owns
  dimensions and units.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md) owns the rules for packages of
  several units.
- `proposals/filament-v1/definitions/native-diagnostics.md` lists the
  `empty-package` cause of `missing_declaration`.
