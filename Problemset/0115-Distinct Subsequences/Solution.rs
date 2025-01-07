impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; t.len()]; s.len()];

        for i in 0..s.len() {
            for j in 0..t.len().min(i + 1) {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j];
                }
                if s[i] == t[j] {
                    if j == 0 {
                        dp[i][j] += 1;
                    } else {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[s.len() - 1][t.len() - 1]
    }
}
