use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut counter = HashMap::new();

        for ch in s.clone() {
            *counter.entry(ch).or_insert(0) -= 1;
        }

        s.sort_unstable();
        s.sort_by_key(|k| counter.get(&k));

        s.iter().collect()
    }
}
