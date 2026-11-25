//! `hex` — hex codec. Pure-handl impl (`src/encode.hnl`, `src/decode.hnl`,
//! flattened in `src/index_flat.hnl`); the legacy Rust handls (`ops.rs`)
//! were pruned after parity — every consumer now loads the `.hnl` module
//! (plugin-lib-stdlib package table + handl-kit-stdlib).

/// The handl source declaring the hex `op` interface.
pub const HEX_HNL: &str = include_str!("index.hnl");

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const HEX_FUNCS: &[&str] = &["encode", "decode", "codec"];
pub const HEX_SOURCES: &[(&str, &str, &[&str])] = &[
    ("encode", include_str!("encode.hnl"), &["encode"]),
    ("decode", include_str!("decode.hnl"), &["decode"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const HEX_FLAT_HNL: &str = include_str!("index_flat.hnl");
