pub mod radius;

use std::env;
use std::path::PathBuf;

use mlua::prelude::*;

pub fn load(script: &str) -> () {
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

    let lua = unsafe { Lua::unsafe_new() };
    lua.load(&format!("package.cpath = \"{}\"", cpath))
        .exec()
        .unwrap();
    lua.load(script).exec().unwrap();

    ()
}

#[mlua::lua_module]
fn libwl(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    // exports.set("imsi_from_range",
    //             lua.create_function(radius::imsi_from_range)?)?;
    Ok(exports)
}
