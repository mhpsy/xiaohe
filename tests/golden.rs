use std::fs;

use xiaohe::{decode_code, encode_syllable, render_keyboard, Style, SYLLABLES};

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

        let encoded = encode_syllable(syllable)
            .unwrap_or_else(|e| panic!("line {lineno}: encode {syllable} failed: {e:?}"));
        let actual: String = encoded.iter().collect();
        assert_eq!(
            actual, code,
            "line {lineno}: encode({syllable}) -> {actual}, expected {code}"
        );

        let decoded = decode_code(code).expect("decode_code");
        assert!(
            decoded.contains(&syllable),
            "line {lineno}: decode({code}) = {decoded:?}, missing {syllable}"
        );
        count += 1;
    }
    assert!(count >= 400, "expected ~410 syllables, got {count}");
}

#[test]
fn table_plain_matches_snapshot() {
    let actual = render_keyboard(Style::Plain);
    let expected = include_str!("fixtures/table.snapshot");
    assert_eq!(actual, expected, "tests/fixtures/table.snapshot is stale; \
        regenerate with `cargo run -q -- table > tests/fixtures/table.snapshot`");
}

#[test]
fn list_plain_matches_snapshot() {
    let mut actual = String::new();
    for s in SYLLABLES {
        let code = encode_syllable(s).expect("SYLLABLES entry must encode");
        actual.push_str(&format!("{s}\t{}{}\n", code[0], code[1]));
    }
    let expected = include_str!("fixtures/list.snapshot");
    assert_eq!(actual, expected, "tests/fixtures/list.snapshot is stale; \
        regenerate with `cargo run -q -- list > tests/fixtures/list.snapshot`");
}
