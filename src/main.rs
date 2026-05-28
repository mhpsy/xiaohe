use std::io::{BufRead, IsTerminal, Write};
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
    /// Encode pinyin syllables interactively, one per line
    #[command(visible_alias = "i")]
    Interactive,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Encode { syllable } => run_encode(&syllable),
        Cmd::Decode { code } => run_decode(&code),
        Cmd::Table => run_table(),
        Cmd::List { filter } => run_list(filter.as_deref()),
        Cmd::Interactive => run_interactive(),
    }
}

fn run_encode(input: &str) -> ExitCode {
    let mut out = std::io::stdout().lock();
    let mut err = std::io::stderr().lock();
    if report_encode(input, &mut out, &mut err) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// Encode `input` and write the result to `out` or an error to `err`.
/// Returns whether encoding succeeded.
fn report_encode(input: &str, out: &mut impl Write, err: &mut impl Write) -> bool {
    match encode_syllable(input) {
        Ok(code) => {
            writeln!(out, "{}{}", code[0], code[1]).ok();
            true
        }
        Err(EncodeError::Empty) => {
            writeln!(err, "error: empty input").ok();
            false
        }
        Err(EncodeError::InvalidSyllable { input, suggestions }) => {
            writeln!(err, "error: '{input}' is not a legal pinyin syllable").ok();
            if !suggestions.is_empty() {
                let joined = suggestions
                    .iter()
                    .map(|s| format!("'{s}'"))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(err, "hint: did you mean {joined}?").ok();
            }
            false
        }
    }
}

fn run_interactive() -> ExitCode {
    eprintln!("xiaohe interactive — type a pinyin syllable, Ctrl-D or 'quit' to exit");
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut out = std::io::stdout().lock();
    let mut err = std::io::stderr().lock();
    match run_interactive_loop(reader, &mut out, &mut err) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run_interactive_loop(
    reader: impl BufRead,
    out: &mut impl Write,
    err: &mut impl Write,
) -> std::io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "quit" || trimmed == "exit" {
            break;
        }
        report_encode(trimmed, out, err);
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn report_encode_valid_writes_code_and_returns_true() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let ok = report_encode("hao", &mut out, &mut err);
        assert!(ok);
        assert_eq!(String::from_utf8(out).unwrap(), "hc\n");
        assert!(err.is_empty());
    }

    #[test]
    fn report_encode_invalid_writes_error_and_returns_false() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let ok = report_encode("zzzz", &mut out, &mut err);
        assert!(!ok);
        assert!(out.is_empty());
        let err = String::from_utf8(err).unwrap();
        assert!(
            err.contains("not a legal pinyin syllable"),
            "unexpected stderr: {err}"
        );
    }

    #[test]
    fn interactive_loop_encodes_each_line_skips_blank_stops_at_quit() {
        let input = Cursor::new("hao\n\nzzzz\nxian\nquit\nhao\n");
        let mut out = Vec::new();
        let mut err = Vec::new();
        run_interactive_loop(input, &mut out, &mut err).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "hc\nxm\n");
        let err = String::from_utf8(err).unwrap();
        assert!(
            err.contains("not a legal pinyin syllable"),
            "unexpected stderr: {err}"
        );
    }

    #[test]
    fn interactive_loop_stops_at_eof() {
        let input = Cursor::new("hao\n");
        let mut out = Vec::new();
        let mut err = Vec::new();
        run_interactive_loop(input, &mut out, &mut err).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "hc\n");
    }
}
