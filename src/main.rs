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
