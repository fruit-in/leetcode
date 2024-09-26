impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![usize::MAX; n as usize + 1];
        dp[1] = 0;

        for i in 1..dp.len() {
            for j in (i..dp.len()).step_by(i) {
                dp[j] = dp[j].min(dp[i] + j / i);
            }
        }

        dp[n as usize] as i32
    }
}
