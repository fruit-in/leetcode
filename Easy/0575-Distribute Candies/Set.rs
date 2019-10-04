use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let sister: HashSet<_> = candies.iter().collect();
        sister.len().min(candies.len() / 2) as i32
    }
}
