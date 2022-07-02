//! Defines the types and auxilary functions to load RADIUS dictionaries
//! and access them.

use std::path::Path;

use super::error::{RadiusError};

#[allow(dead_code)]
const RADIUS_DICTIONARIES_DIR: &'static str = "/usr/share/radius";

/// Set RADIUS of dictionaries to load.
pub enum DictionarySet {
    /// Load all existing RADIUS dictionaries.
    All,
    /// Load RADIUS dictionaries from the given `vector`.
    Set(Vec<String>),
}

/// Load set of RADIUS dictionaries specified by the given
/// dictionary `set`.
///
/// The dictionaries will be loaded from:
///
///   * The `path` which points to directory with RADIUS dictionaries.
///   * If the `path` is `None` the `RADIUS_DICTIONARIES_DIR` environment
///     variable will be checked.
///   * `/usr/share/radius` directory will be used if both previous souces
///     are not set.
pub fn load_dictionaries(_set: DictionarySet, _path: Option<&Path>) -> Result<(), RadiusError> {
    Ok(())
}
