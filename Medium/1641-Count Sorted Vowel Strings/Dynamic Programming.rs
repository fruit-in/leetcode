impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [1; 5];

        for _ in 1..n {
            dp[1] += dp[0];
            dp[2] += dp[1];
            dp[3] += dp[2];
            dp[4] += dp[3];
        }

        dp.iter().sum()
    }
}
