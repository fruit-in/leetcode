use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('0'))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
