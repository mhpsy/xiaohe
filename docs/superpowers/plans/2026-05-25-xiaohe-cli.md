# xiaohe CLI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship a Rust CLI that encodes/decodes pinyin syllables to Xiaohe Shuangpin (小鹤双拼) codes, renders the keyboard layout, and lists the full scheme — with friendly errors and a golden table-driven test.

**Architecture:** Single Cargo crate (lib + bin). `tables.rs` holds three `'static` tables (initial map, final map, syllables); `encode.rs` / `decode.rs` / `suggest.rs` / `keyboard.rs` are small pure-function modules; `main.rs` is a thin clap shell. Ground-truth golden TSV in `tests/fixtures/syllables.tsv` drives the integration test.

**Tech Stack:** Rust (edition 2024), `clap` (derive), `anstream` + `anstyle`, `strsim`.

---

## Reference: Xiaohe Shuangpin Scheme

The plan hard-codes the official scheme below. Before encoding tables in Rust (Task 3), cross-check against an authoritative source — recommended: https://flypy.com/ (creator's site) or the Rime schema https://github.com/rime/rime-flypy (search for `flypy.schema.yaml`). If any cell below contradicts the official scheme, the official scheme wins.

**Initial map (only non-trivial entries; all other single-letter initials map to themselves):**

| pinyin initial | key |
|---|---|
| zh | v |
| ch | i |
| sh | u |

**Final map (35 finals on 26 keys; some keys carry two finals — disambiguated by which initials they combine with):**

| key | finals |
|---|---|
| q | iu |
| w | ei |
| e | e |
| r | uan, er |
| t | ue, ve |
| y | un |
| u | u |
| i | i |
| o | uo, o |
| p | ie |
| a | a |
| s | iong, ong |
| d | ai |
| f | en |
| g | eng, ng |
| h | ang |
| j | an |
| k | ing, uai |
| l | iang, uang |
| z | ou |
| x | ia, ua |
| c | ao |
| v | v, ui |
| b | in |
| n | iao |
| m | ian |

**Zero-initial syllables** (those whose pinyin spelling has no consonant: `a, ai, an, ang, ao, e, ei, en, eng, er, o, ou`): the initial-position key is the FIRST LETTER of the syllable; the final-position key is the normal `FINAL_MAP` lookup. Example: `ang` → key `a` (initial slot) + key `h` (`ang` final) = `ah`.

**`v` substitutes for `ü`** at the input level (`nv` is encoded as if it were `nü`).

---

## Task 1: Cargo project scaffolding

**Files:**
- Create: `/home/mhpsy/apps/xiaohe/Cargo.toml`
- Create: `/home/mhpsy/apps/xiaohe/LICENSE`
- Create: `/home/mhpsy/apps/xiaohe/README.md`
- Create: `/home/mhpsy/apps/xiaohe/src/main.rs`
- Create: `/home/mhpsy/apps/xiaohe/src/lib.rs`

- [ ] **Step 1.1: Write `Cargo.toml`**

```toml
[package]
name = "xiaohe"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
description = "CLI for querying the Xiaohe Shuangpin (小鹤双拼) scheme"
license = "MIT"
repository = "https://github.com/mhpsy/xiaohe"
readme = "README.md"

[dependencies]
clap = { version = "4", features = ["derive"] }
anstream = "0.6"
anstyle = "1"
strsim = "0.11"

[lib]
name = "xiaohe"
path = "src/lib.rs"

[[bin]]
name = "xiaohe"
path = "src/main.rs"
```

- [ ] **Step 1.2: Write `LICENSE` (MIT)**

```
MIT License

Copyright (c) 2026 mhpsy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 1.3: Write minimal `README.md` stub**

```markdown
# xiaohe

CLI for querying the Xiaohe Shuangpin (小鹤双拼) scheme.

