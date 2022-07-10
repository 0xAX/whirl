use super::LUA_SCOPE;
use mlua::Table;

const WORKERS: u8 = 4;

#[derive(Debug)]
pub struct Config {
    workers: u8,
    radius: Option<RadiusConfig>,
}

#[derive(Debug)]
pub enum ConfigError {
    LuaError(mlua::prelude::LuaError),
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        let lua = LUA_SCOPE.lock().unwrap();
        let globals = lua.globals();

        let workload = match globals.get::<_, Table>("workload") {
            Ok(w) => w,
            Err(e) => {
                return Err(ConfigError::LuaError(e));
            }
        };

        let workers: u8 = match workload.get::<_, u8>("workers") {
            Ok(w) => w,
            _ => WORKERS,
        };

        let radius_conf: Result<RadiusConfig, ConfigError> = Self::maybe_radius_conf(&workload);
        match radius_conf {
            Err(e) => {
                return Err(e);
            }
            Ok(_) => {}
        }

        let config = Config {
            workers: workers,
            radius: Some(radius_conf.unwrap()),
        };

        Ok(config)
    }

    fn maybe_radius_conf(_workload: &Table) -> Result<RadiusConfig, ConfigError> {
        Ok(RadiusConfig {})
    }
}

#[derive(Debug)]
struct RadiusConfig {}
