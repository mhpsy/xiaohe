use anstyle::{AnsiColor, Reset, Style as Ansi};
use crate::tables::{FINAL_MAP, MULTI_LETTER_INITIALS};

#[derive(Copy, Clone, Debug)]
pub enum Style {
    Color,
    Plain,
}

const ROW_1: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const ROW_2: &[char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
const ROW_3: &[char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];

const CELL_WIDTH: usize = 5;
const INITIAL_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Blue)));
const FINAL_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
const DIM_STYLE: Ansi = Ansi::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::BrightBlack)));

fn initial_for(key: char) -> String {
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

pub fn render_keyboard(style: Style) -> String {
    let mut out = String::new();
    out.push_str(&render_row(ROW_1, 0, style));
    out.push_str(&render_row(ROW_2, 3, style));
    out.push_str(&render_row(ROW_3, 6, style));
    out
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
}
