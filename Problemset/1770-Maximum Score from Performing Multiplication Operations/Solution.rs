impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![i32::MIN; m + 1]; m + 1];
        dp[0][0] = 0;

        for i in 1..=m {
            for j in 0..i {
                dp[j + 1][i - 1 - j] =
                    dp[j + 1][i - 1 - j].max(dp[j][i - 1 - j] + multipliers[i - 1] * nums[j]);
                dp[j][i - j] =
                    dp[j][i - j].max(dp[j][i - 1 - j] + multipliers[i - 1] * nums[n + j - i]);
            }
        }

        (0..=m).map(|i| dp[i][m - i]).max().unwrap()
    }
}
