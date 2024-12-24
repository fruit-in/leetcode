impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let (low, high) = (low as usize, high as usize);
        let (zero, one) = (zero as usize, one as usize);
        let mut dp = vec![0; high + 1];
        let mut ret = 0;
        dp[0] = 1;

        for i in 0..=high {
            if i >= low {
                ret = (ret + dp[i]) % 1_000_000_007;
            }

            if i + zero <= high {
                dp[i + zero] = (dp[i + zero] + dp[i]) % 1_000_000_007;
            }
            if i + one <= high {
                dp[i + one] = (dp[i + one] + dp[i]) % 1_000_000_007;
            }
        }

        ret
    }
}
