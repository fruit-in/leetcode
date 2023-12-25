impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![(0, 0); questions.len()];

        for i in (0..dp.len()).rev() {
            let (points, brainpower) = (questions[i][0] as i64, questions[i][1] as usize);

            dp[i].0 = points;
            if i + brainpower + 1 < dp.len() {
                dp[i].0 += dp[i + brainpower + 1].0.max(dp[i + brainpower + 1].1);
            }
            if i + 1 < dp.len() {
                dp[i].1 = dp[i + 1].0.max(dp[i + 1].1);
            }
        }

        dp[0].0.max(dp[0].1)
    }
}
