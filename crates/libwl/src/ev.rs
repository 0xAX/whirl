use super::conf::Config;
use mlua::{Function, Lua, Table, Value};
use num_bigint::BigInt;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use radius::attribute::{Attribute, Vendor};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum IOEngine {
    WIO,
    Tokio,
}

#[derive(Debug)]
pub struct Ev {
    threads: u8,
    engine: IOEngine,
}

impl Ev {
    pub fn new() -> Ev {
        Ev {
            threads: 4,
            engine: IOEngine::WIO,
        }
    }

    pub fn set_threads(&mut self, threads: u8) -> &mut Self {
        self.threads = threads;
        self
    }

    pub fn set_io_engine(&mut self, engine: IOEngine) -> &mut Self {
        self.engine = engine;
        self
    }

    #[tokio::main]
    pub async fn run(&mut self, _config: &Config) -> () {
        let l = super::LUA_SCOPE.lock().unwrap();
        let g = l.globals();

        g.set("radius_send", l.create_async_function(radius_send).unwrap()).unwrap();

        let workload = g.get::<_, Table>("workload").unwrap();
        let imsi_range: String = workload.get::<_, String>("imsi_range").unwrap();
        let imsis = imsi_range.split('-').collect::<Vec<&str>>();
        let mut imsi_start = BigInt::from_str(imsis[0]).unwrap();
        let imsi_end = BigInt::from_str(imsis[1]).unwrap();

        loop {
            if imsi_start == imsi_end {
                break;
            }
            imsi_start += 1;

            let imsi = imsi_start.to_string();

            tokio::spawn(async move {
                let l = super::LUA_SCOPE.lock().unwrap();
                let g = l.globals();

                let state = l.create_table().unwrap();
                let _ = state.set("3GPP-IMSI", imsi);

                let run_cb = g.get::<_, Function>("run").unwrap();
                run_cb.call::<_, ()>(state).unwrap();
            });
        }

        // TODO: remove this as now it is only for testing the event loop
        sleep(Duration::from_millis(100)).await;
    }
}

// TODO: this function should be moved out of here, probably to libwl:: lua ns
async fn radius_send<'a>(_lua: &Lua, data: (String, mlua::Table<'a>, String)) -> mlua::Result<()> {
    let (server, packet, secret) = data;

    for pair in packet.pairs::<Value, Value>() {
        let (attr_name, attr_value) = pair.unwrap();
        let attr = match attr_name {
            Value::String(s) => {
                s.to_str().unwrap().to_string()
            },
            _ => {
                // TODO: something should be better than panic here
                panic!("Wrong attribute");
            }
        };

        let val = match attr_value {
            Value::String(s) => {
                s.to_str().unwrap().to_string()
            },
            Value::Integer(i) => {
                i.to_string()
            },
            Value::Function(f) => {
                // TODO: this is on_response cb, so we should fetch and call it properly
                "".to_string()
            }
            _ => {
                // TODO: something should be better than panic here
                panic!("Wrong attribute");
            }
        };

        // println!("{:?}", radius::RADIUS_DICTIONARIES.lock().unwrap()[&attr]);
    }
    Ok(())
}
