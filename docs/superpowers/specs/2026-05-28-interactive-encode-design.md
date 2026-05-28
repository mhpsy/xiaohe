# Interactive encode mode — design

Date: 2026-05-28

## Goal

Add an interactive subcommand so a user can encode many pinyin syllables in one
session without re-invoking the CLI each time.

## Scope (YAGNI)

- Encode-only. No decode, table, or list inside the loop.
- Single-syllable input per line, same rules as the existing `encode` command.

## CLI surface

New subcommand `interactive` with visible alias `i`:

```
xiaohe interactive
xiaohe i
```

## Behavior

- On start, print a one-line hint to **stderr** (keeps stdout clean for pipes):
  `xiaohe interactive — type a pinyin syllable, Ctrl-D or 'quit' to exit`
- Read stdin line by line:
  - Trim whitespace.
  - Empty line → skip, prompt again.
  - `quit` or `exit` → break, exit success.
  - Otherwise treat as a syllable and encode it:
    - Success → print the 2-key code (`xn`) to stdout.
    - Failure → print `error: ...` and any spelling suggestions to stderr,
      then continue (do not exit).
- EOF (Ctrl-D) → exit success.

## Implementation notes

- Extract the encode-and-print logic currently inline in `run_encode` into a
  helper `report_encode(input: &str) -> bool` (returns whether it succeeded).
  - `run_encode` calls it and maps the bool to an `ExitCode`.
  - `run_interactive` calls it in the loop and ignores the bool.
- No new dependencies. Use `std::io::stdin().lock().lines()`.

## Testing

- Integration test that feeds stdin (e.g. `xn\nzz\nquit\n`) and asserts:
  - valid syllable prints its code on stdout,
  - invalid input prints an error on stderr and the loop continues,
  - `quit` ends the session.
- Existing snapshot tests for one-shot `encode` remain unchanged.
