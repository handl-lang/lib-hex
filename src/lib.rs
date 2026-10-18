//! `hex` — hex codec. Impl is now pure-handl (`src/encode.hnl`, `src/decode.hnl`);
//! the Rust handls below remain until consumers switch to the `.hnl` module.

pub const HEX_HNL: &str = include_str!("index.hnl");

pub mod codec;
pub mod ops;

pub use codec::{register, Hex};
pub use ops::{hex_op_registry, HEX_OPS};
