use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = HashMap::new();
        let mut max = 0;
        let mut ret = 0;

        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                *count.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }

        count.into_iter().map(|(k, v)| (v, k)).max().unwrap().1
    }
}
