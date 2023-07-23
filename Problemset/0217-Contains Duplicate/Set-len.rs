use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }
}
