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

/// Sorted list of all legal Mandarin pinyin syllables (ASCII, with `v` for ü).
/// 409 entries. Each one's Shuangpin code is the deterministic output of
/// `encode::encode_syllable` — see `tests/fixtures/syllables.tsv` for the
/// canonical syllable → code mapping exercised by the golden integration test.
pub static SYLLABLES: &[&str] = &[
    "a", "ai", "an", "ang", "ao", "ba", "bai", "ban",
    "bang", "bao", "bei", "ben", "beng", "bi", "bian", "biao",
    "bie", "bin", "bing", "bo", "bu", "ca", "cai", "can",
    "cang", "cao", "ce", "cen", "ceng", "cha", "chai", "chan",
    "chang", "chao", "che", "chen", "cheng", "chi", "chong", "chou",
    "chu", "chua", "chuai", "chuan", "chuang", "chui", "chun", "chuo",
    "ci", "cong", "cou", "cu", "cuan", "cui", "cun", "cuo",
    "da", "dai", "dan", "dang", "dao", "de", "dei", "den",
    "deng", "di", "dian", "diao", "die", "ding", "diu", "dong",
    "dou", "du", "duan", "dui", "dun", "duo", "e", "ei",
    "en", "eng", "er", "fa", "fan", "fang", "fei", "fen",
    "feng", "fo", "fou", "fu", "ga", "gai", "gan", "gang",
    "gao", "ge", "gei", "gen", "geng", "gong", "gou", "gu",
    "gua", "guai", "guan", "guang", "gui", "gun", "guo", "ha",
    "hai", "han", "hang", "hao", "he", "hei", "hen", "heng",
    "hong", "hou", "hu", "hua", "huai", "huan", "huang", "hui",
    "hun", "huo", "ji", "jia", "jian", "jiang", "jiao", "jie",
    "jin", "jing", "jiong", "jiu", "ju", "juan", "jue", "jun",
    "ka", "kai", "kan", "kang", "kao", "ke", "ken", "keng",
    "kong", "kou", "ku", "kua", "kuai", "kuan", "kuang", "kui",
    "kun", "kuo", "la", "lai", "lan", "lang", "lao", "le",
    "lei", "leng", "li", "lia", "lian", "liang", "liao", "lie",
    "lin", "ling", "liu", "lo", "long", "lou", "lu", "luan",
    "lun", "luo", "lv", "lve", "ma", "mai", "man", "mang",
    "mao", "me", "mei", "men", "meng", "mi", "mian", "miao",
    "mie", "min", "ming", "miu", "mo", "mou", "mu", "na",
    "nai", "nan", "nang", "nao", "ne", "nei", "nen", "neng",
    "ni", "nian", "niang", "niao", "nie", "nin", "ning", "niu",
    "nong", "nou", "nu", "nuan", "nun", "nuo", "nv", "nve",
    "o", "ou", "pa", "pai", "pan", "pang", "pao", "pei",
    "pen", "peng", "pi", "pian", "piao", "pie", "pin", "ping",
    "po", "pou", "pu", "qi", "qia", "qian", "qiang", "qiao",
    "qie", "qin", "qing", "qiong", "qiu", "qu", "quan", "que",
    "qun", "ran", "rang", "rao", "re", "ren", "reng", "ri",
    "rong", "rou", "ru", "rua", "ruan", "rui", "run", "ruo",
    "sa", "sai", "san", "sang", "sao", "se", "sen", "seng",
    "sha", "shai", "shan", "shang", "shao", "she", "shei", "shen",
    "sheng", "shi", "shou", "shu", "shua", "shuai", "shuan", "shuang",
    "shui", "shun", "shuo", "si", "song", "sou", "su", "suan",
    "sui", "sun", "suo", "ta", "tai", "tan", "tang", "tao",
    "te", "teng", "ti", "tian", "tiao", "tie", "ting", "tong",
    "tou", "tu", "tuan", "tui", "tun", "tuo", "wa", "wai",
    "wan", "wang", "wei", "wen", "weng", "wo", "wu", "xi",
    "xia", "xian", "xiang", "xiao", "xie", "xin", "xing", "xiong",
    "xiu", "xu", "xuan", "xue", "xun", "ya", "yan", "yang",
    "yao", "ye", "yi", "yin", "ying", "yo", "yong", "you",
    "yu", "yuan", "yue", "yun", "za", "zai", "zan", "zang",
    "zao", "ze", "zei", "zen", "zeng", "zha", "zhai", "zhan",
    "zhang", "zhao", "zhe", "zhei", "zhen", "zheng", "zhi", "zhong",
    "zhou", "zhu", "zhua", "zhuai", "zhuan", "zhuang", "zhui", "zhun",
    "zhuo", "zi", "zong", "zou", "zu", "zuan", "zui", "zun",
    "zuo",
];

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

    #[test]
    fn syllables_are_sorted_and_unique() {
        for w in SYLLABLES.windows(2) {
            assert!(w[0] < w[1], "out of order or duplicate: {} >= {}", w[0], w[1]);
        }
    }
}
