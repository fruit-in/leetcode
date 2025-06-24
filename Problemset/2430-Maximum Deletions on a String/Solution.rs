impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0; n]; n];
        let mut dp = vec![1; n];

        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = 1;
                    if j + 1 < n {
                        lcp[i][j] += lcp[i + 1][j + 1];
                    }
                }
            }
        }

        for i in (0..n - 1).rev() {
            for j in i + 1..=(n + i) / 2 {
                if lcp[i][j] >= j - i {
                    dp[i] = dp[i].max(1 + dp[j]);
                }
            }
        }

        dp[0]
    }
}
