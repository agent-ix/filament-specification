---
id: SR-202611210022442313144108556
title: "Code review of the compact formatter corpus expectation"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; proposals/conformance-corpus-v1/fixtures/complete-v1.json; QSL-2823"
review_set: subset
---

## Summary

Reviewed the single formatter-vector expectation correction in PR 805. The output header follows the existing compact formatting rules; the authored input and all vector metadata remain unchanged.

## Verdict

**PASS (source review)** — no scoped defect found. The named upstream checks and the consuming formatter qualification checks remain pending; this is not a runtime or full qualification verdict.

## Scope

```yaml
scope:
  - id: V1-TOOL-004-positive-format
    path: proposals/conformance-corpus-v1/fixtures/complete-v1.json
    role: examined
    excerpt: 'function zero using v(): Integer pure {'
  - id: FR-003-AC-12
    path: agent-ix/filament-spec-language/spec/functional/FR-003-format-native-source.md
    role: examined
    excerpt: 'Formatting `present (x)`, `forall (r in c : p)`, a parameter `(v : T)`, `Int [0,9]`, `always [0,30] holds (p)`, `convert < T > (e)`, `Option < Int[0,9] >` and `a<b` prints `present(x)`, `forall(r in c: p)`, `(v: T)`, `Int[0, 9]`, `always[0, 30] holds(p)`, `convert<T>(e)`, `Option<Int[0, 9]>` and `a < b`'
  - id: FR-336
    path: spec/objects/interfaces/FR-336-conformance-corpus.md
    role: examined
    excerpt: |-
      `expected.disposition` is
      either `value` or `refusal` and is the vector's independently authored
      oracle: `value` carries the operation's
      typed JSON result; `refusal` carries `code` and `cause`, with optional
      operation-defined `details`.
  - id: FR-336-AC-4
    path: spec/objects/interfaces/FR-336-conformance-corpus.md
    role: examined
    excerpt: 'The published schema admits the complete-v1 fixture and refuses a missing required member, an unknown member, and out-of-domain class, consumer-type or expected-disposition values.'
  - id: FR-336/Identity and validation
    path: spec/objects/interfaces/FR-336-conformance-corpus.md
    role: context_only
    excerpt: |-
      Vector identity covers its complete input bytes and expected oracle; corpus
      identity covers the ordered vector set.
  - id: FR-660-AC-1
    path: spec/functional/ecosystem/FR-660-version-the-wire-the-published-files-and-the-consumer-facades.md
    role: context_only
    excerpt: '`published_file` returns the exact bytes of every file under `proposals/` by its repository-relative path, and `None` for a path outside `proposals/` and `spec/test-cases/` and for a path under them that names no file'
```

## Review evidence

- The entire diff removes exactly two spaces from the expected function header. The input file, vector id, capability, applicability, disposition, coverage and results are unchanged.
- The unchanged input deliberately retains `using v () : Integer`, so the vector continues to exercise formatting rather than compare already-normalized input with itself.
- The formatter's `needs_space` attaches `(` after an identifier, emits no space before `:`, and retains one space after it. Its existing examples and compact parameter criterion agree with `using v(): Integer`.
- The independently authored expected value remains a complete output string, including its body, indentation and final newline. No byte comparison, schema rule or consumer assertion is relaxed.
- The fixture remains one JSON line with no final newline. The only edit is inside its string value; the surrounding canonical key order and JSON escapes are unchanged.
- The existing corpus test checks schema admission, adverse structural cases and equality with canonical JSON bytes. The existing publication test compares each published proposal file with the library accessor's bytes. Reading those tests does not establish that they passed.
- The standard's fixture is an illustrative corpus, not a complete coverage or qualification claim. Its `results` remains empty; no prior outcome is rebound to the changed oracle.
- No applicable AssuranceProfile is declared under the repository's spec tree.

## Method selection and validation

Applied the language-independent code-review, integrity, duplication, requirement-consistency and code-test-alignment checks to the single JSON fixture. There is no new copy, helper, executable implementation, Rust or manifest change. Rust review, implementation gap-analysis and a separate formal requirement review do not apply to this fixture-only diff.

`git diff --check` is clean. The review artifact validates with the installed SpecReview contract (exit 0); loading that module alone also reports unrelated missing-role/edge registry diagnostics. No repository builds, tests or qualification executables were run locally. Schema/publication tests alone would not demonstrate the consuming formatter's qualification; the three named consumer checks must follow the separate standard-pin update.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
