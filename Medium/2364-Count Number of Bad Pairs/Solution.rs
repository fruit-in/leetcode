use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            ret += i as i64 - count.get(&(i as i32 - nums[i])).unwrap_or(&0);
            count
                .entry(i as i32 - nums[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        ret
    }
}
