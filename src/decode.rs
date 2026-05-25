use crate::encode::encode_syllable;
use crate::tables::SYLLABLES;

/// Errors that can arise when decoding a Shuangpin code back to pinyin.
#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The input was not exactly two ASCII lowercase letters.
    InvalidFormat { input: String },
}

/// Decode a 2-letter Shuangpin code into every legal pinyin syllable that
/// produces that code under [`encode_syllable`]. The returned vector is
/// sorted lexicographically and may be empty.
pub fn decode_code(code: &str) -> Result<Vec<&'static str>, DecodeError> {
    let normalized = code.trim().to_ascii_lowercase();
    if normalized.chars().count() != 2 || !normalized.chars().all(|c| c.is_ascii_lowercase()) {
        return Err(DecodeError::InvalidFormat { input: normalized });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_xn_to_xiao() {
        let out = decode_code("xn").unwrap();
        assert!(out.contains(&"xiao"), "expected xiao in {out:?}");
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(matches!(decode_code("x"), Err(DecodeError::InvalidFormat { .. })));
        assert!(matches!(decode_code("xyz"), Err(DecodeError::InvalidFormat { .. })));
        assert!(matches!(decode_code("x1"), Err(DecodeError::InvalidFormat { .. })));
    }
}
