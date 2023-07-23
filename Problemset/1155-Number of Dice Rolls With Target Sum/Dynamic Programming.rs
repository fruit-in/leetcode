impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        if target < d || target > d * f {
            return 0;
        }

        let d = d as usize;
        let f = f as usize;
        let target = target as usize;
        let mut dp = vec![vec![0; target + 1]; d + 1];
        dp[0][0] = 1;

        for i in 0..d {
            for j in i.max(target.saturating_sub((d - i) * f))..=(i * f).min(target - d + i) {
                for k in 1..=f.min(target - j) {
                    dp[i + 1][j + k] += dp[i][j];
                    dp[i + 1][j + k] %= 1_000_000_007;
                }
            }
        }

        dp[d][target]
    }
}
