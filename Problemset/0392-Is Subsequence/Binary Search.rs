use std::collections::HashMap;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut char_index = HashMap::new();
        for (i, c) in t.chars().enumerate() {
            char_index.entry(c).or_insert(Vec::new()).push(i);
        }

        let mut prev = 0;
        for c in s.chars() {
            if let Some(v) = char_index.get(&c) {
                match v.binary_search(&prev) {
                    Ok(i) => prev += 1,
                    Err(i) => {
                        if i == v.len() {
                            return false;
                        }
                        prev = v[i] + 1;
                    },
                };
            } else {
                return false;
            }
        }
        true
    }
}
