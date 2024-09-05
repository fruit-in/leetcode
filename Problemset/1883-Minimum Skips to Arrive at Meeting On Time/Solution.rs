impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let speed = speed as i64;
        let hours_before = hours_before as i64 * speed;
        let n = dist.len();
        let mut dp = vec![vec![[i64::MAX; 2]; n + 1]; n + 1];
        dp[0][0] = [0; 2];

        for i in 0..n {
            for j in 0..=i {
                dp[i + 1][j][0] =
                    (dp[i][j][0].min(dp[i][j][1]) + dist[i] as i64 + speed - 1) / speed * speed;
                dp[i + 1][j + 1][1] = dp[i][j][0].min(dp[i][j][1]) + dist[i] as i64;
            }
        }

        for i in 0..=n {
            if dp[n][i][0].min(dp[n][i][1]) <= hours_before {
                return i as i32;
            }
        }

        -1
    }
}
