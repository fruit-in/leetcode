use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for word in &words {
            let x = word.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')));

            count.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }

        count.values().map(|c| c * (c - 1) / 2).sum()
    }
}
