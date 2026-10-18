//! `hex` — hex codec. Impl is now pure-handl (`src/encode.hnl`, `src/decode.hnl`);
//! the Rust handls below remain until consumers switch to the `.hnl` module.

pub const HEX_HNL: &str = include_str!("index.hnl");

pub mod codec;
pub mod ops;

pub use codec::{register, Hex};
pub use ops::{hex_op_registry, HEX_OPS};

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const HEX_FUNCS: &[&str] = &["encode","decode"];
pub const HEX_SOURCES: &[(&str, &str, &[&str])] = &[
    ("encode", include_str!("encode.hnl"), &["encode"]),
    ("decode", include_str!("decode.hnl"), &["decode"]),
];
