extern crate libwl;
extern crate radius;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::path::{PathBuf};

use radius::dictionary::{
    load_dictionaries,
    DictionarySet
};

use getopts::{HasArg, Occur, Options};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage(opts: Options) {
    let brief = format!("Usage: whirl -s SCRIPT [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.opt(
        "d",
        "dictionaries",
        "path to the directory with RADIUS dictionaries",
        "DIR",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.opt(
        "s",
        "script",
        "path to the script with a scenario",
        "SCRIPT",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.optflag("h", "help", "display this help text and exit");
    opts.optflag("v", "version", "display version of whirl");

    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("Error: {0}", f.to_string());
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        process::exit(0);
    }

    if matches.opt_present("v") {
        println!("whirl - {0}", VERSION);
        process::exit(0);
    }
    
    let script = matches.opt_str("s");
    if script == None {
        print_usage(opts);
        process::exit(1);
    }

    // read the scenario script
    let mut script_file = File::open(script.unwrap()).expect("TEST");
    let mut script = String::new();
    script_file
        .read_to_string(&mut script)
        .expect("could not read lua script");

    // load radius dictionaries
    let dicts_dir = matches.opt_str("d").and_then(|dir| {
        let mut p = PathBuf::new();
        p.push(dir);
        Some(p)
    });
    match load_dictionaries(DictionarySet::All, dicts_dir) {
        Ok(_) => { }
        Err(err) => {
            eprintln!("Error: Can't load RADIUS dictionaries - {:?}", err);
            process::exit(1);
        }
    }

    // load scripts and start execution
    libwl::load(script.as_ref());

    process::exit(0);
}
