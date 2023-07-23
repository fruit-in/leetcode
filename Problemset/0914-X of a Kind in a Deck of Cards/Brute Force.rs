use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in &deck {
            *map.entry(n).or_insert(0) += 1;
        }

        for x in (2..=(deck.len() / map.len())).filter(|x| deck.len() % x == 0) {
            if map.values().all(|v| v % x == 0) {
                return true;
            }
        }

        false
    }
}
