//! Rust implementation of RADIUS protocol.

// #![deny(missing_docs)]
// #![deny(missing_docs_in_private_items)]

pub mod attribute;
pub mod dictionary;
pub mod error;

mod md5;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex};

lazy_static! {
    pub static ref RADIUS_DICTIONARIES: Mutex<HashMap<String, attribute::Attribute>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}
