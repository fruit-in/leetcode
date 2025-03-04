impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![0; s.len()]; s.len()];

        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        for size in 2..=s.len() {
            for i in 0..s.len() - size + 1 {
                dp[i][i + size - 1] =
                    dp[i + 1][i + size - 2] + (s[i] == s[i + size - 1]) as i32 * 2;
                dp[i][i + size - 1] = dp[i][i + size - 1]
                    .max(dp[i][i + size - 2])
                    .max(dp[i + 1][i + size - 1]);
            }
        }

        dp[0][s.len() - 1]
    }
}
