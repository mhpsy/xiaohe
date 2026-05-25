pub mod tables;
pub mod encode;
pub mod decode;
pub mod suggest;
pub mod keyboard;

pub use decode::{decode_code, DecodeError};
pub use encode::{encode_syllable, EncodeError};
pub use keyboard::{render_keyboard, Style};
pub use tables::SYLLABLES;
