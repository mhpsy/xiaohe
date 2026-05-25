use crate::tables::{lookup_final, lookup_initial, SYLLABLES, ZERO_INITIAL_FINALS};

/// Errors that can arise when encoding a pinyin syllable to its Shuangpin code.
#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    /// Input was empty or whitespace-only.
    Empty,
    /// Input was not a legal Mandarin pinyin syllable. `suggestions` lists up
    /// to three closest legal syllables (Damerau-Levenshtein distance ≤ 2).
    InvalidSyllable {
        input: String,
        suggestions: Vec<&'static str>,
    },
}

/// Encode a single pinyin syllable to its 2-character Xiaohe Shuangpin code.
///
/// Input is normalized by trimming, ASCII-lowercasing, and substituting any
/// literal `ü` (U+00FC) with `v` per the Xiaohe convention. Returns
/// [`EncodeError::Empty`] for blank input and [`EncodeError::InvalidSyllable`]
/// if the normalized form is not in [`crate::SYLLABLES`].
pub fn encode_syllable(input: &str) -> Result<[char; 2], EncodeError> {
    let normalized = input.trim().to_ascii_lowercase().replace('\u{00fc}', "v");
    if normalized.is_empty() {
        return Err(EncodeError::Empty);
    }

    if !SYLLABLES.contains(&normalized.as_str()) {
        return Err(EncodeError::InvalidSyllable {
            input: normalized.clone(),
            suggestions: crate::suggest::nearest(&normalized, 3),
        });
    }

    // Zero-initial: the syllable IS its own final.
    if ZERO_INITIAL_FINALS.contains(&normalized.as_str()) {
        let initial_key = normalized.as_bytes()[0] as char;
        let final_key = lookup_final(&normalized).expect("zero-initial final in FINAL_MAP");
        return Ok([initial_key, final_key]);
    }

    // Two-letter initial first (zh/ch/sh).
    if normalized.len() >= 3 {
        let head = &normalized[..2];
        if let Some(initial_key) = lookup_initial(head) {
            let tail = &normalized[2..];
            if let Some(final_key) = lookup_final(tail) {
                return Ok([initial_key, final_key]);
            }
        }
    }

    // Single-letter initial + final.
    let head = normalized.as_bytes()[0] as char;
    let tail = &normalized[1..];
    let final_key = lookup_final(tail)
        .expect("SYLLABLES entry must split into known initial + final");
    Ok([head, final_key])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_xiao() {
        assert_eq!(encode_syllable("xiao"), Ok(['x', 'n']));
    }

    #[test]
    fn encodes_zh_ch_sh_initials() {
        assert_eq!(encode_syllable("zhong"), Ok(['v', 's'])); // zh + ong
        assert_eq!(encode_syllable("chuang"), Ok(['i', 'l'])); // ch + uang
        assert_eq!(encode_syllable("shuai"), Ok(['u', 'k'])); // sh + uai
    }

    #[test]
    fn encodes_zero_initial_syllables() {
        assert_eq!(encode_syllable("ang"), Ok(['a', 'h'])); // a + ang
        assert_eq!(encode_syllable("ai"), Ok(['a', 'd']));   // a + ai
        assert_eq!(encode_syllable("er"), Ok(['e', 'r']));   // e + er
        assert_eq!(encode_syllable("ou"), Ok(['o', 'z']));   // o + ou
        assert_eq!(encode_syllable("a"), Ok(['a', 'a']));    // a + a
        assert_eq!(encode_syllable("e"), Ok(['e', 'e']));    // e + e
    }

    #[test]
    fn encodes_v_as_u_umlaut() {
        assert_eq!(encode_syllable("nv"), Ok(['n', 'v']));   // nü
        assert_eq!(encode_syllable("lve"), Ok(['l', 't']));  // lüe (ve = üe → t)
        assert_eq!(encode_syllable("nve"), Ok(['n', 't']));  // nüe
    }

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

    #[test]
    fn accepts_literal_u_umlaut_character() {
        assert_eq!(encode_syllable("n\u{00fc}"), Ok(['n', 'v']));
        assert_eq!(encode_syllable("l\u{00fc}e"), Ok(['l', 't']));
    }

    #[test]
    fn rejects_non_syllable_that_splits_cleanly() {
        // 'biang' splits as b+iang -> 'bl', but 'biang' is not a legal pinyin syllable.
        assert!(matches!(
            encode_syllable("biang"),
            Err(EncodeError::InvalidSyllable { .. })
        ));
        // 'juo' splits as j+uo -> 'jo', but 'juo' is not a legal syllable.
        assert!(matches!(
            encode_syllable("juo"),
            Err(EncodeError::InvalidSyllable { .. })
        ));
    }

    #[test]
    fn xian_does_not_split_as_xi_plus_an() {
        assert_eq!(encode_syllable("xian"), Ok(['x', 'm']));
    }

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
}
