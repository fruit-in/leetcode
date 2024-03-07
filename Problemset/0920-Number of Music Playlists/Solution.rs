impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let n = n as usize;
        let goal = goal as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; n + 1]; goal + 1];
        dp[0][0] = 1;

        for i in 0..goal {
            for j in 0..=n.min(i) {
                if j < n {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j] * (n - j)) % 1_000_000_007;
                }
                if j > k {
                    dp[i + 1][j] = (dp[i + 1][j] + dp[i][j] * (j - k)) % 1_000_000_007;
                }
            }
        }

        dp[goal][n] as i32
    }
}
