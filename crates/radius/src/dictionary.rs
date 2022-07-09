//! Defines the types and auxilary functions to load RADIUS dictionaries
//! and access them.

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::error::RadiusError;

use walkdir::WalkDir;
use yaml_rust::YamlLoader;

#[allow(dead_code)]
const RADIUS_DICTIONARIES_DIR: &'static str = "/usr/share/radius";

const ENV_RADIUS_DICTIONARIES_DIR: &'static str = "RADIUS_DICTIONARIES_DIR";

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
pub fn load_dictionaries(_set: DictionarySet, path: Option<PathBuf>) -> Result<(), RadiusError> {
    let dicts_dir = dictionaries_path(path);

    if !dicts_dir.exists() {
        return Err(RadiusError::InvalidDictionaryDir(dicts_dir));
    }

    if !dicts_dir.is_dir() {
        return Err(RadiusError::InvalidDictionaryDir(dicts_dir));
    }

    for entry in WalkDir::new(dicts_dir).into_iter().filter_map(|e| e.ok()) {
        let dictionary: &Path = entry.path();
        if dictionary.is_file() && dictionary.extension().unwrap() == "yaml" {
            // Try to open RADIUS dictionary
            let fd = File::open(dictionary);
            match fd {
                Ok(_) => {}
                Err(err) => {
                    return Err(RadiusError::InvalidDictionaryFile(
                        err,
                        dictionary.to_owned(),
                    ));
                }
            }

            // read yaml data
            let mut dict = String::new();
            fd.unwrap().read_to_string(&mut dict).unwrap();

            // load yaml
            let yaml = YamlLoader::load_from_str(dict.as_ref());
            match yaml {
                Ok(_) => {}
                Err(err) => {
                    return Err(RadiusError::IvalidDictionaryYaml(
                        err,
                        dictionary.to_owned(),
                    ));
                }
            };
        }
    }

    Ok(())
}

fn dictionaries_path(path: Option<PathBuf>) -> PathBuf {
    path.or_else(|| -> Option<PathBuf> {
        env::var_os(ENV_RADIUS_DICTIONARIES_DIR)
            .as_ref()
            .and_then(|p: &OsString| -> Option<PathBuf> {
                Some(AsRef::<Path>::as_ref(p).to_owned())
            })
            .or_else(|| -> Option<PathBuf> { Some(Path::new(RADIUS_DICTIONARIES_DIR).to_owned()) })
    })
    .unwrap()
}
