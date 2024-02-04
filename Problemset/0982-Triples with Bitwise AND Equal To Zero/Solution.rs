use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                *count.entry(nums[i] & nums[j]).or_insert(0) += 1;
            }
        }

        for k in 0..nums.len() {
            for (x, c) in count.iter() {
                if x & nums[k] == 0 {
                    ret += c;
                }
            }
        }

        ret
    }
}
