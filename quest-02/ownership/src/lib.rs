pub fn first_subword(mut s: String) -> String {
    let mut i = 0;
    while i < s.len(){
        if (s.chars().nth(i).unwrap().is_uppercase() && i != 0) ||
        s.chars().nth(i).unwrap() == '_' {
            break;
        }
        i += 1;
    }
    s.truncate(i);
    s
}