use mlua::prelude::*;

pub fn imsi_from_range(_: &Lua, range: String) -> LuaResult<String> {
    Ok(range)
}
