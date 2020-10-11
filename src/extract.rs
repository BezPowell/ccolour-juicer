use crate::colour::ColourMatch;
use crate::helpers::detect_duplicates;
use regex::Regex;
use std::collections::HashMap;

pub struct ColourExtract<'a> {
    file: &'a str,
    colours: HashMap<String, ColourMatch>,
}

impl<'a> ColourExtract<'a> {
    pub fn new(file: &str) -> ColourExtract {
        ColourExtract {
            file,
            colours: HashMap::new(),
        }
    }

    pub fn extract_hex(mut self) -> ColourExtract<'a> {
        // Setup regex string
        let hex_match = Regex::new(r"\#(([0-9a-fA-F]{2}){3}|([0-9a-fA-F]{3}))").unwrap();

        // Match colours
        self.match_colours(hex_match);

        self
    }

    pub fn build(self) -> Vec<ColourMatch> {
        // Convert colours to vec and sort.
        let mut colours: Vec<ColourMatch> = self.colours.iter().map(|x| x.1.clone()).collect();
        colours.sort_by(|a, b| b.count().cmp(&a.count()));

        // Detect duplicates
        detect_duplicates(&mut colours);

        colours
    }

    fn match_colours(&mut self, match_string: Regex) {
        // Add each match to hashmap
        for cap in match_string.captures_iter(self.file) {
            if let Some(colour) = self.colours.get_mut(&cap[0]) {
                // Colour already found, increment counter
                colour.increment();
            } else {
                // Colour not already found, add to map
                self.colours
                    .insert(cap[0].to_string(), ColourMatch::new(cap[0].to_string()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex() {
        let css = "color: #010101; background-color: rgb(255,255,255); border-color: #zvab00; outline-color: #777; color: #010101;";
        let extract = ColourExtract::new(css).extract_hex();
        assert!(extract.colours.contains_key("#010101") && extract.colours.contains_key("#777"));
        assert_eq!(extract.colours.len(), 2);
    }
}
