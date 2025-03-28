use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut dp = vec![i64::MAX; nums.len() + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            let mut count = HashMap::new();
            let mut trimmed_length = 0;

            for j in (0..i).rev() {
                *count.entry(nums[j]).or_insert(0) += 1;
                trimmed_length += match count.get(&nums[j]).unwrap() {
                    &1 => 0,
                    &2 => 2,
                    _ => 1,
                };
                dp[i] = dp[i].min(dp[j] + k + trimmed_length);
            }
        }

        *dp.last().unwrap() as i32
    }
}
