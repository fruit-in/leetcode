impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let min_profit = min_profit as usize;
        let mut dp = vec![vec![0; n + 1]; min_profit + 1];
        dp[0][n] = 1;

        for i in 0..group.len() {
            for p in (0..=min_profit).rev() {
                for m in group[i] as usize..=n {
                    dp[min_profit.min(p + profit[i] as usize)][m - group[i] as usize] += dp[p][m];
                    dp[min_profit.min(p + profit[i] as usize)][m - group[i] as usize] %=
                        1_000_000_007;
                }
            }
        }

        dp[min_profit]
            .iter()
            .fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
