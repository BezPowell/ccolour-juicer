use crate::extract::ColourMatch;
use std::collections::HashMap;

pub fn detect_duplicates(map: &HashMap<String, ColourMatch>) -> Vec<ColourMatch> {
    let mut colours: Vec<ColourMatch> = map.iter().map(|x| x.1.clone()).collect();
    colours.sort_by(|a, b| b.count.cmp(&a.count));

    let mut new_colours = Vec::new();
    for (_string, colour) in map {
        let mut new = colour.clone();

        // Check if duplicate colour exists.
        if let Some(duplicate) = colours.iter().find(|x| x.colour == colour.colour) {
            // Only report duplicate is this is not the most common occurence of colour
            if duplicate.string != colour.string {
                new.duplicate = Some(duplicate.string.clone());
            }
        }

        // Push checking colour to new array
        new_colours.push(new);
    }

    // Sort new array, reverse
    new_colours.sort_by(|a, b| b.count.cmp(&a.count));

    new_colours
}
