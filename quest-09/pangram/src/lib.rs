pub fn is_pangram(s: &str) -> bool {
    let mut alpha = [false; 26];
    for c in s.chars().filter(|c| c.is_ascii_alphabetic()) {
        alpha[(c.to_ascii_lowercase() as u8 - b'a') as usize] = true;
    }
    alpha.iter().all(|&b| b)
}
