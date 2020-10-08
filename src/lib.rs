use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod extract;
use extract::extract_hex;

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

    let colours = extract_hex(&contents);

    for line in colours {
        println!("{}", line);
    }

    Ok(())
}
