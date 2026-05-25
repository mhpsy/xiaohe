use crate::tables::SYLLABLES;
use strsim::damerau_levenshtein;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_xiao_for_xio() {
        let out = nearest("xio", 3);
        assert!(out.contains(&"xiao"), "expected xiao in {out:?}");
    }

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
}
