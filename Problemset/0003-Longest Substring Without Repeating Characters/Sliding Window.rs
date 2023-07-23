use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut indices = HashMap::new();
        let mut ret = 0;
        let mut i = 0;

        for j in 0..s.len() {
            match indices.insert(s[j], j) {
                Some(x) => i = i.max(x + 1),
                None => (),
            }
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
