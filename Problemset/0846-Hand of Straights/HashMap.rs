use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let mut needs: HashMap<i32, Vec<i32>> = HashMap::new();
        hand.sort_unstable();

        for x in hand {
            if let Some(v) = needs.get_mut(&(x - 1)) {
                match v.pop() {
                    Some(1) => (),
                    Some(y) => needs.entry(x).or_insert(vec![]).push(y - 1),
                    None => needs.entry(x).or_insert(vec![]).push(group_size - 1),
                }
            } else if group_size > 1 {
                needs.entry(x).or_insert(vec![]).push(group_size - 1);
            }
        }

        needs.values().all(|v| v.is_empty())
    }
}
