//! Utility module for exporting to Python.

use crate::SimulatedPath;

use std::fs::File;
use std::io::prelude::*;

#[cfg(feature = "py")]
#[cfg_attr(feature = "py", derive(serde::Serialize))]
pub struct JSONExport {
    ts: Vec<f32>,
    xs: Vec<f32>,
}

/// Typealias for a Pickle byte vector.
#[cfg(feature = "py")]
pub type PickleBytes = Vec<u8>;

/// Serialize a [`SimulatedPath`] to a [`PickleBytes`].
#[cfg(feature = "py")]
pub fn serialize_simulated_path(s: SimulatedPath) -> PickleBytes {
    use nalgebra::RawStorage;

    let out = JSONExport {
        ts: unsafe { s.columns(0, 1).data.as_slice_unchecked().to_vec() },
        xs: unsafe { s.columns(1, 1).data.as_slice_unchecked().to_vec() },
    };

    serde_pickle::to_vec(&out, Default::default()).unwrap()
}

/// Write a [`PickleBytes`] to a `.pickle` file.
#[cfg(feature = "py")]
pub fn write_pickle_file(s: PickleBytes, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&s)?;
    Ok(())
}

/// Export a [`SimulatedPath`] to a `.pickle` file.
#[cfg(feature = "py")]
pub fn export_to_pickle(s: SimulatedPath, path: &str) -> std::io::Result<()> {
    write_pickle_file(serialize_simulated_path(s), path)
}
