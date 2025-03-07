impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut dp = vec![vec![i32::MAX; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = 0;

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i > 0 && j > 0 && s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + s1[i - 1] as i32);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] + s2[j - 1] as i32);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}
