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

- Every name the mapper writes SHALL belong to exactly one of the name
  classes in the table below, and SHALL be spelled as that class says. A
  *prefixed* class writes a kind prefix followed by the mangled FSL name
  (Mangling, below). A *fixed* class writes a name from a closed set the
  mapper owns, never derived from an FSL name. No other name appears in the
  text.

  | Class | Spelling |
  | --- | --- |
  | population variable | `v_` + mangled object type name |
  | FSL type, enum or record type | `T_` + mangled name |
  | Lean population key and record types (FR-812) | `Key_` and `Rec_` + mangled object type name |
  | operation | `op_` + mangled name |
  | FSL function | `f_` + mangled name |
  | operation parameter | `p_` + mangled parameter name |
  | FSL function parameter | `a_` + mangled parameter name |
  | receiver (FSL `self`) of an operation or of a clause read per object | the fixed binder `r_self` |
  | field of an object type, record or variant | `fld_` + mangled field name |
  | quantifier, `let` and lambda binder of an FSL expression | `x_` + mangled binder name |
  | clause and invariant | `c_` + mangled name |
  | Lean clause semantics (FR-813) | `holds_` + mangled clause name |
  | Lean obligation proposition | `Ob_` + mangled clause name |
  | TLA+ obligation theorem | `Th_` + mangled clause name |
  | enum member | `e_` + mangled member name |
  | population bound (language FR-1035) | `b_` + mangled bound name |
  | monitor and history variable (FR-813) | `m_` + mangled clause name, optionally followed by `_` and one of the letters `h`, `r`, `c`, `s` and a decimal ordinal (`m_T_r0`) |
  | step counter (FR-813) | `n_` + mangled clause name |
  | per-clause composed definition (FR-813) | one of the words `vars`, `TypeOK`, `Init`, `Mon`, `Next`, `Spec`, `accept`, `fair` + `_` + mangled clause name |
  | identity universe (FR-813) | `Ids_` + mangled object type name |
  | clock variable (FR-813) | `k_` + mangled object type name + `_` + mangled field name |
  | protocol subject variable (FR-813) | `pr_` + mangled protocol name + `_` + one of the words `over`, `last`, `control`, `observations`, `bounds`, `queues`, `roles`, `memory` |
  | protocol step action (FR-813) | `s_` + mangled protocol name + `_` + mangled static declaration path of the step's node, channel, template or role, optionally followed by `_` and a step word (`fork`, `join`, `timeout`, `duplicate`, `lose`) |
  | protocol binder (FR-813) | `w_` + mangled binder name |
  | protocol subject definition (FR-813) | one of the words `PState`, `PInit`, `PStep`, `PNext`, `PSpec`, `Pvars`, `PBehaviour` + `_` + mangled protocol name |
  | module and namespace | `M_` + mangled package name |
  | generated helper | `Q_`, `F_`, `D_` or `PK_` (FR-813) + a fixed helper word of ASCII letters, optionally followed by `_` and a mangled FSL name the helper is generated for |
  | structural definition | the fixed names `State`, `WF`, `Init`, `Step`, `Behaviour`, `Next`, `Spec`, `TypeOK`, `vars`, `Reachable`, `Q` and `F`, and for FR-813 `J`, `Clocks`, `TBehaviour`, `now`, `tick`, `Tick`, `TNext`, `TimeInv`, `Urgent` and `NZ`, each written once |
  | generated value field | the fixed field and tag names of FR-812's value encodings: `num`, `den`, `tag`, `value`, `bits`, the floating-point `finite` record's fields, the tags `none`, `some`, `finite`, `inf` and `nan`, and the Lean tuple fields `_1`, `_2`, … |
  | generated local binder | one lowercase ASCII letter, optionally followed by decimal digits and then optionally by `'` (`s`, `s'`, `b`, `i`, `j`, `k`, `k2`), bound only inside the definition the mapper generates it in |

- The prefix words `v`, `T`, `Key`, `Rec`, `op`, `f`, `p`, `a`, `r`, `fld`,
  `x`, `c`, `holds`, `Ob`, `Th`, `e`, `b`, `m`, `M`, `Q`, `F`, `D`, `n`,
  `vars`, `TypeOK`, `Init`, `Mon`, `Next`, `Spec`, `accept`, `fair`, `Ids`,
  `k`, `pr`, `s`, `w`, `PK`, `PState`, `PInit`, `PStep`, `PNext`, `PSpec`,
  `Pvars` and `PBehaviour` SHALL be
  pairwise distinct words of ASCII letters, each followed by one `_`. Where
  one word is a proper prefix of another (`T` and `Th`, `f` and `fld`), the
  two names differ at the character after the shorter word (`_` against a
  letter). A prefixed name therefore determines its class by the text before
  its first `_`, because no prefix word holds a `_`, and its FSL name by the
  rest, because mangling is injective. A helper name determines its helper
  word the same way. The receiver binder `r_self` is the only name of the
  `r_` class. A fixed structural name holds no `_`, a generated value field
  either holds no `_` or is a Lean tuple field `_<digits>` that begins with
  `_`, and a generated local binder holds no `_`; every prefixed name begins
  with a letter and holds a `_` after its prefix word. The whole naming is
  therefore injective: two generated names are equal only when they are the
  same class and the same FSL name, or the same fixed name.
