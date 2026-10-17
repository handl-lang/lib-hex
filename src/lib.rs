//! `hex` — Handl stdlib codec (`src/index.hnl`).
//!
//! Transitional: the ops are Rust until plan `stdlib-lib-migration` Phase 7
//! rewrites them as `ffi.call`.

pub const HEX_HNL: &str = include_str!("index.hnl");

pub mod codec;
pub mod ops;

pub use codec::{register, Hex};
pub use ops::{hex_op_registry, HEX_OPS};
