//! `hex` — Handl stdlib codec (`src/index.hnl`).
//!
//! Transitional: the ops are Rust until plan `stdlib-lib-migration` Phase 7
//! rewrites them as `ffi.call`.

pub const HEX_HNL: &str = include_str!("index.hnl");

pub mod ops;

pub use ops::{HEX_OPS, hex_op_registry};