- The receiver has its own class so that it never collides with a declared
  parameter. FR-012 admits a reserved word, `self` included, as a domain
  model parameter name, so a parameter spelled `self` is `p_self`, distinct
  from the receiver `r_self`. Likewise a field spelled `num` is `fld_num`,
  distinct from the generated rational field `num`, and a `let` binder
  spelled `Next` is `x_Next`, distinct from the structural `Next`.
- No prefixed name SHALL collide with a TLA+ or Lean keyword, operator or
  library name. TLA+ reserved words (`DOMAIN`, `INSTANCE`, `LET`, …) and
  Lean keywords (`instance`, `fun`, `def`, …) hold no `_` except words that
  begin with none of the prefix words above (Lean's `set_option`,
  `termination_by`, `decreasing_by`, `simp_all`, `macro_rules`), TLA+'s
  `WF_` and `SF_` fairness operators begin with no prefix word, and no
  module or namespace the output uses begins with a prefix word followed by
  `_`. No fixed name SHALL be a TLA+ or Lean keyword: the structural names,
  the generated value fields and the one-letter local binders are none.
  Fixed names do not shadow one another, because a generated local binder is
  bound only where no structural name of the same spelling is in scope.
- *Mangling* SHALL keep ASCII letters and digits, write `_` as `__`, write
  the namespace separator `::` as `_N_`, and write every other Unicode
  scalar as `_u` followed by its uppercase hexadecimal code point and `_`.
  The mapping is injective: distinct FSL names give distinct target names.
- The package SHALL carry a name map from each prefixed name to its FSL
  node id and source region; a target tool's trace or error names a
  generated name, which the map reads back to the FSL node. A fixed name
  (the receiver binder `r_self`, a structural name, a generated value field
  or a generated local binder) names no FSL node of its own and has no
  entry; a trace value of `r_self` is read through the entry of the
  operation or clause that binds it.
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
- The comment SHALL be spelled exactly `<marker> fsl: <node id> <region>`,
  with `<marker>` `\*` (TLA+) or `--` (Lean), one space between tokens and
  nothing after the region. `<node id>` is the node's `node_id` as the
  checked package writes it. `<region>` is
  `<authority>/<identity>:<start>-<end>`: the region's source document by
  its authority and identity, then `:`, then the zero-based start offset
  and the exclusive end offset in bytes, as decimals with no sign or leading
  zero, joined by `-`. Example:
  `\* fsl: <node id> agent-ix/example/counter-unit:120-188`. This is the
  same `<source>:<start>-<end>` form a diagnostic prints for a region it
  cannot resolve to a line, so one reader parses both. The region is read
  from the right: the last `:` ends the source and the one `-` after it
  separates the offsets. A node with no source region writes `none` in place
  of the region (FR-203 "Absence"), never `0-0`. A space or a character
  outside printable ASCII in the authority or identity is written as
  `\u{XXXX}`, so the node id and the region are each one token without
  spaces.
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
| FR-814-AC-6 | Every `fsl:` comment line of the counter's TLA+ and Lean exports matches `^(\\\*\|--) fsl: [^ ]+ ([^ ]+:(0\|[1-9][0-9]*)-(0\|[1-9][0-9]*)\|none)$`. The node id is a node of the package. The region's source is the unit's `<authority>/<identity>` and `start < end`, and that region's bytes are the definition's source declaration. A definition of a node with no source region writes `none`. | Test (TC-2030) |
| FR-814-AC-7 | A model whose object type has a field `total`, whose operation `add` has a parameter `total`, and which declares a function `total` with a parameter `total`, exports in both targets the field as `fld_total`, the operation parameter as `p_total`, the function as `f_total` and the function parameter as `a_total`; the four names are pairwise distinct and each name-map entry reads back to its own FSL node (field, operation parameter, function, function parameter). Text that writes the field as `f_total`, or either parameter as unprefixed `total`, fails this criterion. | Test (TC-1973) |
| FR-814-AC-8 | An operation `rename(self: Text)` of object type `Item`, whose declared parameter is spelled `self` (FR-012), exports in TLA+ as `op_rename(r_self, p_self)` bound in `Next` by `\E r_self \in DOMAIN v_Item, p_self \in Dom` and in Lean as `def op_rename (s s' : State) (r_self : Key_Item) (p_self : …)`, with `p_self` mapped to the parameter's node and `r_self`, a fixed name, with no name-map entry, and SANY and Lean parse both files. A mapper that wrote the receiver and the parameter under one name would declare one formal parameter twice, which SANY reports as a multiply-defined symbol and Lean as a duplicate binder, so this criterion fails for it. | Test (TC-1973) |
| FR-814-AC-9 | An invariant whose quantifier and `let` binders are named `Next`, `instance`, `DOMAIN`, `num` and `tag`, over an object type with fields `num` and `tag`, a `Decimal` field and an `Option` field, exports the binders as `x_Next`, `x_instance`, `x_DOMAIN`, `x_num` and `x_tag` and the fields as `fld_num` and `fld_tag`, distinct from the structural `Next` and the generated rational fields `num` and `tag`; SANY and Lean parse both files. Unprefixed, `DOMAIN` is a TLA+ reserved word and `instance` a Lean keyword, both of which fail to parse, and a binder `Next` redefines the module's `Next`, which SANY rejects, so a mapper that writes any binder unprefixed fails this criterion. | Test (TC-1973) |


## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-125](FR-125-produce-deterministic-output-packages.md); FR-203 owns
  source regions.
