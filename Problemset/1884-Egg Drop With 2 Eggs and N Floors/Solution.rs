impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = (0..=n).collect::<Vec<_>>();

        for i in 1..=n {
            for x in 1..=i {
                dp[i] = dp[i].min(1 + dp[i - x].max(x - 1));
            }
        }

        dp[n] as i32
    }
}
