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

    pub fn extract_computed(mut self) -> ColourExtract<'a> {
        // Setup regex string
        let computed_match = Regex::new(
            r"(#(?:[0-9a-fA-F]{2}){3}|(#[0-9a-f]{3})|(rgb|hsl)a?\((-?\d+%?[,\s]+){2,3}\s*[\d\.]+%?\))",
        )
        .unwrap();

        // Match colours
        self.match_colours(computed_match);

        self
    }

    pub fn extract_named(mut self) -> ColourExtract<'a> {
        // Setup regex string
        let named_match = Regex::new(
            r"(black|silver|gray|whitesmoke|maroon|red|purple|fuchsia|green|lime|olivedrab|yellow|navy|blue|teal|aquamarine|orange|aliceblue|antiquewhite|aqua|azure|beige|bisque|blanchedalmond|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|darkblue|darkcyan|darkgoldenrod|darkgray|darkgreen|darkgrey|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|gainsboro|ghostwhite|goldenrod|gold|greenyellow|grey|honeydew|hotpink|indianred|indigo|ivory|khaki|lavenderblush|lavender|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgreen|lightgrey|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|limegreen|linen|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|oldlace|olive|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|thistle|tomato|turquoise|violet|wheat|white|yellowgreen|rebeccapurple|cyan|magenta)\b",
        )
        .unwrap();

        // Match colours
        self.match_colours(named_match);

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
                let col_string = cap[0].to_string();
                match ColourMatch::new(cap[0].to_string()) {
                    Ok(colour) => {
                        self.colours.insert(col_string, colour);
                    }
                    Err(e) => eprintln!("Error parsing {}: {}", col_string, e),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_computed() {
        let extract = ColourExtract::new(COMPUTED_TEST).extract_computed();
        assert_eq!(extract.colours.get("#ff0000").unwrap().count(), 1);
        assert_eq!(extract.colours.get("rgb(255,0,0)").unwrap().count(), 2);
        assert_eq!(
            extract.colours.get("hsl(120, 100%, 50%)").unwrap().count(),
            2
        );
        assert_eq!(extract.colours.len(), 13);
    }

    #[test]
    fn test_named() {
        let extract = ColourExtract::new(NAMED_TEST).extract_named();
        assert_eq!(extract.colours.len(), 147);
    }
}
