impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut dp = [1; 10];

        for _ in 1..n {
            let mut dp_ = [0; 10];
            for i in 0..10 {
                dp_[i] = match i {
                    0 => dp[4] + dp[6],
                    1 | 3 => dp[7 - i] + dp[8],
                    2 | 8 => dp[9 - i] + dp[11 - i],
                    4 | 6 => (dp[0] + dp[7 - i]) % 1_000_000_007 + dp[13 - i],
                    7 | 9 => dp[2] + dp[13 - i],
                    _ => 0,
                } % 1_000_000_007;
            }
            dp = dp_;
        }

        dp.iter().fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
