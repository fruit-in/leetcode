impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        for i in 1..n {
            for j in 0..n - i {
                dp[j][j + i] = (prefix_sum[j + i + 1] - prefix_sum[j + 1] - dp[j + 1][j + i])
                    .max(prefix_sum[j + i] - prefix_sum[j] - dp[j][j + i - 1]);
            }
        }

        dp[0][n - 1]
    }
}
