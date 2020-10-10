use css_color_parser2::Color;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColourMatch {
    pub count: u32,
    pub colour: Color,
    pub string: String,
    pub duplicate: Option<String>,
}

pub fn extract_hex(text: &str) -> HashMap<String, ColourMatch> {
    // Setup regex string
    let hex_match =
        Regex::new(r"\#(([0-9a-fA-F]{2}){4}|([0-9a-fA-F]{2}){3}|([0-9a-fA-F]){3})").unwrap();
    let mut map: HashMap<String, ColourMatch> = HashMap::new();

    // Add each match to hashmap
    for cap in hex_match.captures_iter(text) {
        if let Some(colour) = map.get_mut(&cap[0]) {
            // Colour already found, increment counter
            colour.count += 1;
        } else {
            // Colour not already found, add to map
            let colour = cap[0].parse::<Color>().unwrap();
            map.insert(
                cap[0].to_string(),
                ColourMatch {
                    count: 1,
                    colour,
                    string: cap[0].to_string(),
                    duplicate: None,
                },
            );
        }
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex() {
        let css = "color: #010101; background-color: rgb(255,255,255); border-color: #zvab00; outline-color: #777; color: #010101;";
        let colours = extract_hex(css);
        assert!(colours.contains_key("#010101") && colours.contains_key("#777"));
        assert_eq!(colours.len(), 2);
    }
}
