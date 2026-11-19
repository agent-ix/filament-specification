---
id: FR-339
title: "Define the complete native V1 grammar"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-131
    type: references
  - target: ix://agent-ix/filament-specification/AD-003
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-035
    type: references
  - target: ix://agent-ix/filament-specification/FR-746
    type: references
---

# FR-339: Define the complete native V1 grammar

## Description

When a source declares the complete V1 profile, the parser SHALL recognize
exactly the composed EBNF and lexical rules in the complete-V1 grammar contract.

## Inputs

UTF-8 source bytes, selected native edition and complete-V1 profile identity.

## Outputs

A lossless source tree containing every token and trivia occurrence, or located
lexical/syntactic refusals.

## Behavior

The [complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) is
incorporated by this requirement as the normative syntax contract. Its base
grammar and complete-V1 facets form one grammar; a conforming implementation may
factor parser modules but cannot expose backend-specific syntax, omit a facet or
accept an undeclared extension under the complete-V1 identity. Reserved words,
literal spellings, operator precedence and declaration forms are fixed by
the linked language profile. Lossless parsing preserves source bytes and loci
without granting syntactically invalid input semantic meaning. Parsing,
checking and packaging take no provider registry, so their output is
independent of which backends or plugins are installed.

### Decision: a unit with no declaration parses and refuses at check

A complete unit holds zero or more declarations after its header, profile,
import and model lines. The parser admits a unit with none, and the rule
that a package needs at least one declaration that has a node is checked at
S3 as [FR-746](FR-746-write-every-node-bearing-declaration-and-refuse-a-package-that-writes-no-node.md)
states (`missing_declaration`/`empty-package` at the unit's header, exit 20).
One rule then refuses a header-only unit, a unit of profiles only and a unit
of type aliases only, with one code and locus; a syntax refusal at end of
input would give the same empty package a second outcome. A word after the
last complete declaration stands where a top-level declaration begins, so a
word that is no core keyword and no selected extension's leading keyword
refuses `unsupported_construct`/`declaration-form` naming the word at its
span (exit 21), as every unknown declaration word does; a token that cannot
begin any declaration, such as `}` or a number, refuses
`invalid_syntax`/`unexpected-token` at that token (exit 20).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-339-AC-1 | Every production in the base and complete-V1 facet grammar has positive, boundary and located-negative corpus vectors. | Test (TC-180) |
| FR-339-AC-2 | Parsing and lossless rendering reproduce accepted UTF-8 source bytes exactly, including comments and whitespace. | Test (TC-180) |
| FR-339-AC-3 | Backend installation cannot add, remove or reinterpret syntax under the same language/profile identity: compiling one source with an empty provider registry and with a populated one gives byte-equal checked-package bytes and the same check outcome. | Test (TC-180) |
| FR-339-AC-4 | An undeclared keyword, operator or extension refuses at its exact source span without producing a checked declaration. | Test (TC-180) |
| FR-339-AC-5 | Under `1-draft` with the complete-V1 profile, `helper`, `rec`, `cast` and `Tuple` are ordinary identifiers (complete-V1 grammar, lexical contract). For each spelling `w` of the four, each of the following units, beside `profile v = "filament.value.complete/v1";`, parses and checks with no `invalid_syntax` and no `unsupported_construct`: `function w using v(x: Integer): Integer pure { x }` with `function c using v(): Integer pure { w(1) }`, `c()` evaluating to `1`; `profile w = "filament.value.complete/v1";` with `function f using w(x: Integer): Integer pure { x }`; `function g using v(w: Integer): Integer pure { let z = w in z }`; `function h using v(x: Integer): Integer pure { let w = x in w }`; `record w { w: Integer; }`; `enum E { w }`; `tuple w(Integer);`; `union w { A, B }`; `union U { w, B }`; `type w = Integer;`; and `model w = "acme/rpc";` over an admitted domain package `acme/rpc`, beside `function k using v(): Integer pure { 1 }`. `record tuple { a: Integer; }` refuses `invalid_syntax`/`unexpected-token` at `tuple`. | Test (TC-180) |
| FR-339-AC-6 | `language "ix:native" edition "1-draft";` alone, and the same header followed by `profile v = "filament.value.complete/v1";`, each parse with no `invalid_syntax` diagnostic and format unchanged; checking or compiling either refuses `missing_declaration`/`empty-package` at the `language` header, exit 20, with no checked package. | Test (TC-2043) |
| FR-339-AC-7 | Beside `profile v = "filament.value.complete/v1";`, `function probe using v(): Boolean pure { true } garbage` refuses `unsupported_construct`/`declaration-form` naming `garbage` at its span, exit 21, with no checked package and no `invalid_syntax`; `function probe using v(): Boolean pure { true } }` refuses `invalid_syntax`/`unexpected-token` at the trailing `}`, exit 20. | Test (TC-2044) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) is the
  incorporated lexical and syntactic authority.
- [Subsystem and interface index](../../subsystems/index.md) owns the parser and
  source-package boundary.
- Every requirement linked in frontmatter is a normative prerequisite.
