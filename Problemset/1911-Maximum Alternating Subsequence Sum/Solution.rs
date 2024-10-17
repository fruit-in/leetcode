impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut dp = vec![[0; 2]; nums.len() + 1];

        for i in 0..nums.len() {
            dp[i + 1][0] = dp[i][0].max(dp[i][1] + nums[i] as i64);
            dp[i + 1][1] = dp[i][1].max(dp[i][0] - nums[i] as i64);
        }

        dp.last().unwrap()[0]
    }
}
