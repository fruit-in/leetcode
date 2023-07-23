use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                *counter.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }

        counter.values().map(|c| c * (c - 1) * 4).sum()
    }
}
