use std::collections::HashMap;
use std::str;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut counter = HashMap::new();

        for w in s.windows(10) {
            *counter.entry(str::from_utf8(w).unwrap()).or_insert(0) += 1;
        }

        counter
            .into_iter()
            .filter(|(_, c)| *c > 1)
            .map(|(s, _)| s.to_string())
            .collect()
    }
}
