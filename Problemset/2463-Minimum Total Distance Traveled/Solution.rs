impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();

        let m = robot.len();
        let n = factory.len();
        let mut dp = vec![vec![i64::MAX; m + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            for j in 0..=m {
                let mut distance = 0;
                dp[i][j] = dp[i - 1][j];

                for k in 1..=j.min(factory[i - 1][1] as usize) {
                    distance += (factory[i - 1][0] - robot[j - k]).abs() as i64;
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - k].saturating_add(distance));
                }
            }
        }

        dp[n][m]
    }
}