Usage and examples will be added once the implementation is complete.
```

- [ ] **Step 1.4: Write placeholder `src/lib.rs`**

```rust
pub mod tables;
pub mod encode;
pub mod decode;
pub mod suggest;
pub mod keyboard;
```

- [ ] **Step 1.5: Write placeholder `src/main.rs`**

```rust
fn main() {
    eprintln!("xiaohe: not yet wired up");
    std::process::exit(2);
}
```

- [ ] **Step 1.6: Create empty module files so `lib.rs` compiles**

Create each of these as an empty file (zero bytes):
- `src/tables.rs`
- `src/encode.rs`
- `src/decode.rs`
- `src/suggest.rs`
- `src/keyboard.rs`

- [ ] **Step 1.7: Verify it builds**

Run: `cargo build`
Expected: clean build, only warnings about unused empty modules.

- [ ] **Step 1.8: Commit**

```bash
git add Cargo.toml Cargo.lock LICENSE README.md src/
git commit -m "chore: scaffold xiaohe crate"
```

---

## Task 2: Initial/final tables in `tables.rs`

**Files:**
- Modify: `src/tables.rs`

- [ ] **Step 2.1: Write the failing test for the initial table**

Replace `src/tables.rs` with the test scaffolding first, plus an empty implementation:

```rust
/// Pinyin initials that map to a non-self key. All other single-letter
/// initials (b/p/m/f/d/t/n/l/g/k/h/j/q/x/r/z/c/s/y/w) map to themselves
/// and are NOT listed here — `initial_key()` returns `Some(c)` for them
/// by falling back to the first character.
pub static MULTI_LETTER_INITIALS: &[(&str, char)] = &[];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_letter_initials_cover_zh_ch_sh() {
        let pairs: std::collections::HashMap<&str, char> =
            MULTI_LETTER_INITIALS.iter().copied().collect();
        assert_eq!(pairs.get("zh"), Some(&'v'));
        assert_eq!(pairs.get("ch"), Some(&'i'));
        assert_eq!(pairs.get("sh"), Some(&'u'));
    }
}
```

- [ ] **Step 2.2: Run test to verify it fails**

Run: `cargo test --lib tables::tests::multi_letter_initials_cover_zh_ch_sh`
Expected: FAIL — `MULTI_LETTER_INITIALS` is empty, lookups return `None`.

- [ ] **Step 2.3: Fill in `MULTI_LETTER_INITIALS`**

Replace the empty `&[]` with:

```rust
pub static MULTI_LETTER_INITIALS: &[(&str, char)] = &[
    ("zh", 'v'),
    ("ch", 'i'),
    ("sh", 'u'),
];
```

- [ ] **Step 2.4: Run test to verify it passes**

Run: `cargo test --lib tables::tests::multi_letter_initials_cover_zh_ch_sh`
Expected: PASS.

- [ ] **Step 2.5: Write the failing test for the final table**

Add to the `tests` mod in `src/tables.rs`:

```rust
#[test]
fn final_map_has_expected_mappings() {
    let pairs: std::collections::HashMap<&str, char> =
        FINAL_MAP.iter().copied().collect();
    // Spot-check a handful from the spec; full coverage comes from the
    // golden TSV integration test.
    assert_eq!(pairs.get("iao"), Some(&'n'));
    assert_eq!(pairs.get("uang"), Some(&'l'));
    assert_eq!(pairs.get("ang"), Some(&'h'));
    assert_eq!(pairs.get("a"), Some(&'a'));
    assert_eq!(pairs.get("ve"), Some(&'t'));
    assert_eq!(pairs.get("ui"), Some(&'v'));
    assert_eq!(pairs.get("ing"), Some(&'k'));
}

#[test]
fn final_map_keys_are_unique() {
    let mut seen = std::collections::HashSet::new();
    for (k, _) in FINAL_MAP {
        assert!(seen.insert(*k), "duplicate final entry: {k}");
    }
}
```

Above the tests mod (still in `src/tables.rs`), declare an empty placeholder:

```rust
pub static FINAL_MAP: &[(&str, char)] = &[];
```

- [ ] **Step 2.6: Run tests to verify they fail**

Run: `cargo test --lib tables`
Expected: the two new tests FAIL.

- [ ] **Step 2.7: Populate `FINAL_MAP`**

Replace the empty `FINAL_MAP` with the full table from the Reference section above:

```rust
pub static FINAL_MAP: &[(&str, char)] = &[
    ("iu", 'q'),
    ("ei", 'w'),
    ("e", 'e'),
    ("uan", 'r'), ("er", 'r'),
    ("ue", 't'), ("ve", 't'),
    ("un", 'y'),
    ("u", 'u'),
    ("i", 'i'),
    ("uo", 'o'), ("o", 'o'),
    ("ie", 'p'),
    ("a", 'a'),
    ("iong", 's'), ("ong", 's'),
    ("ai", 'd'),
    ("en", 'f'),
    ("eng", 'g'), ("ng", 'g'),
    ("ang", 'h'),
    ("an", 'j'),
    ("ing", 'k'), ("uai", 'k'),
    ("iang", 'l'), ("uang", 'l'),
    ("ou", 'z'),
    ("ia", 'x'), ("ua", 'x'),
    ("ao", 'c'),
    ("v", 'v'), ("ui", 'v'),
    ("in", 'b'),
    ("iao", 'n'),
    ("ian", 'm'),
];
```

- [ ] **Step 2.8: Add the zero-initial finals list and an accessor**

Append to `src/tables.rs` (above the `tests` mod):

```rust
/// Finals whose syllables have no initial consonant in pinyin spelling.
/// For these, the initial-position key is the FIRST LETTER of the syllable.
pub static ZERO_INITIAL_FINALS: &[&str] = &[
    "a", "ai", "an", "ang", "ao",
    "e", "ei", "en", "eng", "er",
    "o", "ou",
];

/// Look up the key for a multi-letter initial (zh/ch/sh).
/// Returns `None` for any other input — single-letter initials map to themselves
/// and are handled by the caller.
pub fn lookup_initial(s: &str) -> Option<char> {
    MULTI_LETTER_INITIALS
        .iter()
        .find_map(|(k, v)| if *k == s { Some(*v) } else { None })
}

/// Look up the key for a final.
pub fn lookup_final(s: &str) -> Option<char> {
    FINAL_MAP
        .iter()
        .find_map(|(k, v)| if *k == s { Some(*v) } else { None })
}
```

- [ ] **Step 2.9: Run all tests**

Run: `cargo test --lib`
Expected: 3 tests PASS.

- [ ] **Step 2.10: Commit**

```bash
git add src/tables.rs
git commit -m "feat: add Xiaohe Shuangpin initial and final tables"
```

---

## Task 3: `encode_syllable` (TDD)

**Files:**
- Modify: `src/encode.rs`

- [ ] **Step 3.1: Write the failing test for the simplest case**

Write `src/encode.rs`:

```rust
use crate::tables::{lookup_final, lookup_initial, ZERO_INITIAL_FINALS};

#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    Empty,
    InvalidSyllable {
        input: String,
        suggestions: Vec<&'static str>,
    },
}

