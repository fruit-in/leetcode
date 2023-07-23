use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut is_lonely = HashMap::new();

        for &n in &nums {
            is_lonely
                .entry(n)
                .and_modify(|b| *b = false)
                .or_insert(true);
            is_lonely.insert(n + 1, false);
            is_lonely.insert(n - 1, false);
        }

        nums.into_iter().filter(|n| is_lonely[n]).collect()
    }
}
