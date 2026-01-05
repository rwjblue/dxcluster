pub fn comment(input: &str) -> String {
    input.trim().replace('\n', " ")
}

pub fn callsign(input: &str) -> String {
    input.trim().to_uppercase()
}

pub(crate) fn trim_units(input: &str) -> String {
    input
        .trim()
        .trim_end_matches(|c: char| c.is_ascii_alphabetic())
        .to_string()
}