pub fn encode_syllable(_input: &str) -> Result<[char; 2], EncodeError> {
    Err(EncodeError::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_xiao() {
        assert_eq!(encode_syllable("xiao"), Ok(['x', 'n']));
    }
}
```

- [ ] **Step 3.2: Run test to verify it fails**

Run: `cargo test --lib encode::tests::encodes_xiao`
Expected: FAIL — returns `Empty`.

- [ ] **Step 3.3: Implement the happy path (single-letter initial + final)**

Replace `encode_syllable` with:

```rust
pub fn encode_syllable(input: &str) -> Result<[char; 2], EncodeError> {
    let normalized = input.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(EncodeError::Empty);
    }

    // 1) Two-letter initial first (zh/ch/sh).
    if normalized.len() >= 3 {
        let head = &normalized[..2];
        if let Some(initial_key) = lookup_initial(head) {
            let tail = &normalized[2..];
            if let Some(final_key) = lookup_final(tail) {
                return Ok([initial_key, final_key]);
            }
        }
    }

    // 2) Single-letter initial + final.
    if normalized.len() >= 2 {
        let head = normalized.as_bytes()[0] as char;
        let tail = &normalized[1..];
        if let Some(final_key) = lookup_final(tail) {
            return Ok([head, final_key]);
        }
    }

    Err(EncodeError::InvalidSyllable {
        input: normalized,
        suggestions: Vec::new(),
    })
}
```

- [ ] **Step 3.4: Run test to verify it passes**

Run: `cargo test --lib encode::tests::encodes_xiao`
Expected: PASS.

- [ ] **Step 3.5: Add the zh/ch/sh test**

Append to the `tests` mod:

```rust
#[test]
fn encodes_zh_ch_sh_initials() {
    assert_eq!(encode_syllable("zhong"), Ok(['v', 's'])); // zh + ong
    assert_eq!(encode_syllable("chuang"), Ok(['i', 'l'])); // ch + uang
    assert_eq!(encode_syllable("shuai"), Ok(['u', 'k'])); // sh + uai
}
```

- [ ] **Step 3.6: Run tests to verify they pass (impl already handles them)**

Run: `cargo test --lib encode`
Expected: PASS.

- [ ] **Step 3.7: Write the failing zero-initial test**

Append:

```rust
#[test]
fn encodes_zero_initial_syllables() {
    assert_eq!(encode_syllable("ang"), Ok(['a', 'h'])); // a + ang
    assert_eq!(encode_syllable("ai"), Ok(['a', 'd']));   // a + ai
    assert_eq!(encode_syllable("er"), Ok(['e', 'r']));   // e + er
    assert_eq!(encode_syllable("ou"), Ok(['o', 'z']));   // o + ou
    assert_eq!(encode_syllable("a"), Ok(['a', 'a']));    // a + a
    assert_eq!(encode_syllable("e"), Ok(['e', 'e']));    // e + e
}
```

- [ ] **Step 3.8: Run tests to verify the new ones fail**

Run: `cargo test --lib encode::tests::encodes_zero_initial_syllables`
Expected: FAIL on `ang` / `ai` etc.

- [ ] **Step 3.9: Add the zero-initial branch**

Insert at the top of the `encode_syllable` body, AFTER the `normalized` / empty check but BEFORE the two-letter initial lookup:

```rust
    // Zero-initial: the syllable IS its own final.
    if ZERO_INITIAL_FINALS.contains(&normalized.as_str()) {
        let initial_key = normalized.as_bytes()[0] as char;
        let final_key = lookup_final(&normalized).expect("zero-initial final in FINAL_MAP");
        return Ok([initial_key, final_key]);
    }
```

- [ ] **Step 3.10: Run tests to verify they pass**

Run: `cargo test --lib encode`
Expected: all PASS.

- [ ] **Step 3.11: Write the `v=ü` test**

Append:

```rust
#[test]
fn encodes_v_as_u_umlaut() {
    assert_eq!(encode_syllable("nv"), Ok(['n', 'v']));   // nü
    assert_eq!(encode_syllable("lve"), Ok(['l', 't']));  // lüe (ve = üe → t)
    assert_eq!(encode_syllable("nve"), Ok(['n', 't']));  // nüe
}
```

- [ ] **Step 3.12: Run test to verify it passes**

Run: `cargo test --lib encode::tests::encodes_v_as_u_umlaut`
Expected: PASS — `FINAL_MAP` already includes `("v", 'v')` and `("ve", 't')`, so the existing logic handles these.

- [ ] **Step 3.13: Write the invalid-input test**

Append:

```rust
#[test]
fn returns_empty_error_for_blank_input() {
    assert_eq!(encode_syllable(""), Err(EncodeError::Empty));
    assert_eq!(encode_syllable("   "), Err(EncodeError::Empty));
}

