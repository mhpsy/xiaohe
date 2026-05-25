//! Library backing the `xiaohe` CLI.
//!
//! Public surface: [`encode_syllable`], [`decode_code`], [`render_keyboard`],
//! and the static [`SYLLABLES`] list. Internal modules (tables, suggest) are
//! intentionally crate-private.

mod tables;
mod encode;
mod decode;
mod suggest;
mod keyboard;

pub use decode::{decode_code, DecodeError};
pub use encode::{encode_syllable, EncodeError};
pub use keyboard::{render_keyboard, Style};
pub use tables::SYLLABLES;
