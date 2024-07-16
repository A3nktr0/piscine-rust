#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }

    let atbash: String = original
        .chars()
        .map(|c| match c {
            'a'..='z' => ('z' as u8 - (c as u8 - 'a' as u8)) as char,
            'A'..='Z' => ('Z' as u8 - (c as u8 - 'A' as u8)) as char,
            _ => c,
        })
        .collect();

    if original.len() != ciphered.len() || atbash != ciphered {
        return Some(Err(CipherError::new(false, atbash.to_string())));
    }

    Some(Ok(true))
}