#[test]
fn returns_invalid_syllable_for_garbage() {
    match encode_syllable("zzz") {
        Err(EncodeError::InvalidSyllable { input, .. }) => assert_eq!(input, "zzz"),
        other => panic!("unexpected: {other:?}"),
    }
}
```

- [ ] **Step 3.14: Run all encode tests**

Run: `cargo test --lib encode`
Expected: all PASS.

- [ ] **Step 3.15: Commit**

```bash
git add src/encode.rs
git commit -m "feat: encode pinyin syllable to Xiaohe Shuangpin"
```

---

## Task 4: `decode_code` (TDD)

**Files:**
- Modify: `src/tables.rs` (add `SYLLABLES` placeholder)
- Modify: `src/decode.rs`

- [ ] **Step 4.1: Add a minimal `SYLLABLES` placeholder to `tables.rs`**

Append to `src/tables.rs` (above the `tests` mod):

```rust
/// Sorted list of all legal Mandarin pinyin syllables (ASCII, with `v` for ü).
/// Populated fully in Task 9 from the golden TSV; the entries below are a
/// representative subset used by unit tests in earlier tasks.
pub static SYLLABLES: &[&str] = &[
    "a", "ai", "an", "ang", "ao",
    "ba", "chuang", "e", "er", "lve", "nv", "ou",
    "shuai", "xiao", "xie", "xian", "yi", "ying", "zhong",
];
```

- [ ] **Step 4.2: Write the failing test**

Replace `src/decode.rs`:

```rust
use crate::encode::encode_syllable;
use crate::tables::SYLLABLES;

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InvalidLength { input: String },
}

pub fn decode_code(_code: &str) -> Result<Vec<&'static str>, DecodeError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_xn_to_xiao() {
        let out = decode_code("xn").unwrap();
        assert!(out.contains(&"xiao"), "expected xiao in {out:?}");
    }
}
```

- [ ] **Step 4.3: Run test to verify it fails**

Run: `cargo test --lib decode::tests::decodes_xn_to_xiao`
Expected: FAIL — empty Vec.

- [ ] **Step 4.4: Implement `decode_code`**

Replace the placeholder body:

```rust
pub fn decode_code(code: &str) -> Result<Vec<&'static str>, DecodeError> {
    let normalized = code.trim().to_ascii_lowercase();
    if normalized.chars().count() != 2 || !normalized.chars().all(|c| c.is_ascii_lowercase()) {
        return Err(DecodeError::InvalidLength { input: normalized });
    }
    let target: [char; 2] = {
        let mut it = normalized.chars();
        [it.next().unwrap(), it.next().unwrap()]
    };

    let mut matches: Vec<&'static str> = SYLLABLES
        .iter()
        .copied()
        .filter(|s| encode_syllable(s).map(|k| k == target).unwrap_or(false))
        .collect();
    matches.sort_unstable();
    Ok(matches)
}
```

- [ ] **Step 4.5: Run test to verify it passes**

Run: `cargo test --lib decode::tests::decodes_xn_to_xiao`
Expected: PASS.

- [ ] **Step 4.6: Add invalid-length test**

Append to the `tests` mod:

```rust
#[test]
fn rejects_non_two_char_input() {
    assert!(matches!(decode_code("x"), Err(DecodeError::InvalidLength { .. })));
    assert!(matches!(decode_code("xyz"), Err(DecodeError::InvalidLength { .. })));
    assert!(matches!(decode_code("x1"), Err(DecodeError::InvalidLength { .. })));
}
```

- [ ] **Step 4.7: Run tests**

Run: `cargo test --lib decode`
Expected: PASS.

- [ ] **Step 4.8: Commit**

```bash
git add src/decode.rs src/tables.rs
git commit -m "feat: decode Xiaohe Shuangpin code to candidate syllables"
```

---

## Task 5: `suggest::nearest` (TDD)

**Files:**
- Modify: `src/suggest.rs`

- [ ] **Step 5.1: Write the failing test**

Write `src/suggest.rs`:

```rust
use crate::tables::SYLLABLES;
use strsim::damerau_levenshtein;

pub fn nearest(_input: &str, _k: usize) -> Vec<&'static str> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_xiao_for_xio() {
        let out = nearest("xio", 3);
        assert!(out.contains(&"xiao"), "expected xiao in {out:?}");
    }
}
```

- [ ] **Step 5.2: Run test to verify it fails**

Run: `cargo test --lib suggest::tests::suggests_xiao_for_xio`
Expected: FAIL.

- [ ] **Step 5.3: Implement `nearest`**

Replace the body:

```rust
pub fn nearest(input: &str, k: usize) -> Vec<&'static str> {
    let input = input.trim().to_ascii_lowercase();
    let mut ranked: Vec<(usize, &'static str)> = SYLLABLES
        .iter()
        .copied()
        .map(|s| (damerau_levenshtein(&input, s), s))
        .filter(|(d, _)| *d <= 2)
        .collect();
    // Sort by distance ascending, then lexicographic.
    ranked.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)));
    ranked.into_iter().take(k).map(|(_, s)| s).collect()
}
```

- [ ] **Step 5.4: Run test to verify it passes**

Run: `cargo test --lib suggest`
Expected: PASS.

- [ ] **Step 5.5: Add edge-case tests**

Append:

```rust
#[test]
fn returns_empty_for_completely_unrelated_input() {
    let out = nearest("qqqqqqq", 3);
    assert!(out.is_empty(), "expected empty, got {out:?}");
}

