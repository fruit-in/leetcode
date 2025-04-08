use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = HashMap::new();
        let mut pairs = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *count.entry(nums[j]).or_insert(0) += 1;
            pairs += count[&nums[j]] - 1;

            while pairs >= k {
                *count.get_mut(&nums[i]).unwrap() -= 1;
                pairs -= count[&nums[i]];
                i += 1;
            }

            ret += i as i64;
        }

        ret
    }
}
