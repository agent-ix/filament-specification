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
  - target: ix://agent-ix/filament-specification/FR-299
    type: depends_on
---

# FR-817: Request a TLA+ or Lean export and write its outcome and file

## Description

When a caller asks for a TLA+ or Lean export, through the lifecycle API, a
`native-run/1` request's `export` member or the CLI verb `export`, the
coordinator SHALL read one export request strictly, write one export outcome
document, and, when an output file is named, write the target file only when
a package was assembled, never over an existing file, with the exit code of
FR-301's catalog.

### Decision: the export request is a run request member

The CLI verb's operand is the same `native-run/1` request file that names
the program, its models and its sources for every other run, with the export
request as its `export` member. A separate export document would repeat the
program, model and source selection that the run request already owns, and
would need its own reader for them. One request file then serves the library
call, the CLI verb and repeated runs.

### Decision: the language writes the file

The language implementation writes the target file, as it writes a compiled
package for a compile request's `output` member. The CLI passes the path it
was given and writes no target bytes of its own. One writer owns the
never-overwrite rule and the no-partial-file rule, so the request member and
the CLI option produce byte-identical files.

### Decision: the outcome reports no exploration of its own

An export maps the checked package to target text; it explores no states.
The outcome therefore carries no native state count or verdict. When an
observer model checker runs, its `reachable_states` and `verdict` are the
tool's report. The qualification of FR-818 compares them with the language's
own explicit-state exploration of the same model, which is a separate
analysis run, so an export never runs an exploration as a side effect and its
cost is bounded by the export limits alone.

## Inputs

A `native-run/1` request whose `export` member is the export request below,
and the checked package that request's program compiles to.

## Outputs

An export outcome document and, when an output file is named and a package
was assembled, the target file.

## Behavior

### Request

The export request is the `export` member of a `native-run/1` request:

```json
{
  "format": "native-run/1",
  "request": {
    "models": [{"format": "semantic-ir/2.0.0", "source": {"file": "counter.json", "authority": "agent-ix", "identity": "example/counter", "document": "Counter", "formal_revision": 1}}],
    "program": {"source": {"file": "counter.native", "authority": "agent-ix", "identity": "example/counter-unit", "document": "Counter", "formal_revision": 1}},
    "export": {
      "profile": "filament.output.tla2/v1",
      "target_family": "tla",
      "obligations": ["Bounded"],
      "minimum_fidelity": "conditional",
      "observer": null,
      "initial": "counter-0.json",
      "limits": {"export.output_bytes": 16777216},
      "output": {"file": "counter.tla"}
    }
  }
}
```

- `profile` selects the FR-297 profile; `target_family` (optional) must
  match it (FR-297-AC-5); `obligations` names the requested obligations in
  order, each once; `minimum_fidelity` (optional, default `conditional`) is
  FR-811's floor; `observer` (optional) selects a structural observer
  (`sany`, `tlc`, `apalache`, `lean`) run on the assembled package.
- `initial` (optional) names a `filament.state.snapshot/v1` document,
  resolved against the request file's directory, whose state the target's
  `Init` states (FR-812). Without it, `Init` states the model's declared
  initial constraints.
- `limits` (optional) holds export limits by their FR-461 setting names.
  The export limits SHALL be FR-461 limits with the stage name `export`:
  `export.output_bytes` (FR-461 Behavior 2, the bytes of the target text),
  `export.nodes` (nodes of the dependency closures read) and
  `export.work_units` (one per node mapped and per region written). Each is
  an unsigned integer used as given. An absent limit takes its published
  default: `export.output_bytes` 16777216 bytes, `export.nodes` 100000 nodes
  and `export.work_units` 16777216 units, the defaults of the compile
  lifecycle's emit and checking limits, so a model that compiles under the
  defaults exports under them. A denied charge stops the export `incomplete`
  naming the limit, with no package.
- `output` (optional) is a closed record `{"file": P}`, `P` a non-empty path
  of the target file to create, resolved against the request file's
  directory.
- A malformed member refuses `invalid_runtime_input` at its pointer with
  FR-805's causes; an unknown `limits` name refuses at its pointer naming
  the setting; a family mismatch, an unknown profile or a duplicate
  obligation refuses as FR-120-AC-2 states.

### Outcome document

```json
{
  "format": "filament.export-outcome/v1",
  "mapping_profile_id": "filament.output.tla2/v1",
  "target_family": "tla",
  "module": "M_Counter",
  "status": "assembled",
  "records": [ {"obligation": "Bounded", "disposition": "preserved", "conditions": [], "causes": [], "regions": [[120, 188]]} ],
  "package": {
    "source_package": {"identity": "example/counter-unit", "version": "1"},
    "target_profile": "filament.output.tla2/v1",
    "generator": {"identity": "<generator>", "version": "<semantic version>"},
    "limits": {"export.output_bytes": 16777216, "export.nodes": 100000, "export.work_units": 16777216}
  },
  "target_text": "---- MODULE M_Counter ----\n...",
  "name_map": [ {"name": "op_inc", "node": "<NodeId>", "region": [10, 40]} ],
  "observer": null,
  "output": {"path": "counter.tla", "bytes_written": 1234}
}
```

