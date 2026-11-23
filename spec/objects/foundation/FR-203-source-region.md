---
id: FR-203
title: "Source region"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/filament-specification/FR-003
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-036
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-047
    type: specifies
---
# FR-203: Source region

## Description

When a producer reports a location in authored source, the producer SHALL express it as a half-open byte region over one identified source document rather than as a reconstructed position.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| document | AuthoredSourceDocument | 1..1 |  |
| start | Integer | 1..1 |  |
| end | Integer | 1..1 |  |
| column | Integer | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| start | zero-based byte offset |
| end | zero-based exclusive byte offset; regions are half-open |
| column | one-based scalar column, where reported |


- **document** (`FR-202` reference) — the authored source document the region
  indexes into. A region is meaningless without it.
- **start** (`byte offset`) — zero-based
  ([FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md)).
- **end** (`byte offset`) — zero-based and exclusive; regions are half-open
  (FR-003, and [FR-036](../../functional/foundation/FR-036-retain-lexical-source-locations.md)
  "original half-open byte spans").
- **column** (`scalar column`, where reported) — one-based (FR-003).

A region must not split a UTF-8 scalar: a span that does receives
`invalid_source_map` (FR-003-AC-2). Decoding JSON string escapes does not
replace original spelling or offsets (FR-036).

### Source-map provenance

A source map produced with authored source available SHALL be verified by its
producer before emission: each mapped occurrence SHALL identify the selected
definition and occurrence that produced it, and every region SHALL be the exact
half-open byte span in the admitted source text for that occurrence. The
producer SHALL also verify byte bounds and UTF-8 scalar boundaries against that
source. A package reader that receives only the package SHALL validate the
source-map wire structure and joins to declared source references, but SHALL
NOT claim to authenticate the authored bytes or that a region identifies the
claimed syntax. A later analyzer or driver SHALL bind each selected definition
to its own admitted definition before using the package's provenance or
interpreting a rule identity for a decision. A package reference or opaque
identity string alone is not authority to select a definition or rule.

## Identity and comparison

- **Identity kind** — *value*. A region has no identity of its own and is
  compared componentwise.
- **Key** — (`document`, `start`, `end`). Equality is componentwise over exactly
  these. The rendered text inside the region, the message that reports it, its
  locale and its formatting never participate: FR-047-AC-1 requires that
  changing or localizing human-readable messages leaves the same typed failure's
  stage, code and cause details unchanged.
- **Key order** — lexicographic over (`document` key, `start`, `end`), for
  collection comparison only. It carries no causal meaning: FR-052 constructs the
  causal graph "without deriving causal edges from source order", so an earlier
  region is not a causal predecessor.
- **Causal order** — none.
- **Absence** — a region that is not available is explicitly absent. Byte zero,
  an empty path or a name-search match cannot stand in for it
  ([native diagnostics catalog](../../../proposals/filament-v1/definitions/native-diagnostics.md)),
  and FR-047-AC-4 requires that diagnostics do not reconstruct a location by
  searching for a repeated name.

## Location kinds are distinct

An authored source span, a JSON field or index path, and a runtime object or
value path remain different location kinds and are not interchangeable
([native diagnostics catalog](../../../proposals/filament-v1/definitions/native-diagnostics.md)).
This artifact types the first kind only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-203-AC-1 | Two regions with equal offsets over different documents compare unequal. | Analysis |
| FR-203-AC-2 | An unavailable region compares only to absence and is not represented as offset zero. | Analysis |
| FR-203-AC-3 | Changing the human-readable message that reports a region leaves the region's key unchanged. | Analysis |
| FR-203-AC-4 | A source span, a JSON field/index path and a runtime value path are not compared to one another. | Inspection |
| FR-203-AC-5 | An emitter given the selected definition, exact authored source and occurrence refuses a region beyond the source, one splitting a UTF-8 scalar, and one not equal to the occurrence's exact source span; it admits the exact span. A package-only reader admits structurally valid retained region data without claiming source-byte authentication, and a consuming analyzer/driver refuses source-level use until it binds the selected definition to its admitted definition. | Test (TC-988) |

## Resolved extraction boundary

All lexical/source regions use zero-based half-open original-byte offsets and
optional one-based Unicode-scalar columns. Lexing performs no newline conversion.
A container extractor may omit only bytes its selected container contract marks
as container-owned, and it records those omissions as monotone mapping segments.
CRLF and final-newline bytes remain authored unless explicitly container-owned;
no omitted byte changes token spelling or relative order.

## Dependencies

- [FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md) and
  [FR-036](../../functional/foundation/FR-036-retain-lexical-source-locations.md) establish
  the region conventions.
- [FR-202](FR-202-authored-source-document.md) supplies the document identity.
- [FR-200](FR-200-filament-assessment-domain.md) supplies the identity discipline.

## Status

Accepted complete-V1 source-region contract.
