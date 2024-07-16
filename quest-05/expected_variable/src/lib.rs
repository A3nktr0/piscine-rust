pub use case;
pub use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if compared.is_empty() || compared.contains(" ") || compared.contains("-") {
        return None;
    }

    let distance = edit_distance(&compared.to_lowercase(), &expected.to_lowercase());
    let percent = ((1.0 - distance as f64 / expected.len() as f64) * 100.0).round();
    if percent > 50.0 {
        Some(format!("{}%", percent))
    } else {
        None
    }
}