#[test]
fn caps_results_at_k() {
    let out = nearest("a", 2);
    assert!(out.len() <= 2);
}
```

- [ ] **Step 5.6: Run tests**

Run: `cargo test --lib suggest`
Expected: all PASS.

- [ ] **Step 5.7: Commit**

```bash
git add src/suggest.rs
git commit -m "feat: did-you-mean suggestions via Damerau-Levenshtein"
```

---

## Task 6: Wire suggestions into `EncodeError::InvalidSyllable`

**Files:**
- Modify: `src/encode.rs`

- [ ] **Step 6.1: Write the failing test**

Append to the `tests` mod in `src/encode.rs`:

```rust
#[test]
fn invalid_syllable_includes_suggestions() {
    let err = encode_syllable("xio").unwrap_err();
    match err {
        EncodeError::InvalidSyllable { input, suggestions } => {
            assert_eq!(input, "xio");
            assert!(
                suggestions.contains(&"xiao"),
                "expected xiao in suggestions: {suggestions:?}"
            );
            assert!(suggestions.len() <= 3);
        }
        other => panic!("unexpected: {other:?}"),
    }
}
```

- [ ] **Step 6.2: Run test to verify it fails**

Run: `cargo test --lib encode::tests::invalid_syllable_includes_suggestions`
Expected: FAIL — `suggestions` is currently empty.

- [ ] **Step 6.3: Populate suggestions**

In `src/encode.rs`, replace the final `Err(EncodeError::InvalidSyllable { ... })` line of `encode_syllable` with:

```rust
    Err(EncodeError::InvalidSyllable {
        input: normalized.clone(),
        suggestions: crate::suggest::nearest(&normalized, 3),
    })
```

- [ ] **Step 6.4: Run test to verify it passes**

Run: `cargo test --lib encode`
Expected: all PASS.

- [ ] **Step 6.5: Commit**

```bash
git add src/encode.rs
git commit -m "feat: populate did-you-mean suggestions in encode errors"
```

---

## Task 7: `render_keyboard` in `keyboard.rs`

**Files:**
- Modify: `src/keyboard.rs`

- [ ] **Step 7.1: Write the failing test for plain rendering**

Write `src/keyboard.rs`:

```rust
use crate::tables::{FINAL_MAP, MULTI_LETTER_INITIALS};

#[derive(Copy, Clone, Debug)]
pub enum Style {
    Color,
    Plain,
}

pub fn render_keyboard(_style: Style) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_output_contains_all_letters() {
        let out = render_keyboard(Style::Plain);
        for c in 'a'..='z' {
            assert!(
                out.contains(c.to_ascii_uppercase()),
                "missing key letter {c} in:\n{out}"
            );
        }
    }

    #[test]
    fn plain_output_contains_initial_examples() {
        let out = render_keyboard(Style::Plain);
        assert!(out.contains("zh"), "missing initial 'zh':\n{out}");
        assert!(out.contains("ch"), "missing initial 'ch':\n{out}");
        assert!(out.contains("sh"), "missing initial 'sh':\n{out}");
    }

    #[test]
    fn plain_output_contains_final_examples() {
        let out = render_keyboard(Style::Plain);
        assert!(out.contains("iao"), "missing final 'iao':\n{out}");
        assert!(out.contains("ang"), "missing final 'ang':\n{out}");
        assert!(out.contains("uang"), "missing final 'uang':\n{out}");
    }
}
```

- [ ] **Step 7.2: Run tests to verify they fail**

Run: `cargo test --lib keyboard`
Expected: FAIL — output is empty.

- [ ] **Step 7.3: Implement `render_keyboard`**

Replace the body with a row-by-row builder. Each cell has 3 lines (key letter / initial pinyin / final pinyin); cells join with `│` separators.

```rust
const ROW_1: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const ROW_2: &[char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
const ROW_3: &[char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];

const CELL_WIDTH: usize = 5;

fn initial_for(key: char) -> String {
    // zh/ch/sh live on v/i/u; single-letter initials map to themselves
    // for the standard pinyin initials.
    for (pin, k) in MULTI_LETTER_INITIALS {
        if *k == key {
            return (*pin).to_string();
        }
    }
    let pinyin_initials = "bpmfdtnlgkhjqxrzcsyw";
    if pinyin_initials.contains(key) {
        key.to_string()
    } else {
        "·".to_string()
    }
}

fn finals_for(key: char) -> String {
    let mut parts: Vec<&str> = FINAL_MAP
        .iter()
        .filter_map(|(f, k)| if *k == key { Some(*f) } else { None })
        .collect();
    parts.sort_unstable();
    if parts.is_empty() {
        "·".to_string()
    } else {
        parts.join(",")
    }
}

fn pad(s: &str, w: usize) -> String {
    // pad on both sides for visual centering
    let visual = s.chars().count();
    if visual >= w {
        s.to_string()
    } else {
        let total = w - visual;
        let left = total / 2;
        let right = total - left;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
    }
}

fn render_row(keys: &[char], indent: usize) -> String {
    let prefix = " ".repeat(indent);
    let key_line = keys
        .iter()
        .map(|c| pad(&c.to_ascii_uppercase().to_string(), CELL_WIDTH))
        .collect::<Vec<_>>()
        .join("│");
    let initial_line = keys
        .iter()
        .map(|c| pad(&initial_for(*c), CELL_WIDTH))
        .collect::<Vec<_>>()
        .join("│");
    let final_line = keys
        .iter()
        .map(|c| pad(&finals_for(*c), CELL_WIDTH))
        .collect::<Vec<_>>()
        .join("│");
    format!(
        "{p}│{k}│\n{p}│{i}│\n{p}│{f}│\n",
        p = prefix,
        k = key_line,
        i = initial_line,
        f = final_line,
    )
}

pub fn render_keyboard(_style: Style) -> String {
    let mut out = String::new();
    out.push_str(&render_row(ROW_1, 0));
    out.push_str(&render_row(ROW_2, 3));
    out.push_str(&render_row(ROW_3, 6));
    out
}
```

- [ ] **Step 7.4: Run tests to verify they pass**

Run: `cargo test --lib keyboard`
Expected: all PASS.

- [ ] **Step 7.5: Add color support**

Replace the implementations of `initial_for`, `finals_for`, `render_row`, and `render_keyboard` so that the `Style::Color` path wraps each non-`·` initial in blue and each non-`·` final in green. `Style::Plain` leaves the cells untouched.

First add the imports at the top of the file:

```rust
use anstyle::{AnsiColor, Reset, Style as Ansi};
```

Add two const styles below the existing constants:

```rust
const INITIAL_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Blue)));
const FINAL_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
const DIM_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::BrightBlack)));
```

Change `render_row` to take the style and apply it AFTER padding (so width alignment is unaffected):

```rust
fn render_row(keys: &[char], indent: usize, style: Style) -> String {
    let prefix = " ".repeat(indent);
    let key_line = keys
        .iter()
        .map(|c| pad(&c.to_ascii_uppercase().to_string(), CELL_WIDTH))
        .collect::<Vec<_>>()
        .join("│");
    let initial_line = keys
        .iter()
        .map(|c| {
            let raw = initial_for(*c);
            let padded = pad(&raw, CELL_WIDTH);
            colorize(&padded, &raw, INITIAL_STYLE, style)
        })
        .collect::<Vec<_>>()
        .join("│");
    let final_line = keys
        .iter()
        .map(|c| {
            let raw = finals_for(*c);
            let padded = pad(&raw, CELL_WIDTH);
            colorize(&padded, &raw, FINAL_STYLE, style)
        })
        .collect::<Vec<_>>()
        .join("│");
    format!(
        "{p}│{k}│\n{p}│{i}│\n{p}│{f}│\n",
        p = prefix,
        k = key_line,
        i = initial_line,
        f = final_line,
    )
}

