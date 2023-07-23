use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let nums_set = nums.iter().collect::<HashSet<_>>();

        *nums
            .iter()
            .filter(|&&x| nums_set.contains(&-x))
            .max()
            .unwrap_or(&-1)
    }
}
