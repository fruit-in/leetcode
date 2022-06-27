use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            ret += count.get(&(num + k)).unwrap_or(&0);
            ret += count.get(&(num - k)).unwrap_or(&0);
            *count.entry(num).or_insert(0) += 1;
        }

        ret
    }
}
