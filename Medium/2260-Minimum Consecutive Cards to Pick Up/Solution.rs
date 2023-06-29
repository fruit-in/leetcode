use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut indices = HashMap::new();
        let mut ret = usize::MAX;

        for i in 0..cards.len() {
            if let Some(j) = indices.insert(cards[i], i) {
                ret = ret.min(i - j + 1);
            }
        }

        if ret == usize::MAX {
            return -1;
        }

        ret as i32
    }
}
