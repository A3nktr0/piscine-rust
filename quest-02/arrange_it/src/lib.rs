pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort_by_key(|word| {
        word.chars().find(|c| c.is_digit(10)).unwrap()
    });

    words.iter().map(|word| {
        word.chars().filter(|c| !c.is_digit(10)).collect::<String>()
    }).collect::<Vec<String>>().join(" ")
}