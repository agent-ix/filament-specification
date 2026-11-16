---
id: FR-710
title: "Cancel a running CLI verb on an interrupt or termination signal"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-300
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-522
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-853
    type: traces_to
---
# FR-710: Cancel a running CLI verb on an interrupt or termination signal

## Description

When the CLI process receives an interrupt or termination signal while a
verb runs, the CLI SHALL cancel the verb's library operation through its
cancellation handle (FR-300) with cause `requested`, let the operation return
its cancelled outcome, write that outcome as it writes any other, and exit
with its category's status (FR-301). A person who presses Ctrl-C on a long
analysis, or a supervisor that stops a monitor, gets a parseable incomplete
outcome that keeps every result already settled, not a killed process with
no output.

### Decision: the first signal cancels, a second ends the process

Cancellation stops an operation at its next charge, which is prompt but not
instantaneous. The first signal therefore asks for the cancelled outcome, and
the CLI then restores the operating system's default action for both
signals, so a second signal ends the process at once. An operator who has
asked twice wants the process gone, and the first outcome is the one a
script can rely on.

## Inputs

- On POSIX systems, `SIGINT` and `SIGTERM`.
- On Windows, the console control events `CTRL_C_EVENT`, `CTRL_BREAK_EVENT`
  and `CTRL_CLOSE_EVENT`.

## Outputs

The verb's cancelled outcome document (machine mode) or diagnostic line (text
mode), and its exit status.

## Behavior

- The CLI SHALL install its signal handling before it calls the verb's
  library operation, for every verb whose operation takes a cancellation
  handle.
- On the first signal, the CLI SHALL cancel the handle with cause
  `requested`, and SHALL NOT end the process itself. The operation returns
  its cancelled outcome: category incomplete, code `cancelled`, cause
  `requested`, exit `22`; a multi-item verb keeps each settled item's record
  and settles each open item `incomplete` with that cause (FR-300).
- A verb that reads its input from a stream, such as `monitor` over standard
  input, SHALL stop waiting for input once the handle is cancelled, so a read
  blocked on an open stream does not delay the outcome. The cancellation is
  not an end of input: the stream's open results are reported incomplete,
  never closed into a verdict.
- After the first signal, the CLI SHALL restore the default action of both
  signals. A second signal ends the process by that action; output already
  written stays, and any partial line is not a complete document.
- A handle already cancelled with cause `deadline` keeps that cause: the first
  cause to reach the handle is the one the outcome names.
- A signal that arrives after the outcome was written changes neither the
  output nor the exit status.
- A closed output pipe keeps FR-301's rule (the CLI ends quietly with the
  outcome's status); it is not a cancellation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-710-AC-1 | An `analyze` run of two items whose second runs until cancelled, sent `SIGINT` after the first settles, writes one complete outcome document whose first item keeps its record and whose second is `incomplete` with cause `requested`, and exits `22`. | Driver test (TC-1740): signal handling exists only in the CLI binary, so the driver code ticket adds the test in the driver repository. |
| FR-710-AC-2 | A `run` of a long-running function sent `SIGTERM` writes the incomplete outcome with code `cancelled` and cause `requested`, exit `22`. | Driver test (TC-1740) |
| FR-710-AC-3 | A `monitor` reading a standard input the test holds open, sent `SIGINT` after two records, writes its summary with the open clauses `incomplete` and cause `requested` within one second, exit `22`, without the input being closed. | Driver test (TC-1740) |
| FR-710-AC-4 | A second `SIGINT` sent to a verb whose operation has not yet reached its next charge ends the process by the signal's default action, and the exit status reports that signal. | Driver test (TC-1740) |
| FR-710-AC-5 | A run whose deadline has already cancelled its handle, sent `SIGINT`, reports cause `deadline`; a signal sent after the outcome is written leaves the output and the exit status unchanged. | Driver test (TC-1740) |

## Dependencies

- FR-300 owns the cancellation handle and its causes; FR-301 owns the exit
  statuses and stream rules.
