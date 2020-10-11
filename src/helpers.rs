use crate::colour::ColourMatch;

pub fn detect_duplicates(colours: &mut Vec<ColourMatch>) {
    let match_colours = colours.clone();
    for colour in colours {
        // Check if duplicate colour exists.
        if let Some(duplicate) = match_colours.iter().find(|x| x.colour() == colour.colour()) {
            // Only report duplicate is this is not the most common occurence of colour
            if duplicate.string() != colour.string() {
                colour.set_duplicate(duplicate.string());
            }
        }
    }
}
