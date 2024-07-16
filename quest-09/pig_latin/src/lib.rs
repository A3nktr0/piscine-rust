pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let f = text.chars().next().unwrap();
    let mut res = String::new();

    if vowels.contains(&f) {
        res = text.to_string();
        res.push_str("ay");
    } else {
        if text[1..=2] == "qu".to_string() {
            res = (&text[3..]).to_string();
            res.push_str(&text[0..=2]);
        } else {
            let tmp = text
                .chars()
                .take_while(|&c| !vowels.contains(&c))
                .collect::<String>();
            res = (&text[tmp.len()..]).to_string();
            res.push_str(&tmp);
        }
        res.push_str("ay");
    }
    res
}
