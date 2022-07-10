use super::conf::Config;
use super::LUA_SCOPE;
use mlua::{Function, Table};
use std::thread;
use std::time;

pub fn run(_config: &Config) {
    // {
    //     let lua = LUA_SCOPE.lock().unwrap();
    //     let globals = lua.globals();
    //     let f = globals.get::<_, Function>("run").unwrap();
    //     f.call::<_, ()>("test").unwrap();
    // }

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
