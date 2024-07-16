use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + std::cmp::Eq, U>(
    keys: &'a [T],
    values: &'a [U],
) -> HashMap<&'a T, &'a U> {
    keys.iter().zip(values.iter()).collect()
}
