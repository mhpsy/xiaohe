# xiaohe CLI — Design Spec

- **Date**: 2026-05-25
- **Author**: mhpsy
- **Status**: Draft for implementation

## 1. Purpose

A small offline command-line tool for learning and querying the **Xiaohe Shuangpin (小鹤双拼)** input scheme. It maps Chinese pinyin syllables to their two-key Shuangpin codes and back, and prints the scheme tables for memorization.

The first version is a **rule-mapping query tool**, not an input method engine. It does not handle Chinese characters, word segmentation, candidate ranking, or text input.

## 2. Scope

### In scope (v0.1)

- Encode a single legal pinyin syllable → 2-character Shuangpin code.
- Decode a 2-character Shuangpin code → all matching pinyin syllables.
- Print the Shuangpin keyboard layout (initials + finals per key).
- Print the full list of legal syllables and their codes, filterable by prefix.
- Friendly "did-you-mean" suggestions for invalid input.
- Strict compliance with the official Xiaohe scheme, including:
  - `v` as substitute for `ü` (e.g. `nv` is treated as `nü`).
  - Zero-initial syllables (`a`, `ang`, `er`, `ou`, ...) use the final's first letter in the initial position.

### Out of scope (v0.1)

- Chinese character lookup (pinyin ↔ Hanzi).
- The "shape code" / 字形辅助码 portion of Xiaohe **音形** (this tool covers the 双拼 portion only).
- Multi-syllable encoding (e.g. `nihao` → `nihk`).
- Shell completion, man pages.
- JSON output.
- `crates.io` publishing.
- CI workflows.

## 3. Command Interface

```
xiaohe encode <SYLLABLE>      # alias: e   e.g.  xiaohe e xiao    → xk
xiaohe decode <CODE>          # alias: d   e.g.  xiaohe d xk      → xiao
xiaohe table                              # ASCII keyboard layout
xiaohe list                               # full syllable → shuangpin listing
xiaohe list --filter <PREFIX>             # e.g. --filter x   →   xi xia xian xiao ...
xiaohe help | --version
```

Exit codes:

- `0` — success.
- `1` — input is well-formed but semantically invalid (e.g. unknown syllable).
- `2` — argument parsing error (clap default).

Output:

- `encode` / `decode` print plain results on stdout (one per line, sorted), pipe-friendly.
- `table` prints the colored keyboard when stdout is a TTY, plain ASCII otherwise.
- `list` prints `<syllable>\t<code>` lines on stdout.
- All errors go to stderr.

## 4. Architecture

Single Cargo crate exposing both a library and a binary.

```
xiaohe/
├── Cargo.toml                   # edition = "2024"
├── README.md
├── LICENSE                      # MIT
├── .gitignore
├── src/
│   ├── main.rs                  # clap CLI; thin shell over lib
│   ├── lib.rs                   # public API re-exports
│   ├── tables.rs                # 'static initial/final/syllable tables
│   ├── encode.rs                # encode_syllable
│   ├── decode.rs                # decode_code
│   ├── keyboard.rs              # render_keyboard
│   └── suggest.rs               # nearest(input, k) for did-you-mean
└── tests/
    ├── golden.rs                # table-driven integration test
    └── fixtures/
        └── syllables.tsv        # ground truth: ~410 lines of <syllable>\t<code>
```

### Core types

```rust
// tables.rs
pub static INITIAL_MAP: &[(&str, char)] = &[
    ("zh", 'v'), ("ch", 'i'), ("sh", 'u'),
    // single-letter initials map to themselves: ("b", 'b'), ...
];
pub static FINAL_MAP: &[(&str, char)] = &[
    ("iao", 'n'), ("uang", 'l'), /* ... */
];
pub static SYLLABLES: &[&str] = &["a", "ai", /* ... */ "zuo"];

// encode.rs
#[derive(Debug)]
pub enum EncodeError {
    Empty,
    InvalidSyllable { input: String, suggestions: Vec<&'static str> },
}
pub fn encode_syllable(s: &str) -> Result<[char; 2], EncodeError>;

// decode.rs
pub fn decode_code(code: &str) -> Result<Vec<&'static str>, DecodeError>;

// suggest.rs
pub fn nearest(input: &str, k: usize) -> Vec<&'static str>;

// keyboard.rs
pub enum Style { Color, Plain }
pub fn render_keyboard(style: Style) -> String;
```

### Encoding algorithm

