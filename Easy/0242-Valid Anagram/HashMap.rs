use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        let mut map = HashMap::new();
        for i in 0..s_vec.len() {
            *map.entry(s_vec[i]).or_insert(0) += 1;
            *map.entry(t_vec[i]).or_insert(0) -= 1;
        }
        map.values().filter(|x| **x != 0).count() == 0
    }
}
