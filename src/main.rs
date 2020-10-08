extern crate ccolour_juicer;

use std::env;
use std::process;

use ccolour_juicer::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
