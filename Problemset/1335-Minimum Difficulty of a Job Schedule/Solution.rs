impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;

        if n < d {
            return -1;
        }

        let mut dp = vec![vec![i32::MAX; n + 1]; d + 1];
        let mut max_difficulty = 0;

        for i in 0..=n - d {
            max_difficulty = max_difficulty.max(job_difficulty[i]);
            dp[1][i + 1] = max_difficulty;
        }

        for i in 1..d {
            for j in i..=n + i - d {
                max_difficulty = 0;

                for k in 1..=n + i + 1 - j - d {
                    max_difficulty = max_difficulty.max(job_difficulty[j + k - 1]);
                    dp[i + 1][j + k] = dp[i + 1][j + k].min(dp[i][j] + max_difficulty);
                }
            }
        }

        dp[d][n]
    }
}
