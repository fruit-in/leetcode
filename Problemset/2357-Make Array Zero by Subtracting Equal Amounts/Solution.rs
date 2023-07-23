use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&x| x > 0)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
