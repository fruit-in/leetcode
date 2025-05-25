impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k - 1 + max_pts <= n || k == 0 {
            return 1.;
        }

        let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);
        let mut window_sum = 1.;
        let mut dp = vec![0.; n + 1];
        let mut ret = 0.;
        dp[0] = 1.;

        for i in 1..=n {
            dp[i] = window_sum / max_pts as f64;
            if i >= max_pts {
                window_sum -= dp[i - max_pts];
            }
            if i >= k {
                ret += dp[i];
            } else {
                window_sum += dp[i];
            }
        }

        ret
    }
}
