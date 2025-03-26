impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        if k == 1 {
            return s.len() as i32;
        }

        let s = s.as_bytes();
        let k = k as usize;
        let mut dp = vec![0; s.len()];

        for i in k - 1..s.len() {
            dp[i] = dp[i - 1];
            if (0..k / 2).all(|j| s[i - k + 1 + j] == s[i - j]) {
                dp[i] = dp[i].max(dp[i.saturating_sub(k)] + 1);
            } else if i >= k && (0..(k + 1) / 2).all(|j| s[i - k + j] == s[i - j]) {
                dp[i] = dp[i].max(dp[i.saturating_sub(k + 1)] + 1);
            }
        }

        *dp.last().unwrap()
    }
}