1. Lowercase and trim the input. Replace any `ü` with `v`.
2. If the input is not in `SYLLABLES`, return `InvalidSyllable` with up to 3 suggestions from `suggest::nearest`.
3. Otherwise split the syllable into `(initial, final)`:
   - If the syllable starts with one of the zero-initial finals (`a`, `ai`, `an`, `ang`, `ao`, `e`, `ei`, `en`, `eng`, `er`, `o`, `ou`) and has no preceding consonant, the initial-position key is the first letter of the final (per Xiaohe spec) and the final-position key is `FINAL_MAP[full_final]`. Example: `ang` → `a` (initial slot, the letter `a`) + `h` (the key for final `ang`) = `ah`.
   - Otherwise, take the longest matching prefix in `INITIAL_MAP` (handles `zh`/`ch`/`sh` and single-letter initials); the remainder is the final.
4. Look up both halves in their maps; return the two key characters.

The look-up-then-split order avoids ambiguous segmentation (e.g. `xian` is in `SYLLABLES`, so it is never split as `xi+an`).

### Decoding algorithm

Linear scan of `SYLLABLES` (~410 entries; fast enough that an index is unjustified for v0.1): collect every syllable whose `encode_syllable` result equals the requested code. Return sorted.

### Did-you-mean

`strsim::damerau_levenshtein` against all of `SYLLABLES`. Keep the `k` closest with distance ≤ 2; ties broken by lexicographic order.

## 5. Keyboard rendering

`xiaohe table` prints an ASCII layout where each key cell shows three lines:

1. Uppercase key letter.
2. Initial pinyin on that key (or `·` if none — every Latin letter is an initial except for the empty-initial slot, so this row is mostly populated).
3. Final pinyin on that key (or `·` if none).

Color (TTY only):

- Initial row: blue.
- Final row: green.
- `·` placeholders: dim gray.

A non-TTY stdout (pipe, redirect) automatically falls back to plain ASCII via `anstream`.

The exact mapping is the **official Xiaohe Shuangpin scheme**, to be encoded as `'static` data in `tables.rs` during implementation. Any discrepancy between the prototype layout in this spec and the official scheme is resolved in favor of the official scheme.

## 6. Error handling

`encode` failure example:

```
$ xiaohe encode xio
error: 'xio' is not a legal pinyin syllable
hint: did you mean 'xiao', 'xie', 'xi'?
```

- Errors are printed to stderr; stdout stays empty so pipes do not pick up garbage.
- Suggestions are omitted when none are within distance 2.
- Empty input or whitespace-only input is reported as `error: empty input`.

## 7. Testing

- **Ground truth**: `tests/fixtures/syllables.tsv`, ~410 lines of `<syllable>\t<code>` covering every legal Mandarin pinyin syllable. This file is the source of truth from which `SYLLABLES` is also derived (a build-time check or unit test asserts they agree).
- **Golden integration test** (`tests/golden.rs`): for each line, assert `encode_syllable(syllable) == code` and `decode_code(code).contains(syllable)`.
- **Edge-case unit tests** (in `src/encode.rs` under `#[cfg(test)]`):
  - `v` ↔ `ü`: `nv` (nü), `lve` (lüe), `nve` (nüe).
  - Zero-initial: `a`, `ang`, `er`, `ou`.
  - `xian` does not segment as `xi+an`.
  - `did-you-mean('xio')` includes `xiao`.
- **Snapshot tests**: `table` and `list` outputs compared via `assert_eq!(actual, include_str!("..."))`. No external snapshot library.

## 8. Dependencies

Minimal, all small:

- `clap` (derive feature) — argument parsing.
- `anstream` + `anstyle` — TTY-aware colored output, zero deps cascade.
- `strsim` — edit distance for did-you-mean.

Explicitly **not** included: `serde`, `serde_json`, `tokio`, `colored`, `insta`.

## 9. Project setup

- Rust edition 2024, `rust-version = "1.83"`.
- `cargo init` produces the binary; the library is added by introducing `src/lib.rs`.
- `LICENSE`: MIT.
- `.gitignore`: `/target`.
- `README.md`: usage examples for each subcommand, attribution to the Xiaohe scheme designers, license.

## 10. Delivery

1. Scaffold the crate, write `tables.rs` with the official mappings, populate `syllables.tsv` from a public Mandarin syllable list, and reconcile.
2. Implement `encode`, `decode`, `suggest`, `keyboard` in that order, each guarded by tests written first.
3. Wire `main.rs` with clap; verify each subcommand's output by hand.
4. `git init -b main` → commit scaffolding → commit each milestone.
5. `gh repo create xiaohe --public --source=. --remote=origin --push` (one-time, requires user confirmation).

## 11. Open questions / deferred to v0.2

- Multi-syllable encoding with longest-match segmentation.
- `pinyin --tone` parsing (e.g. accepting `xiǎo`).
- Hanzi lookup (`xiaohe encode 小` → `xk`) backed by a pinyin-data file.
- Shell completions, man page, GitHub Actions CI, crates.io publish.