fn colorize(padded: &str, raw: &str, color: Ansi, style: Style) -> String {
    match style {
        Style::Plain => padded.to_string(),
        Style::Color => {
            let active = if raw == "·" { DIM_STYLE } else { color };
            format!("{}{}{}", active.render(), padded, Reset.render())
        }
    }
}
```

And change `render_keyboard` to thread `style` through:

```rust
pub fn render_keyboard(style: Style) -> String {
    let mut out = String::new();
    out.push_str(&render_row(ROW_1, 0, style));
    out.push_str(&render_row(ROW_2, 3, style));
    out.push_str(&render_row(ROW_3, 6, style));
    out
}
```

- [ ] **Step 7.6: Add a test that color path emits ANSI escapes**

Append to the `tests` mod:

```rust
#[test]
fn color_style_emits_ansi_escapes() {
    let out = render_keyboard(Style::Color);
    assert!(out.contains("\x1b["), "expected ANSI escape codes in colored output");
}

#[test]
fn plain_style_emits_no_ansi_escapes() {
    let out = render_keyboard(Style::Plain);
    assert!(!out.contains("\x1b["), "plain output must not contain escape codes");
}
```

- [ ] **Step 7.7: Run all keyboard tests**

Run: `cargo test --lib keyboard`
Expected: all PASS (existing `contains` tests still match raw substrings because color codes are wrappers, not insertions inside the text).

- [ ] **Step 7.8: Commit**

```bash
git add src/keyboard.rs
git commit -m "feat: render Xiaohe keyboard layout with optional color"
```

---

## Task 8: Populate the golden TSV fixture

**Files:**
- Create: `tests/fixtures/syllables.tsv`

This task replaces the placeholder `SYLLABLES` in `tables.rs` with the full set. The TSV is the source of truth.

- [ ] **Step 8.1: Source the canonical pinyin syllable list**

The list of legal Mandarin pinyin syllables (including erhua-free standard ones, plus `nü`/`lü` as `nv`/`lv`) is fixed at ~410 entries. Pull from one of:
- https://en.wikipedia.org/wiki/Pinyin_table (canonical reference)
- https://github.com/rime/rime-flypy (`flypy.schema.yaml` + companion dict)
- https://github.com/mozillazg/python-pinyin/blob/master/pinyin_dict/pinyin_dict.py

Manual curation note: substitute `ü` → `v` and `üe` → `ve` everywhere. Drop tone marks. Drop syllables with only erhua (`r`-suffix) variants.

- [ ] **Step 8.2: Create `tests/fixtures/syllables.tsv`**

Write the file with one `<syllable>\t<code>` line per syllable, sorted lexicographically by syllable. The `<code>` column will be filled in Step 8.3 by a one-off helper.

Start the file with the curated syllable list, one per line, with a placeholder tab and `??` in the second column:

```
a	??
ai	??
an	??
ang	??
ao	??
ba	??
...
zuo	??
```

- [ ] **Step 8.3: Fill in the codes mechanically**

Write a throwaway binary `examples/fill_tsv.rs` that reads the TSV, runs `encode_syllable` on each first column, and writes back the result in the second column:

```rust
// examples/fill_tsv.rs
use std::fs;
use std::io::Write;
use xiaohe::encode::encode_syllable;

