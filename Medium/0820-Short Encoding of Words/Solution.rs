use std::collections::HashSet;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let words = words.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut suffixes = HashSet::new();

        for w in &words {
            for i in 1..w.len() {
                suffixes.insert(w.get(i..).unwrap());
            }
        }

        words
            .difference(&suffixes)
            .map(|w| w.len() as i32 + 1)
            .sum()
    }
}
