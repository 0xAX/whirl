#[derive(Debug)]
pub struct Config {
    workers: u32,
    radius: Option<RadiusConfig>,
}

#[derive(Debug)]
pub enum ConfigError {
    LuaError(mlua::prelude::LuaError),
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        // TODO: load workers amount and other things to build config
        // let g = lua.globals();
        // let _workload = match g.get::<_, Table>("workload") {
        //     Ok(w) => w,
        //     Err(e) => {
        //         return Err(ConfigError::LuaError(e));
        //     }
        // };

        // TODO: use amount of CPUs as defalut amount of workers not given
        let config = Config {
            workers: 8,
            radius: None,
        };

        Ok(config)
    }
}

#[derive(Debug)]
struct RadiusConfig {}
