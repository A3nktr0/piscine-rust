pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();

    input.chars().for_each(|c| {
        if c.is_alphabetic() {
            let b: u8 = if c.is_lowercase() { b'a' } else { b'A' };
            let offset = c as u8 - b;
            let rot = (offset as i8 + key).rem_euclid(26) as u8 + b;
            res.push(rot as char);
        } else {
            res.push(c);
        }
    });
    res
}
