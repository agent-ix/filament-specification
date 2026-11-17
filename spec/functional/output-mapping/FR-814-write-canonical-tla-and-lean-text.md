---
id: FR-814
title: "Write canonical TLA+ and Lean 4 text with injective names"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-125
    type: depends_on
---

# FR-814: Write canonical TLA+ and Lean 4 text with injective names

## Description

When the TLA+ or Lean 4 mapper writes its text, it SHALL derive every
target name from the FSL name by one injective mangling that cannot collide
with a target keyword or library name, order every definition by one fixed
rule, and lay out the bytes canonically, so that equal requests give
byte-identical text (FR-125) and every name maps back to one FSL node.

## Inputs

The regions of FR-812 and FR-813.

## Outputs

The target bytes and the package's name map.

## Behavior

### Names

- Every generated name SHALL be a kind prefix followed by the mangled FSL
  name: `v_` variables, `T_` types, `op_` operations, `f_` functions, `c_`
  clauses and invariants, `holds_` Lean clause semantics (FR-813), `Ob_`
  Lean obligation propositions, `Th_` TLA+ obligation theorems, `e_` enum
  members, `m_` monitor variables, `M_` modules, and `Q_`, `F_` and `D_`
  generated helpers. Each prefix is a distinct word of ASCII letters
  followed by one `_`; where one word is a proper prefix of another (`T`
  and `Th`), the two names differ at the character after the shorter word
  (`_` against a letter), so names of different kinds never coincide, and
  with the injective mangling below the whole naming is injective.
  No TLA+ or Lean keyword, and no name of a module or namespace the output
  uses, begins with one of these prefixes, so no generated name collides
  with one.
- The structural names `State`, `WF`, `Init`, `Step`, `Behaviour`, `Next`,
  `Spec`, `TypeOK`, `vars`, `Reachable`, `Q` and `F` SHALL be written unprefixed,
  once each; no mangled name equals one, because every mangled name carries
  a prefix.
- *Mangling* SHALL keep ASCII letters and digits, write `_` as `__`, write
  the namespace separator `::` as `_N_`, and write every other Unicode
  scalar as `_u` followed by its uppercase hexadecimal code point and `_`.
  The mapping is injective: distinct FSL names give distinct target names.
- The package SHALL carry a name map from each generated name to its FSL
  node id and source region; a target tool's trace or error names a
  generated name, which the map reads back to the FSL node.
- The TLA+ module name and file name SHALL be `M_` followed by the mangled
  package name, with the file `<module>.tla`; the Lean file is `<module>.lean`
  and its definitions sit in `namespace <module>`.

### Text

- Output SHALL be ASCII only: non-ASCII FSL text appears only as scalar
  sequences (FR-812) and in comments as `\u{XXXX}` escapes; the Lean output
  writes `forall`, `exists`, `fun` and `->` rather than their Unicode
  notations.
- Each definition SHALL be preceded by one comment line `\* fsl: <node id>
  <source region>` (TLA+) or `-- fsl: <node id> <source region>` (Lean),
  the region written as FR-203's half-open byte region. A comment holds no
  path, timestamp, user name or tool version.
- Definitions SHALL be written in dependency order: a definition follows
  every definition it uses; definitions with no dependency between them
  follow the checked package's canonical node order. A recursive group is
  written as one block in node order.
- Lines SHALL end with LF, indentation SHALL be two spaces per level, one
  blank line SHALL separate definitions, no line SHALL carry trailing
  whitespace, and the file SHALL end with exactly one LF.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-814-AC-1 | Operations named `MODULE`, `Next` and `theorem` in FSL export as `op_MODULE`, `op_Next` and `op_theorem`, and SANY and Lean parse both files as observers with no name clash. | Test (TC-1973) |
| FR-814-AC-2 | FSL names `a_b`, `a__b` and `Shop::Order` mangle to `a__b`, `a____b` and `Shop_N_Order` (as operations, `op_a__b`, `op_a____b`, `op_Shop_N_Order`), three distinct names; a name with `é` exports with `_uE9_`. | Test (TC-1973) |
| FR-814-AC-3 | Two exports of one request give byte-identical text that is ASCII only, ends with one LF, has no trailing whitespace, and has one `fsl:` comment per definition naming its node and region. | Test (TC-1973) |
| FR-814-AC-4 | A function used by an invariant is written before it, and reordering two independent declarations in the source changes the text only by their node order. | Test (TC-1973) |
| FR-814-AC-5 | A TLC error trace naming `v_Account` and `op_deposit` reads back through the name map to the FSL object type and operation nodes. | Test (TC-1973) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-125](FR-125-produce-deterministic-output-packages.md); FR-203 owns
  source regions.
