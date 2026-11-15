---
id: TC-1604
title: "TLA+ and Lean 4 output fidelity"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-297
    type: verifies
---

# TC-1604: TLA+ and Lean 4 output fidelity

## Description

Verify TLA+ 2 and Lean 4 export, profile selection, per-obligation fidelity and observer independence. Scope: FR-651-AC-1 to FR-651-AC-5 and FR-297-AC-5.

## Test Procedure

1. Export the counter model and `value <= 5` under each profile; run SANY and Lean as observers.
2. Export a timed clause, a hybrid claim, a 0.9 probabilistic threshold and a probability-1 safety claim.
3. Request a non-obligation identity; make an observer reject; export twice; select `tla` with the Lean profile.
4. Over the export conformance corpus, run TLC on each TLA+ export and FSL exploration on each model.

## Expected Results

- Step 1: the stated definitions with source comments, every obligation `preserved`; observers accept.
- Step 2: `conditional`, `unrepresented`, `unrepresented` and `conditional` as stated, none `preserved`.
- Step 3: the member refused; dispositions unchanged; byte-equal packages; cross-family refusal with no output.
- Step 4: equal reachable state counts and invariant verdicts.
