impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let next_visit = next_visit.iter().map(|&x| x as usize).collect::<Vec<_>>();
        let n = next_visit.len();
        let mut dp = vec![[0, 1_i32]; n];

        for i in 1..n {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1] + 1) % 1_000_000_007;
            if next_visit[i] != i {
                dp[i][1] = dp[i - 1][0] - dp[next_visit[i]][0] + dp[i - 1][1] + 2;
                dp[i][1] = dp[i][1].rem_euclid(1_000_000_007);
            }
        }

        dp[n - 1][0]
    }
}
