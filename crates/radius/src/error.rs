//! Defines the types and auxilary functions to represent and work with
//! RADIUS errors.

use std::path::PathBuf;

use yaml_rust::scanner::ScanError;

/// Represents an error that may appear during any interractions
/// within the library.
#[derive(Debug)]
pub enum RadiusError {
    /// Will be returned from the `load_dictionaries` if the given `path` points
    /// to invalid directory with RADIUS dictionaries.
    ///
    /// The argument will contain path to the given directory with RADIUS
    /// dictionary.
    InvalidDictionaryDir(PathBuf),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary could not be opened.
    ///
    /// The first argument will contain io::Error with the reason why the
    /// dictionary file could not be opened and the second argument will
    /// contain path to the failed RADIUS dictionary.
    InvalidDictionaryFile(std::io::Error, PathBuf),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary contains invalid YAML.
    ///
    /// The first argument will contain yaml-rust ScannerError with the error
    /// that apperaed during parsing of the given RADIUS dictionary and the
    /// second argument will contain path to the failed RADIUS dictionary.
    IvalidDictionaryYaml(ScanError, PathBuf),
}
