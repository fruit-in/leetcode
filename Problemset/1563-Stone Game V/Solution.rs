impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stone_value[i];
        }

        for i in 2..=n {
            for j in 0..n - i + 1 {
                let mut left_sum = 0;
                let mut right_sum = prefix_sum[j + i] - prefix_sum[j];

                for k in j..j + i - 1 {
                    left_sum += stone_value[k];
                    right_sum -= stone_value[k];

                    if left_sum <= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(left_sum + dp[j][k]);
                    }
                    if left_sum >= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(right_sum + dp[k + 1][j + i - 1]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}
