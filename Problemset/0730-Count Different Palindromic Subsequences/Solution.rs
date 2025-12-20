impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0_i32; n]; n];

        for i in (0..n).rev() {
            let mut first = [n; 4];
            let mut last = [n; 4];

            for j in i..n {
                let k = (s[j] - b'a') as usize;
                dp[i][j] = dp[i][j.saturating_sub(1)];

                if first[k] == n {
                    dp[i][j] += 1;
                    first[k] = j;
                } else if first[k] == last[k] {
                    dp[i][j] += dp[first[k] + 1][j - 1] + 1;
                } else {
                    dp[i][j] += dp[first[k] + 1][j - 1] - dp[first[k] + 1][last[k] - 1];
                }

                dp[i][j] = dp[i][j].rem_euclid(1_000_000_007);
                last[k] = j;
            }
        }

        dp[0][n - 1]
    }
}
