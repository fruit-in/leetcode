impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[0, 0]; n];
        dp[0] = [1, 1];

        for i in 1..n {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % 1_000_000_007;
            dp[i][1] = dp[i - 1][0];
        }

        (((dp[n - 1][0] + dp[n - 1][1]) as i64).pow(2) % 1_000_000_007) as i32
    }
}
