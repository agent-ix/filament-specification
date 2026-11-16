---
id: FR-570
title: "Select the integer-division law from a unit header"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-147
    type: references
  - target: ix://agent-ix/filament-specification/FR-001
    type: references
  - target: ix://agent-ix/filament-specification/FR-301
    type: references
---

# FR-570: Select the integer-division law from a unit header

## Description

When a source unit evaluates integer `div` or `rem`, the frontend SHALL take
the unit's integer-division law from a header `profile` declaration that
names one of the three integer-division definitions.

## Inputs

The unit's header `profile` declarations; an optional integer-division
alternative role a caller supplies for the unit's lock selection.

## Outputs

The unit's package selection with its `integer_division_profile` slot filled
by the selected law, or a located refusal.

## Behavior

`div` and `rem` mean different things under the truncating, floor and
Euclidean laws ([FR-147](FR-147-evaluate-integer-division-domains.md)), so the
law is part of what a unit says and part of its package identity. The unit
states it in its header, the one place every consumer of the unit reads. A
caller that compiles the unit may supply the law instead, as the lock
selection's integer-division alternative role, a command option or a request
member ([FR-301](../tooling/FR-301-expose-complete-cli.md)). A supplied law
fills the package selection's `integer_division_profile` slot, so two compiles
of one text under two supplied laws give two packages with different
identities, never one identity with two meanings, and a supplied law that
disagrees with the header is refused, so it never overrides what the author
wrote. An operator keyword would let one unit mix laws, and a default law would
choose a meaning the author never stated; FR-147 rules out both.

- When a header `profile` declaration's identity is
  `filament.value.integer-division.euclidean/v1`,
  `filament.value.integer-division.floor/v1` or
  `filament.value.integer-division.truncating/v1`, the frontend shall select
  that law for the unit and fill the package selection's
  `integer_division_profile` slot with it
  ([value-complete](../../../proposals/filament-v1/definitions/value-complete.md#package-selection)).
  Such a declaration selects no layer and adds no admitted form.
- The selected law governs every integer `div` and `rem` in the unit's own
  declarations, including refinement rows. An imported unit is governed by
  its own header; a law never crosses an import.
- `mod` is the Euclidean remainder under every law. A law header never
  changes it, and a unit whose only integer-division operator is `mod` needs
  no law header.
- Two law headers naming the same law select it once. If two law headers name
  different laws, then the frontend shall refuse with `invalid_package`,
  cause `conflicting-definition`, at the later header's identity literal,
  retaining both definition references.
- If a unit evaluates `div` or `rem` and neither a law header nor a
  caller-supplied alternative role selects a law, then the frontend shall
  refuse each such operator with `missing_declaration`, cause
  `missing-selection`, at the operator. No law is selected by default.
- When a caller supplies the lock selection's integer-division alternative
  role ([FR-001](../foundation/FR-001-select-native-profile.md)-AC-7) and the
  unit has no law header, the supplied role selects the law. If both are
  present and name different laws, then the frontend shall refuse with
  `invalid_package`, cause `conflicting-definition`, at the header's
  identity literal.
- The command line and request files supply the alternative role by law name:
  `euclidean`, `floor` or `truncating` select the definitions above. When a
  command or request supplies a law, the frontend shall apply the rules of the
  supplied role above to it, with the same refusals. A request's program law
  never reaches a library, which takes its own header law or its own supplied
  law.
- A law header in a unit with no `div` or `rem` is admitted. The package
  selection holds no integer-division role, because the `integer_div_rem`
  trigger is absent, so the unit's package identity equals that of the same
  unit without the header.
- If a declaration's `using` alias, or any other profile use site, names a
  law header, then the frontend shall refuse with `unknown_profile`, cause
  `wrong-selection-role`, at the alias.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-570-AC-1 | For each law, a complete-value unit whose header names that law's definition admits `a div b` and `a rem b` over `a: Int[-7, 7]`, `b: Int[1, 3]`, its package selection holds exactly that law's role, and `-7 div 3` and `-7 rem 3` evaluate to that law's FR-147 quotient and remainder. | Test (TC-1280) |
| FR-570-AC-2 | The same unit with no law header and no supplied alternative role refuses `missing_declaration`/`missing-selection` at each `div` and `rem`; a unit using only `mod` is admitted with no law. | Test (TC-1280) |
| FR-570-AC-3 | Two law headers naming floor and truncating refuse `invalid_package`/`conflicting-definition` at the second identity literal; two naming floor select floor once; a floor header with a supplied truncating alternative role refuses the same way at the header's identity literal. | Test (TC-1280) |
| FR-570-AC-4 | A declaration `using` a law header's alias refuses `unknown_profile`/`wrong-selection-role` at the alias. | Test (TC-1280) |
| FR-570-AC-5 | A unit with a floor law header and no `div` or `rem` is admitted with no integer-division role in its package selection, and its package identity equals that of the same unit without the header; an importing unit selecting floor and a library selecting truncating evaluate their own `-7 div 3` to `-3` and `-2`. | Test (TC-1280) |
| FR-570-AC-6 | A complete-value unit with `a div b` and no law header, compiled with a supplied `floor` law name from a command option or request member, admits the operator and evaluates `-7 div 3` to `-3`; with a `floor` header and a supplied `truncating` it refuses `invalid_package`/`conflicting-definition` at the header's identity literal; a supplied name `ceiling` refuses `unknown_profile`/`unsupported-selection` before any source is read; and a library with `div`, no header and no law of its own refuses `missing_declaration`/`missing-selection` though the importing request supplies one. | Test (TC-1652) |

## Dependencies

- [FR-147](FR-147-evaluate-integer-division-domains.md) owns the laws and the
  ban on mixing or inferring them.
- The [complete value definition](../../../proposals/filament-v1/definitions/value-complete.md)
  owns the package selection and its `integer_div_rem` trigger.
- [FR-001](../foundation/FR-001-select-native-profile.md) owns header profile
  selection of layers.

## Status

Draft. Specification acceptance does not claim implementation.
