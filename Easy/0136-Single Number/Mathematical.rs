use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.clone().drain(..).collect();
        let sum1: i32 = set.iter().sum();
        let sum2: i32 = nums.iter().sum();
        2 * sum1 - sum2
    }
}
