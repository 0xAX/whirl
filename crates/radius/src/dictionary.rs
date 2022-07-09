/// Defines the types and auxilary functions to load RADIUS dictionaries.

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::attribute::{Attribute, Vendor};
use super::error::RadiusError;

use walkdir::WalkDir;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

// The path where the whirl RADIUS dictionaries should be installed
// after cargo install.
const RADIUS_DICTIONARIES_DIR: &'static str = "/usr/share/radius";

// The environment variable that could point to the directory where
// whirl RADIUS dictionaries are located.
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
///
/// If loading of RADIUS dictionaries will be succesfully executed the
/// hashmap with mapping of RADIUS attribute names to `radius::Attribute`
/// will be returned. Otherwise one of `RadiusError` value.
pub fn load_dictionaries(
    _set: DictionarySet,
    path: Option<PathBuf>,
) -> Result<HashMap<String, Attribute>, RadiusError> {
    let dicts_dir = dictionaries_path(path);

    if !dicts_dir.exists() {
        return Err(RadiusError::InvalidDictionaryDir(dicts_dir));
    }

    if !dicts_dir.is_dir() {
        return Err(RadiusError::InvalidDictionaryDir(dicts_dir));
    }

    let mut hash = HashMap::new();

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
                    return Err(RadiusError::InvalidYaml(err, dictionary.to_owned()));
                }
            };

            // start to build RADIUS attributes map
            let document: &Yaml = &yaml.unwrap()[0];
            let vendor = &document["vendor"];
            let attributes: &Vec<Yaml> = &document["attributes"].as_vec().unwrap();

            // Go through the RADIUS attributes within yaml document
            for attribute in attributes {
                // read attribute name
                let key = match &attribute["attribute"] {
                    Yaml::String(value) => value,
                    _ => {
                        return Err(RadiusError::DictionaryMissedAttrKey(dictionary.to_owned()));
                    }
                };

                // read attribute id
                let id = match &attribute["id"] {
                    Yaml::Integer(value) => value,
                    _ => {
                        return Err(RadiusError::DictionaryMissedAttrId(
                            dictionary.to_owned(),
                            key.to_string(),
                        ));
                    }
                };

                // read possible vendor id
                let v = match vendor {
                    Yaml::BadValue => None,
                    Yaml::Integer(vnd) => Some(Vendor::new(vnd.clone() as u32, id.clone() as u8)),
                    _ => {
                        return Err(RadiusError::DictionaryIvalidVendorId(dictionary.to_owned()));
                    }
                };

                // According to RFC 2865 5.26:
                //
                // Type
                //       26 for Vendor-Specific.
                let attr_id = match v {
                    None => id.clone() as u8,
                    Some(_) => 26,
                };

                // insert new attribute into hash
                let new_attr = Attribute::new(attr_id, v);
                hash.insert(key.to_string(), new_attr);
            }
        }
    }

    Ok(hash)
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
