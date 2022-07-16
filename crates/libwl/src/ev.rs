use super::conf::Config;
use mlua::{Table};
use std::thread;
use std::time;

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

    pub fn run(&mut self, _config: &Config) -> () {
        thread::spawn(move || {
            let l = super::LUA_SCOPE.lock().unwrap();
            let g = l.globals();
            let workload = g.get::<_, Table>("workload").unwrap();
            println!("{:?}", workload.contains_key::<_>("imsi_range").unwrap());
        });

        thread::spawn(move || {
            let l = super::LUA_SCOPE.lock().unwrap();
            let g = l.globals();
            let workload = g.get::<_, Table>("workload").unwrap();
            println!("{:?}", workload.contains_key::<_>("imsi_range").unwrap());
        });

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
}
