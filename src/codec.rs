//! The `hex` codec — implemented here, not in `lib-encoding`, so the contract
//! crate stays free of concrete impls.

use handl_lib_encoding::{Encoding, EncodingError, EncodingRegistry};
use serde_json::Value;

/// Lowercase hex codec.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hex;

impl Encoding for Hex {
    fn encode(&self, value: &Value) -> Result<Value, EncodingError> {
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("hex.encode expects a string".to_string()))?;
        Ok(Value::String(hex::encode(s.as_bytes())))
    }

    fn decode(&self, value: &Value) -> Result<Value, EncodingError> {
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("hex.decode expects a string".to_string()))?;
        let decoded =
            hex::decode(s).map_err(|e| EncodingError::Message(format!("hex decode error: {e}")))?;
        let out = String::from_utf8(decoded)
            .map_err(|e| EncodingError::Message(format!("hex decode is not valid utf-8: {e}")))?;
        Ok(Value::String(out))
    }
}

/// Register this codec under `"hex"`.
pub fn register(registry: EncodingRegistry) -> EncodingRegistry {
    registry.with("hex", Hex)
}
