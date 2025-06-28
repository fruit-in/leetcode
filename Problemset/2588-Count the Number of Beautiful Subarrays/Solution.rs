use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut mask = 0;
        let mut mask_count = HashMap::from([(0, 1)]);
        let mut ret = 0;

        for &x in &nums {
            mask ^= x;
            ret += mask_count.get(&mask).unwrap_or(&0);
            *mask_count.entry(mask).or_insert(0) += 1;
        }

        ret
    }
}
