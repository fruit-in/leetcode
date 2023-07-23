impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = (cost[0], cost[1]);

        for i in 2..cost.len() {
            dp = (dp.1, dp.0.min(dp.1) + cost[i]);
        }

        dp.0.min(dp.1)
    }
}
