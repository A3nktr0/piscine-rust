pub fn num_to_ordinal(x: u32) -> String {
    match x.rem_euclid(10) {
        1 if x != 11 => format!("{}st", x),
        2 if x != 12 => format!("{}nd", x),
        3 if x != 13 => format!("{}rd", x),
        _ => format!("{}th", x),
    }
}
