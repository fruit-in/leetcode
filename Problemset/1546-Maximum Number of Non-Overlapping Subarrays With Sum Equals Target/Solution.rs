use std::collections::HashMap;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut last_index = HashMap::from([(0, 0)]);
        let mut max_count = vec![0; nums.len() + 1];

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
            if let Some(&j) = last_index.get(&(prefix_sum[i + 1] - target)) {
                max_count[i + 1] = max_count[j] + 1;
            }
            max_count[i + 1] = max_count[i + 1].max(max_count[i]);
            last_index.insert(prefix_sum[i + 1], i + 1);
        }

        *max_count.last().unwrap()
    }
}
