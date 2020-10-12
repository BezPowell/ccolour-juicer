use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod extract;
use extract::ColourExtract;
mod colour;
mod constants;
mod helpers;

pub struct Config {
    pub input: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file"),
        };

        Ok(Config { input })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.input)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let colours = ColourExtract::new(&contents)
        .extract_computed()
        .extract_named()
        .build();

    // Print out
    println!(
        "{0: <20} | {1: <10} | {2: <10}",
        "Colour", "Count", "Duplicate"
    );
    for colour in colours {
        let duplicate = if let Some(dup) = colour.duplicate() {
            dup.clone()
        } else {
            "".to_string()
        };

        println!(
            "{0: <20} | {1: <10} | {2: <10}",
            colour.string(),
            colour.count(),
            duplicate
        );
    }

    Ok(())
}