- `status` is `assembled`, `refused` (with FR-811's `below-fidelity-floor`
  or an admission refusal) or `incomplete` (with the limit).
- `records` are FR-298 records in request order; `target_text` and
  `name_map` are present only when `assembled`.
- `package` is present only when `assembled`. It holds FR-299's key apart
  from the records: the source package reference (identity and version),
  the exact target profile, the generator identity and semantic version, and
  the admitted export limits, every one of the three named with its
  effective value. FR-299's package key is this member together with the
  outcome's `records` in order. It carries no digest, path or timestamp, so
  equal admitted requests give equal `package` members (FR-299-AC-3), and an
  evidence request names an export by this member (FR-815).
- `observer` is present when an observer ran: `{tool, version, outcome,
  messages}`, and, for a model checker, `reachable_states` (the tool's
  distinct-state count) and `verdict` (`holds` or `violated`); it never
  changes a record.
- `output` is present exactly when an output file was named. `path` is the
  path as the caller gave it; `bytes_written` is the number of target bytes
  written, which equals the length of `target_text` when the file was
  written and is `0` when no file was written.

### The CLI verb

`export <request> [--output <file>]` SHALL run the export of the request's
`export` member and write FR-301's outcome document, with operation
`export`, to standard output; in machine mode that document's result holds
the export outcome document unchanged.

- `--output <file>` names the target file, resolved against the working
  directory. The CLI hands the path to the language implementation as the
  request override of the `output` member and writes no target bytes of its
  own. The option and the request's `output` member write byte-identical
  files.
- The target file SHALL be written only when `status` is `assembled`, only
  after the package is assembled, with no partial file on any failure;
  otherwise no file is written.
- When a file or directory already exists at the path, the export SHALL
  refuse `invalid-request` naming the path, exit `20`, with no stage
  reached, and leave the path untouched.
- A request carrying an `output` member while `--output` is given SHALL
  refuse `invalid-request` naming `output` and `--output`, exit `20`, and
  write no file.
- `--limit <name>=<value>` SHALL set an export limit, and the compile
  settings of the program the request compiles, through the FR-461 settings
  operation; a setting given both there and in the request's `limits`
  refuses `invalid-request` naming `limits` and the setting, exit `20`.
- Exits: `assembled` `0`; admission refusal `20`; `below-fidelity-floor`
  `21` (unsupported); `incomplete` `22`; a mapper failure `30`.

`export --evidence <request>` SHALL read a `native-run/1` request carrying
an `export_evidence` member and settle each named obligation as FR-815
states, writing FR-301's outcome document with operation `export` and one
item per settled obligation carrying its FR-331 terminal record. It takes no
other operand, and `--evidence` with `--output` refuses `invalid-request`
naming both, exit `20`, before any stage runs.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-817-AC-1 | `export <request> --output m.tla` over the counter request (operation `inc`, state clause `value <= 5`) writes `M_<package>.tla` text to `m.tla`, an outcome with `status` `assembled`, one `preserved` record with its regions, a name map and `output` `{"path": "m.tla", "bytes_written": N}` with `N` the length of `target_text`, and exits `0`; the same request carrying `output` `{"file": "m.tla"}` and no option writes a byte-identical file. | Test (TC-1976) |
| FR-817-AC-2 | The same command with `m.tla` already present refuses `invalid-request` naming the path, exits `20` and leaves the file byte-identical; the request carrying an `output` member with `--output` given refuses naming `output` and `--output`, exits `20` and writes neither file. | Test (TC-1976) |
| FR-817-AC-3 | A request below its fidelity floor exits `21`, writes no file, and its outcome lists the failing records' causes. | Test (TC-1976) |
| FR-817-AC-4 | `limits` `{"export.output_bytes": 1}` stops the export `incomplete` naming `export.output_bytes`, exits `22` and writes no file; the request without that entry and with `--limit export.output_bytes=1` stops the same way; with `--limit export.output_bytes=16777216` it assembles; the entry and the option together refuse naming `limits` and `export.output_bytes`, exit `20`. | Test (TC-1976) |
| FR-817-AC-5 | A request whose `target_family` is `tla` and whose profile is `filament.output.lean4/v1` refuses with no `target_text`, exit `20`. | Test (TC-1976) |
| FR-817-AC-6 | The `package` member of the counter export holds exactly `source_package` (identity and version), `target_profile`, `generator` (identity and version) and `limits` with the three export settings at their defaults; two exports of the same request give equal `package` members, and an export under a different `export.nodes` gives a different one. | Test (TC-2020) |
| FR-817-AC-7 | `export --evidence <request>` over an evidence request naming the counter export's `package` and `Bounded` writes an outcome document with operation `export` and one item for `Bounded`; `export --evidence <request> --output m.tla` refuses naming `--evidence` and `--output`, exit `20`, and writes no file. | Test (TC-2020) |

## Dependencies

- [FR-120](FR-120-select-output-mapping-profile.md),
  [FR-301](../tooling/FR-301-expose-complete-cli.md),
  [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md),
  [FR-811](FR-811-classify-every-construct-an-export-reads.md),
  [FR-299](../../objects/output-mapping/FR-299-generated-output-package.md)
  (the package key the `package` member carries),
  [FR-815](FR-815-state-what-an-exported-proof-certifies.md) (evidence over an
  export).
