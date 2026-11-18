---
id: TC-2002
title: "Malformed reference-literal spellings refuse at the parser"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-480
    type: verifies
---

# TC-2002: Malformed reference-literal spellings refuse at the parser

## Description

Check that a reference literal is recognized only as a solid
`qualified-name '#' string` spelling at the start of a `primary`, that `#`
inside a string or comment is text, and that each malformed spelling refuses
`invalid_syntax` with the grammar table's cause and locus. Scope:
FR-480-AC-6.

Fixture: a complete-V1 unit selecting domain package `M` (object type
`Server`) whose clause body is `self = <spelling>`.

## Test Procedure

Parse each vector's spelling in the fixture's body position through the real
parser boundary and compare admission, the diagnostic code, its cause and its
locus with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RLS-01 | `M::Server #"s1"`; `(x)#"s1"`; `f(x)#"s1"` | `invalid_syntax`/`unexpected-token` at the `#` |
| RLS-02 | `M::Server#"a"#"b"` | `invalid_syntax`/`unexpected-token` at the second `#` |
| RLS-03 | `M::Server# "s1"`; `M::Server#1`; `M::Server#)` | `invalid_syntax`/`unexpected-token` at `"s1"`, `1` and `)` |
| RLS-04 | a unit whose source ends with `M::Server#` | `invalid_syntax`/`unexpected-end` at the end of the source |
| RLS-05 | `M::Server#"\q"` | `invalid_syntax`/`invalid-escape` at the string |
| RLS-06 | `"a#b"`; a body `true // a#b` | admitted: one string token, and a comment |
| RLS-07 | `M::Server#""` | admitted at the parser as one `reference-literal` |

## Expected Results

Each spelling is admitted or refused exactly as stated, each refusal read by
its typed code, cause and locus and never from a message.
