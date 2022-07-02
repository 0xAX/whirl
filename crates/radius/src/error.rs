//! Defines the types and auxilary functions to represent and work with
//! RADIUS errors.

/// Represents an error that may appear during any interractions
/// within the library.
#[derive(Debug)]
pub enum RadiusError {
    /// Will be returned by the `load_dictionaries` if the given `path` points
    /// to invalid directory with RADIUS dictionaries.
    InvalidDictionaryDir,
}
