//! Utility module for exporting to JSON.

use crate::SimulatedPath;

use std::fs::File;
use std::io::prelude::*;

#[cfg(feature = "json")]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct JSONExport {
    ts: Vec<f32>,
    xs: Vec<f32>,
}

/// Typealias for a JSON byte vector.
#[cfg(feature = "json")]
pub type JSONBytes = Vec<u8>;

/// Serialize a [`SimulatedPath`] to a [`JSONBytes`].
#[cfg(feature = "json")]
pub fn serialize_simluated_path(s: SimulatedPath) -> JSONBytes {
    use nalgebra::RawStorage;

    let out = JSONExport {
        ts: unsafe { s.columns(0, 1).data.as_slice_unchecked().to_vec() },
        xs: unsafe { s.columns(1, 1).data.as_slice_unchecked().to_vec() },
    };

    serde_json::to_vec(&out).unwrap()
}

/// Write a [`JSONBytes`] to a `.json` file.
#[cfg(feature = "json")]
pub fn write_json_file(s: JSONBytes, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&s)?;
    Ok(())
}

/// Export a [`SimulatedPath`] to a `.json` file.
#[cfg(feature = "json")]
pub fn export_to_json(s: SimulatedPath, path: &str) -> std::io::Result<()> {
    write_json_file(serialize_simluated_path(s), path)
}
