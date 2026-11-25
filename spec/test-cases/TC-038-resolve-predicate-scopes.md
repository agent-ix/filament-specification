---
id: TC-038
title: "Resolve predicate names under explicit lexical scopes"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-038
    type: verifies
---
## Description

Accepted portable controls for [FR-038](../functional/foundation/FR-038-resolve-predicate-scopes.md).
These expected cases have not been fully implemented
or executed as this standard's conformance evidence.

## Test Procedure

Link two source units with local alias M bound to different models and distinct predicate declarations. Introduce duplicate predicate names, duplicate parameters, a nested local-binder collision, a case-mismatched call, an alias-name shadow attempt and a native-declaration-name shadow attempt. In a named predicate with parameter x, use x in a let initializer and quantifier domain, then add let and quantifier binders also named x and read x in their bodies. Compare a valid forward acyclic call and a predicate body attempting implicit self/result/pre access.

## Expected Results

Aliases remain source-unit local and every admitted use has one exact declaration. Duplicate parameters, nested local-binder collisions, alias-name shadowing, native-declaration-name shadowing and hidden environment access refuse with original loci. A `let` or quantifier binder named x is admitted when x is a visible parameter; reads in its body resolve to the local binder, while the let initializer or quantifier domain resolves against the outer parameter. Reusing x in distinct predicate declarations is admitted; forward-call order alone does not change meaning.
