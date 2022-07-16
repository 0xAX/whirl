extern crate libwl;
extern crate radius;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use radius::dictionary::{load_dictionaries, DictionarySet};

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
        "path to the directory with RADIUS and Diameter dictionaries",
        "DIR",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.opt(
        "e",
        "engine",
        "I/O engine",
        "ENGINE",
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
    opts.opt(
        "t",
        "threads",
        "number of threads to use",
        "THREADS",
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

    let mut io_engine = libwl::ev::IOEngine::WIO;
    let engine = matches.opt_str("e");
    match engine {
        None => {}
        Some(e) => {
            if e == "wio" {
                io_engine = libwl::ev::IOEngine::WIO;
            }

            if e == "tokio" {
                io_engine = libwl::ev::IOEngine::Tokio;
            }
        }
    }

    let mut threads = matches.opt_str("t");
    if threads == None {
        threads = Some("4".to_string());
    }
    let threads = match threads.unwrap().parse::<u8>() {
        Ok(t) => t,
        Err(_e) => {
            eprintln!("Error: -t/--threads should have numeric value");
            process::exit(1);
        }
    };

    let script = matches.opt_str("s");
    if script == None {
        print_usage(opts);
        process::exit(1);
    }

    let script_file = File::open(script.unwrap());
    match script_file {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: Can't load script with scenario - {:?}", err);
            process::exit(1);
        }
    }

    let mut script = String::new();
    script_file.unwrap().read_to_string(&mut script).unwrap();

    let dicts_dir = matches.opt_str("d").and_then(|dir| {
        let mut p = PathBuf::new();
        p.push(dir);
        Some(p)
    });
    match load_dictionaries(DictionarySet::All, dicts_dir) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: Can't load RADIUS dictionaries - {:?}", err);
            process::exit(1);
        }
    }

    // load the scenario script, build event loop configuration
    // and start execution
    let sceneario = libwl::load(script.as_ref()).unwrap();
    let mut ev = libwl::ev::Ev::new();

    ev.set_threads(threads)
        .set_io_engine(io_engine)
        .run(&sceneario);

    process::exit(0);
}
