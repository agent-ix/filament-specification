---
id: FR-817
title: "Request a TLA+ or Lean export and write its outcome and file"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-120
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-461
    type: depends_on
---

# FR-817: Request a TLA+ or Lean export and write its outcome and file

## Description

When a caller asks for a TLA+ or Lean export, through the lifecycle API, a
run request's `export` member or the CLI verb `export`, the coordinator
SHALL read one export request strictly, write one export outcome document,
and, on the CLI, write the target file only when a package was assembled,
never over an existing file, with the exit code of FR-301's catalog.

## Inputs

An export request (below) and the checked package.

## Outputs

An export outcome document and, on success, the target file.

## Behavior

### Request

```json
{
  "profile": "filament.output.tla2/v1",
  "target_family": "tla",
  "obligations": ["Low"],
  "minimum_fidelity": "conditional",
  "observer": null,
  "limits": {"output_bytes": 1048576}
}
```

- `profile` selects the FR-297 profile; `target_family` (optional) must
  match it (FR-297-AC-5); `obligations` names the requested obligations in
  order, each once; `minimum_fidelity` is FR-811's floor; `observer`
  (optional) selects a structural observer (`sany`, `tlc`, `apalache`,
  `lean`) run on the assembled package; `limits` holds the export limits.
- The export limits SHALL be FR-461 limits with the stage name `export`:
  `export.output_bytes` (FR-461 Behavior 2, the bytes of the target text),
  `export.nodes` (nodes of the dependency closures read) and
  `export.work_units` (one per node mapped and per region written). A denied
  charge stops the export `incomplete` naming the limit, with no package.
- A malformed member refuses `invalid_runtime_input` at its pointer with
  FR-805's causes; a family mismatch, an unknown profile or a duplicate
  obligation refuses as FR-120-AC-2 states.

### Outcome document

```json
{
  "format": "filament.export-outcome/v1",
  "mapping_profile_id": "filament.output.tla2/v1",
  "target_family": "tla",
  "module": "M_Shop",
  "status": "assembled",
  "records": [ {"obligation": "Low", "disposition": "preserved", "conditions": [], "causes": [], "regions": [[120, 188]]} ],
  "package": {"source_package": "<identity>", "generator": "<name> <version>"},
  "target_text": "---- MODULE M_Shop ----\n...",
  "name_map": [ {"name": "op_deposit", "node": "<NodeId>", "region": [10, 40]} ],
  "observer": null
}
```

- `status` is `assembled`, `refused` (with FR-811's `below-fidelity-floor`
  or an admission refusal) or `incomplete` (with the limit).
- `records` are FR-298 records in request order; `target_text` and
  `name_map` are present only when `assembled`.
- `observer` is present when an observer ran: `{tool, version, outcome,
  messages}`, and, for a model checker, `reachable_states` and `verdict`;
  it never changes a record.

### The CLI verb

`export --out <file> <request>` SHALL run the export and write the
outcome document to standard output. When `status` is `assembled` it
writes `target_text` to `<file>`; when `<file>` already exists it refuses
`invalid_runtime_input`/`invalid-value` naming the path and leaves the file
untouched; otherwise it writes no file. Exits: `assembled` `0`; admission
refusal `20`; `below-fidelity-floor` `21` (unsupported); `incomplete` `22`;
a mapper failure `30`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-817-AC-1 | `export --out m.tla` of the counter request writes `M_<package>.tla` text to `m.tla`, an outcome with `status` `assembled`, one `preserved` record with its regions and a name map, and exits `0`. | Test (TC-1976) |
| FR-817-AC-2 | The same command with `m.tla` already present refuses naming the path, exits `20` and leaves the file byte-identical. | Test (TC-1976) |
| FR-817-AC-3 | A request below its fidelity floor exits `21`, writes no file, and its outcome lists the failing records' causes. | Test (TC-1976) |
| FR-817-AC-4 | `export.output_bytes` set below the text length stops the export `incomplete` naming `export.output_bytes`, exits `22` and writes no file; raising it through `--limit export.output_bytes=<n>` assembles it. | Test (TC-1976) |
| FR-817-AC-5 | A request whose `target_family` is `tla` and whose profile is `filament.output.lean4/v1` refuses with no `target_text`, exit `20`. | Test (TC-1976) |

## Dependencies

- [FR-120](FR-120-select-output-mapping-profile.md),
  [FR-301](../tooling/FR-301-expose-complete-cli.md),
  [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md),
  [FR-811](FR-811-classify-every-construct-an-export-reads.md).
