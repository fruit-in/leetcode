impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let mut dp = vec![vec![0.0; n as usize]; n as usize];
        dp[r as usize][c as usize] = 1.0;

        for _ in 0..k {
            let mut dp_ = vec![vec![0.0; n as usize]; n as usize];
            for r in 0..n {
                for c in 0..n {
                    dp_[r as usize][c as usize] = (Self::probability(&dp, n, r - 2, c + 1)
                        + Self::probability(&dp, n, r - 1, c + 2)
                        + Self::probability(&dp, n, r + 1, c + 2)
                        + Self::probability(&dp, n, r + 2, c + 1)
                        + Self::probability(&dp, n, r + 2, c - 1)
                        + Self::probability(&dp, n, r + 1, c - 2)
                        + Self::probability(&dp, n, r - 1, c - 2)
                        + Self::probability(&dp, n, r - 2, c - 1))
                        / 8.0;
                }
            }
            dp = dp_;
        }

        dp.concat().iter().sum::<f64>()
    }

    fn probability(dp: &[Vec<f64>], n: i32, r: i32, c: i32) -> f64 {
        if r < 0 || c < 0 || r >= n || c >= n {
            0.0
        } else {
            dp[r as usize][c as usize]
        }
    }
}
