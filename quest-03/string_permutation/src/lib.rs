pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.chars().collect::<Vec<char>>();
    let mut s2 = s2.chars().collect::<Vec<char>>();
    s1.sort();
    s2.sort();
    s1 == s2
}