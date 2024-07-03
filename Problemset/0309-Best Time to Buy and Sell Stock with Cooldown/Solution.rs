impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![(0, 0, i32::MIN, 0); n];
        dp[0].0 = -prices[0];

        for i in 1..n {
            dp[i].0 = dp[i - 1].3 - prices[i];
            dp[i].2 = dp[i - 1].0.max(dp[i - 1].2);
            dp[i].1 = 0.max(dp[i].2 + prices[i]);
            dp[i].3 = dp[i - 1].1.max(dp[i - 1].3);
        }

        dp[n - 1].1.max(dp[n - 1].3)
    }
}
