impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut dp = vec![vec![0; 1 << n]; 41];
        dp[0][0] = 1;

        for i in 1..=40 {
            dp[i] = dp[i - 1].clone();

            for j in 0..n {
                if !hats[j].contains(&(i as i32)) {
                    continue;
                }

                for k in 0..(1 << n) {
                    if (k >> j) & 1 == 0 {
                        dp[i][k | (1 << j)] = (dp[i][k | (1 << j)] + dp[i - 1][k]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[40][(1 << n) - 1]
    }
}
