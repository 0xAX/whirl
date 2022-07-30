use super::conf::Config;
use mlua::{Table};
use num_bigint::BigInt;
use std::str::FromStr;

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

            let imsi = imsi_start.clone();

            tokio::spawn(async move {
                println!("{:?}", &imsi);
            });
        }
    }
}
