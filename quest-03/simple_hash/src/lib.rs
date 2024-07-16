use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    words.iter().fold(HashMap::new(), |mut acc, &word|{
        let count = acc.entry(word).or_insert(0);
        *count += 1;
        acc
    })
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}