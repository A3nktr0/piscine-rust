pub fn edit_distance(source: &str, target: &str) -> usize {
    let s_len = source.len();
    let t_len = target.len();

    let mut mv = vec![vec![0; t_len + 1]; s_len + 1];

    for i in 0..=s_len {
        mv[i][0] = i;
    }

    for j in 0..=t_len {
        mv[0][j] = j;
    }

    for (i, s_char) in source.chars().enumerate() {
        for (j, t_char) in target.chars().enumerate() {
            let cost = if s_char == t_char { 0 } else { 1 };

            mv[i + 1][j + 1] = *[
                mv[i][j + 1] + 1, // Cost of deletion
                mv[i + 1][j] + 1, // Cost of insertion
                mv[i][j] + cost, // Cost of substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
    }
    mv[s_len][t_len]
}
