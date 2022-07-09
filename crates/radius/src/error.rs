/// Defines the types and auxilary functions to represent and work with
/// RADIUS errors.

use std::path::PathBuf;

use yaml_rust::scanner::ScanError;

/// Path to the directory with RADIUS dictionaries.
type RadiusDictionariesDir = PathBuf;

/// Path to the RADIUS dictionary YAML file.
type RadiusDictionary = PathBuf;

/// RADIUS attribute name.
type RadiusAttrKey = String;

/// Represents an error that may appear during any interractions
/// within the library.
#[derive(Debug)]
pub enum RadiusError {
    /// Will be returned from the `load_dictionaries` if the given `path` points
    /// to invalid directory with RADIUS dictionaries.
    ///
    /// The argument will contain path to the given directory with RADIUS
    /// dictionary.
    InvalidDictionaryDir(RadiusDictionariesDir),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary could not be opened.
    ///
    /// The first argument will contain io::Error with the reason why the
    /// dictionary file could not be opened and the second argument will
    /// contain path to the failed RADIUS dictionary.
    InvalidDictionaryFile(std::io::Error, RadiusDictionary),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary contains invalid YAML.
    ///
    /// The first argument will contain yaml-rust ScannerError with the error
    /// that apperaed during parsing of the given RADIUS dictionary and the
    /// second argument will contain path to the failed RADIUS dictionary.
    InvalidYaml(ScanError, RadiusDictionary),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary does not contain `attrbiute` key for definition of any
    /// RADIUS attributes.
    ///
    /// The first argument will contain path to the given/failed RADIUS
    /// dictionary.
    DictionaryMissedAttrKey(RadiusDictionary),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary does not contain `id` key for definition of any
    /// RADIUS attributes.
    ///
    /// The first argument will contain path to the given/failed RADIUS
    /// dictionary. The second argument is the name of the RADIUS attribute
    /// with the missed `id`.
    DictionaryMissedAttrId(RadiusDictionary, RadiusAttrKey),
    /// Will be returned from the `load_dictionaries` if the given RADIUS
    /// dictionary contains invalid `vendor-id` key.
    ///
    /// The first argument will contain path to the given/failed RADIUS
    /// dictionary.
    DictionaryIvalidVendorId(RadiusDictionary),
}
