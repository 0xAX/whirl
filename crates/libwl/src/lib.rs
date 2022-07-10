pub mod conf;
pub mod ev;
pub mod radius;

use conf::{Config, ConfigError};
use lazy_static::lazy_static;
use mlua::prelude::*;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

// Global LUA context.
lazy_static! {
    pub static ref LUA_SCOPE: Mutex<mlua::Lua> = {
        let lua = unsafe { Lua::unsafe_new() };
        Mutex::new(lua)
    };
}

pub fn load(script: &str) -> Result<Config, ConfigError> {
    let (dylib_path, dylib_ext, separator);

    dylib_path = env::var("LD_LIBRARY_PATH").unwrap();
    dylib_ext = "so";
    separator = ":";

    let cpath = dylib_path
        .split(separator)
        .take(3)
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push(format!("lib?.{}", dylib_ext));
            path.to_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join(";");

    let _ = LUA_SCOPE
        .lock()
        .unwrap()
        .load(&format!("package.cpath = \"{}\"", cpath))
        .exec()
        .unwrap();

    let _ = LUA_SCOPE.lock().unwrap().load(script).exec().unwrap();
    // load scenario script
    // let lua = unsafe { Lua::unsafe_new().into_static() };
    // let _ = lua
    //     .load(&format!("package.cpath = \"{}\"", cpath))
    //     .exec()
    //     .unwrap();
    // let _ = lua.load(script).exec().unwrap();

    // // Try to load configuration from the scenario file
    let config = conf::Config::new();

    config
}

#[mlua::lua_module]
fn libwl(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    // exports.set("imsi_from_range",
    //             lua.create_function(radius::imsi_from_range)?)?;
    Ok(exports)
}
