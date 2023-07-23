use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut count = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;
        count.insert(0, 1);

        for num in nums {
            sum += num;
            ret += count.get(&(sum - goal)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
