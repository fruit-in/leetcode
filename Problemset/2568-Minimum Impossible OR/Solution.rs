use std::collections::HashSet;

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();

        for i in 0..32 {
            if !nums.contains(&(1 << i)) {
                return 1 << i;
            }
        }

        unreachable!()
    }
}