fn main() {
    let path = "tests/fixtures/syllables.tsv";
    let src = fs::read_to_string(path).expect("read tsv");
    let mut out = String::new();
    for line in src.lines() {
        let syllable = line.split('\t').next().unwrap().trim();
        if syllable.is_empty() { continue; }
        let code = encode_syllable(syllable)
            .unwrap_or_else(|e| panic!("encode {syllable}: {e:?}"));
        writeln!(out, "{syllable}\t{}{}", code[0], code[1]).unwrap();
    }
    fs::write(path, out).expect("write tsv");
}
```

Run: `cargo run --example fill_tsv`
Expected: TSV is filled in place. If `encode_syllable` panics on any entry, that syllable either is misspelled in the TSV or exposes a bug in `tables.rs`. Fix and rerun.

- [ ] **Step 8.4: Spot-check the TSV**

Eyeball a handful of well-known entries against an external Xiaohe Shuangpin learner reference (e.g. https://flypy.com/):

```
xiao    xn
zhong   vs
shuang  ul
nv      nv
lve     lt
ang     ah
ying    yk
yuan    yr
```

If any disagree, the bug is in `tables.rs` — fix and rerun `cargo run --example fill_tsv`.

- [ ] **Step 8.5: Replace placeholder `SYLLABLES` in `tables.rs` with the full list**

Rewrite the `SYLLABLES` declaration in `src/tables.rs` so it includes EVERY syllable from the TSV (first column), in sorted order. Since this is mechanical, generate the array body by running:

```bash
awk -F'\t' '{ printf "    \"%s\",\n", $1 }' tests/fixtures/syllables.tsv
```

Paste the output into:

```rust
pub static SYLLABLES: &[&str] = &[
    // ... pasted lines ...
];
```

- [ ] **Step 8.6: Add a consistency test**

Append to `src/tables.rs`'s `tests` mod:

```rust
#[test]
fn syllables_are_sorted_and_unique() {
    for w in SYLLABLES.windows(2) {
        assert!(w[0] < w[1], "out of order or duplicate: {} >= {}", w[0], w[1]);
    }
}
```

- [ ] **Step 8.7: Run all tests**

Run: `cargo test --lib`
Expected: PASS.

- [ ] **Step 8.8: Delete the throwaway example**

```bash
rm examples/fill_tsv.rs
rmdir examples 2>/dev/null || true
```

- [ ] **Step 8.9: Commit**

```bash
git add tests/fixtures/syllables.tsv src/tables.rs
git commit -m "feat: populate full syllable table and golden TSV fixture"
```

---

## Task 9: Golden integration test

**Files:**
- Create: `tests/golden.rs`

- [ ] **Step 9.1: Write the golden test**

Create `tests/golden.rs`:

```rust
use std::fs;

use xiaohe::decode::decode_code;
use xiaohe::encode::encode_syllable;

#[test]
fn golden_tsv_matches_encode_and_decode() {
    let raw = fs::read_to_string("tests/fixtures/syllables.tsv")
        .expect("read syllables.tsv");

    let mut count = 0usize;
    for (lineno, line) in raw.lines().enumerate() {
        let lineno = lineno + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('\t');
        let syllable = parts.next().expect("syllable column");
        let code = parts.next().unwrap_or_else(|| panic!(
            "line {lineno}: missing code for {syllable}"
        ));
        assert_eq!(code.len(), 2, "line {lineno}: code must be 2 chars");

        // Forward: encode_syllable
        let encoded = encode_syllable(syllable)
            .unwrap_or_else(|e| panic!("line {lineno}: encode {syllable} failed: {e:?}"));
        let actual: String = encoded.iter().collect();
        assert_eq!(
            actual, code,
            "line {lineno}: encode({syllable}) -> {actual}, expected {code}"
        );

        // Reverse: decode_code contains the syllable
        let decoded = decode_code(code).expect("decode_code");
        assert!(
            decoded.contains(&syllable),
            "line {lineno}: decode({code}) = {decoded:?}, missing {syllable}"
        );
        count += 1;
    }
    assert!(count >= 400, "expected ~410 syllables, got {count}");
}
```

- [ ] **Step 9.2: Run the test**

Run: `cargo test --test golden`
Expected: PASS.

- [ ] **Step 9.3: Commit**

```bash
git add tests/golden.rs
git commit -m "test: golden TSV drives encode/decode round-trip"
```

---

## Task 10: `main.rs` — clap CLI with subcommands

**Files:**
- Modify: `src/main.rs`
- Modify: `src/lib.rs`

- [ ] **Step 10.1: Expose convenience surface in `lib.rs`**

Make `src/lib.rs`:

```rust
pub mod tables;
pub mod encode;
pub mod decode;
pub mod suggest;
pub mod keyboard;

pub use decode::{decode_code, DecodeError};
pub use encode::{encode_syllable, EncodeError};
pub use keyboard::{render_keyboard, Style};
pub use tables::SYLLABLES;
```

- [ ] **Step 10.2: Wire clap**

Replace `src/main.rs`:

```rust
use std::io::{IsTerminal, Write};
use std::process::ExitCode;

use anstream::{eprintln, println};
use clap::{Parser, Subcommand};

use xiaohe::{decode_code, encode_syllable, render_keyboard, EncodeError, Style, SYLLABLES};

#[derive(Parser)]
#[command(
    name = "xiaohe",
    version,
    about = "Query the Xiaohe Shuangpin (小鹤双拼) scheme"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Encode a pinyin syllable to its 2-key Shuangpin code
    #[command(visible_alias = "e")]
    Encode {
        /// A single legal pinyin syllable (use `v` for ü, e.g. `nv` = nü)
        syllable: String,
    },
    /// Decode a 2-key Shuangpin code to all matching pinyin syllables
    #[command(visible_alias = "d")]
    Decode {
        /// Two ASCII lowercase letters, e.g. `xn`
        code: String,
    },
    /// Print the Xiaohe Shuangpin keyboard layout
    Table,
    /// Print the full list of legal syllables with their Shuangpin codes
    List {
        /// Show only syllables starting with this prefix
        #[arg(long)]
        filter: Option<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Encode { syllable } => run_encode(&syllable),
        Cmd::Decode { code } => run_decode(&code),
        Cmd::Table => run_table(),
        Cmd::List { filter } => run_list(filter.as_deref()),
    }
}

