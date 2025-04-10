impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![[0; 3]; nums.len() + 1];

        for i in 0..nums.len() {
            dp[i + 1] = dp[i];
            match nums[i] {
                0 => dp[i + 1][0] = (dp[i][0] * 2 + 1) % MOD,
                1 => dp[i + 1][1] = (dp[i][1] * 2 % MOD + dp[i][0]) % MOD,
                _ => dp[i + 1][2] = (dp[i][2] * 2 % MOD + dp[i][1]) % MOD,
            }
        }

        dp[nums.len()][2]
    }
}
