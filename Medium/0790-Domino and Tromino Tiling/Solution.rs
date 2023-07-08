impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut dp = vec![(0, 0, 0); n as usize];
        dp[0].0 = 1;
        dp[1] = (1, 1, 1);

        for i in 0..n as usize - 1 {
            dp[i + 1].0 = (dp[i].0 + dp[i + 1].0) % 1_000_000_007;
            dp[i + 1].0 = (dp[i].1 + dp[i + 1].0) % 1_000_000_007;
            dp[i + 1].0 = (dp[i].2 + dp[i + 1].0) % 1_000_000_007;
            dp[i + 1].1 = (dp[i].2 + dp[i + 1].1) % 1_000_000_007;
            dp[i + 1].2 = (dp[i].1 + dp[i + 1].2) % 1_000_000_007;
            if i + 2 < n as usize {
                dp[i + 2].0 = (dp[i].0 + dp[i + 2].0) % 1_000_000_007;
                dp[i + 2].1 = (dp[i].0 + dp[i + 2].1) % 1_000_000_007;
                dp[i + 2].2 = (dp[i].0 + dp[i + 2].2) % 1_000_000_007;
            }
        }

        dp[n as usize - 1].0
    }
}
