impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; days.len() + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            for j in i..dp.len() {
                if days[j - 1] - days[i - 1] >= 30 {
                    break;
                } else if days[j - 1] - days[i - 1] >= 7 {
                    dp[j] = dp[j].min(dp[i - 1] + costs[2]);
                } else if days[j - 1] - days[i - 1] >= 1 {
                    dp[j] = dp[j].min(dp[i - 1] + costs[1].min(costs[2]));
                } else {
                    dp[j] = dp[j].min(dp[i - 1] + costs[0].min(costs[1]).min(costs[2]));
                }
            }
        }

        *dp.last().unwrap()
    }
}