fn run_encode(input: &str) -> ExitCode {
    match encode_syllable(input) {
        Ok(code) => {
            println!("{}{}", code[0], code[1]);
            ExitCode::SUCCESS
        }
        Err(EncodeError::Empty) => {
            eprintln!("error: empty input");
            ExitCode::from(1)
        }
        Err(EncodeError::InvalidSyllable { input, suggestions }) => {
            eprintln!("error: '{input}' is not a legal pinyin syllable");
            if !suggestions.is_empty() {
                let joined = suggestions
                    .iter()
                    .map(|s| format!("'{s}'"))
                    .collect::<Vec<_>>()
                    .join(", ");
                eprintln!("hint: did you mean {joined}?");
            }
            ExitCode::from(1)
        }
    }
}

fn run_decode(input: &str) -> ExitCode {
    match decode_code(input) {
        Ok(matches) if matches.is_empty() => {
            eprintln!("error: no pinyin syllable encodes to '{input}'");
            ExitCode::from(1)
        }
        Ok(matches) => {
            let mut stdout = std::io::stdout().lock();
            for m in matches {
                writeln!(stdout, "{m}").ok();
            }
            ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!("error: code must be exactly two ASCII lowercase letters");
            ExitCode::from(1)
        }
    }
}

fn run_table() -> ExitCode {
    let style = if std::io::stdout().is_terminal() {
        Style::Color
    } else {
        Style::Plain
    };
    println!("{}", render_keyboard(style));
    ExitCode::SUCCESS
}

fn run_list(filter: Option<&str>) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    for s in SYLLABLES {
        if let Some(prefix) = filter {
            if !s.starts_with(prefix) {
                continue;
            }
        }
        let code = encode_syllable(s).expect("SYLLABLES entry must encode");
        writeln!(stdout, "{s}\t{}{}", code[0], code[1]).ok();
    }
    ExitCode::SUCCESS
}
```

- [ ] **Step 10.3: Build the binary**

Run: `cargo build`
Expected: clean build.

- [ ] **Step 10.4: Smoke-test each subcommand**

Run each and verify the output matches:

```bash
$ cargo run -q -- encode xiao
xn

$ cargo run -q -- e xiao
xn

$ cargo run -q -- decode xn
xiao

$ cargo run -q -- d xn
xiao

$ cargo run -q -- encode xio
# stderr:
# error: 'xio' is not a legal pinyin syllable
# hint: did you mean 'xiao', 'xie', 'xi'?
# exit code: 1

$ cargo run -q -- list --filter xi | head
xi	xi
xia	xx
xian	xm
xiang	xl
xiao	xn
xie	xp
xin	xb
xing	xk
xiong	xs
xiu	xq

$ cargo run -q -- table
# Three rows of ASCII keyboard
```

If any output disagrees, fix the impl, NOT the expectations. The `list` codes shown here are derived from the spec — they should be self-consistent.

- [ ] **Step 10.5: Run the full test suite**

Run: `cargo test`
Expected: all PASS.

- [ ] **Step 10.6: Commit**

```bash
git add src/main.rs src/lib.rs
git commit -m "feat: clap CLI with encode/decode/table/list subcommands"
```

---

## Task 11: README polish

**Files:**
- Modify: `README.md`

- [ ] **Step 11.1: Rewrite `README.md`**

Replace the file with:

````markdown
# xiaohe

A small CLI for querying the **Xiaohe Shuangpin (小鹤双拼)** input scheme. It maps pinyin syllables to their two-key Shuangpin codes and back, prints the scheme tables, and offers did-you-mean suggestions for typos.

This is a **learning / lookup tool**, not an input method engine. It does not handle Chinese characters or multi-syllable text.

## Install

```bash
cargo install --git https://github.com/mhpsy/xiaohe
```

## Usage

```bash
$ xiaohe encode xiao         # alias: e
xn

$ xiaohe decode xn           # alias: d
xiao

$ xiaohe table               # ASCII keyboard layout

$ xiaohe list --filter x     # all syllables starting with x
xi	xi
xia	xx
xian	xm
...
```

Use `v` to type `ü`: `xiaohe encode nv` → `nv` (= nü).

Zero-initial syllables follow the Xiaohe rule that the initial key is the first letter of the final: `xiaohe encode ang` → `ah`.

## Why

The Xiaohe scheme has ~35 final mappings to memorize. This CLI is a fast offline lookup so you can drill the table from the terminal.

## License

MIT. See [LICENSE](./LICENSE).

Xiaohe Shuangpin scheme designed by 何海峰. This tool is independent and unaffiliated.
````

- [ ] **Step 11.2: Commit and push**

```bash
git add README.md
git commit -m "docs: write user-facing README"
git push
```

---

## Task 12: Tag v0.1.0

- [ ] **Step 12.1: Confirm a clean test run**

Run: `cargo test`
Expected: all PASS.

- [ ] **Step 12.2: Tag and push**

```bash
git tag -a v0.1.0 -m "v0.1.0: encode/decode/table/list for Xiaohe Shuangpin"
git push --tags
```

- [ ] **Step 12.3: Verify on GitHub**

Open https://github.com/mhpsy/xiaohe/releases — `v0.1.0` should appear.

---

## Done.

At this point the CLI satisfies the spec: encode, decode, table, list (with `--filter`), aliases `e`/`d`, friendly errors with did-you-mean, no JSON, table-driven golden tests.
