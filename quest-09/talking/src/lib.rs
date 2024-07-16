pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let (upper, question) = (
        text.chars().any(|c| c.is_alphabetic()) && text.chars().all(|c| !c.is_lowercase()),
        text.trim().ends_with('?'),
    );

    match (upper, question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
