use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn extract_hex(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HEX_MATCH: Regex =
            Regex::new(r"\#(([0-9a-fA-F]{2}){4}|([0-9a-fA-F]{2}){3}|([0-9a-fA-F]){3})").unwrap();
    }

    HEX_MATCH.find_iter(text).map(|mat| mat.as_str()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex() {
        let css = "color: #010101; background-color: rgb(255,255,255); border-color: #zvab00; outline-color: #777; color: #010101;";
        let colours = extract_hex(css);
        assert!(colours.contains("#010101") && colours.contains("#777"));
        assert_eq!(colours.len(), 2);
    }
}
