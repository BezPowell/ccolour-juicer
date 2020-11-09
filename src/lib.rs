use std::fs::File;
use std::{error::Error, io::Read};

mod extract;
use colour::ColourMatch;
use extract::ColourExtract;
mod colour;
mod constants;
mod helpers;

pub struct Config {
    pub input: String,
    pub csv: bool,
    pub skip_named: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        // Get input file
        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file"),
        };

        // Process any other flags
        let mut csv = false;
        let mut skip_named = false;

        for flag in args {
            match flag.as_str() {
                "--csv" | "-C" => {
                    csv = true;
                }
                "--skip-named" | "-S" => {
                    skip_named = true;
                }
                _s => {
                    return Err("Unrecognised argument");
                }
            }
        }

        Ok(Config {
            input,
            csv,
            skip_named,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.input)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let colours = extract(&contents, config.skip_named);
    let output = print_out(&colours, config.csv);

    println!("{}", output);

    Ok(())
}

fn extract(css: &str, skip_named: bool) -> Vec<ColourMatch> {
    if skip_named {
        ColourExtract::new(css).extract_computed().build()
    } else {
        ColourExtract::new(css)
            .extract_computed()
            .extract_named()
            .build()
    }
}

fn print_out(colours: &Vec<ColourMatch>, csv: bool) -> String {
    // Print out
    let mut output = if csv {
        format!("{0},{1},{2}\n", "Colour", "Count", "Duplicate")
    } else {
        format!(
            "{0: <22} | {1: <10} | {2: <10}\n",
            "Colour", "Count", "Duplicate"
        )
    };

    for colour in colours {
        let duplicate = if let Some(dup) = colour.duplicate() {
            dup.clone()
        } else {
            "".to_string()
        };

        let line = if csv {
            format!(
                "\"{0}\",{1},\"{2}\"\n",
                colour.string(),
                colour.count(),
                duplicate
            )
        } else {
            format!(
                "{0: <22} | {1: <10} | {2: <10}\n",
                colour.string(),
                colour.count(),
                duplicate
            )
        };

        output.push_str(&line);
    }

    output
}

#[cfg(test)]
mod test {
    use crate::{extract, print_out};

    const TEST_STR: &str = "em { color: #fff; background-color: #fff; border-color: #fff; }
    em { color: white; background-color: white; }
    em { color: rgb(0, 0, 0); }";

    const CSV_STR: &str =
        "Colour,Count,Duplicate\n\"#fff\",3,\"\"\n\"white\",2,\"#fff\"\n\"rgb(0, 0, 0)\",1,\"\"\n";

    const PRETTY_STR: &str = "Colour                 | Count      | Duplicate \n#fff                   | 3          |           \nrgb(0, 0, 0)           | 1          |           \n";

    #[test]
    fn skip_named() {
        let matches: Vec<String> = extract(TEST_STR, true)
            .iter()
            .map(|x| x.string().clone())
            .collect();

        assert!(matches.contains(&"#fff".to_string()));
        assert!(!matches.contains(&"white".to_string()));
    }

    #[test]
    fn include_named() {
        let matches: Vec<String> = extract(TEST_STR, false)
            .iter()
            .map(|x| x.string().clone())
            .collect();

        assert!(matches.contains(&"#fff".to_string()));
        assert!(matches.contains(&"white".to_string()));
    }

    #[test]
    fn format_csv() {
        let matches = extract(TEST_STR, false);
        let output = print_out(&matches, true);

        assert_eq!(output, CSV_STR);
    }

    #[test]
    fn format_pretty() {
        let matches = extract(TEST_STR, true);
        let output = print_out(&matches, false);

        assert_eq!(output, PRETTY_STR);
    }
}
