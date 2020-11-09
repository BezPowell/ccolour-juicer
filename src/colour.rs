use css_color_parser2::{Color, ColorParseError};
#[derive(Debug, Clone, PartialEq)]
pub struct ColourMatch {
    count: u32,
    colour: Color,
    string: String,
    duplicate: Option<String>,
}

impl ColourMatch {
    pub fn new(string: String) -> Result<ColourMatch, ColorParseError> {
        let colour = string.parse::<Color>()?;

        Ok(ColourMatch {
            count: 1,
            colour,
            string,
            duplicate: None,
        })
    }

    pub fn string(&self) -> &String {
        &self.string
    }

    pub fn colour(&self) -> &Color {
        &self.colour
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn duplicate(&self) -> &Option<String> {
        &self.duplicate
    }

    pub fn set_duplicate(&mut self, string: &String) {
        self.duplicate = Some(string.clone());
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}
