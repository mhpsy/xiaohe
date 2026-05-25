/// Pinyin initials that map to a non-self key. All other single-letter
/// initials (b/p/m/f/d/t/n/l/g/k/h/j/q/x/r/z/c/s/y/w) map to themselves
/// and are NOT listed here — `initial_key()` returns `Some(c)` for them
/// by falling back to the first character.
pub static MULTI_LETTER_INITIALS: &[(&str, char)] = &[
    ("zh", 'v'),
    ("ch", 'i'),
    ("sh", 'u'),
];

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

    #[test]
    fn final_map_has_expected_mappings() {
        let pairs: std::collections::HashMap<&str, char> =
            FINAL_MAP.iter().copied().collect();
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
}
