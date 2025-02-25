impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let max_time = max_time as usize;
        let n = edges.iter().map(|edge| edge[0].max(edge[1])).max().unwrap() as usize + 1;
        let mut dp = vec![vec![i32::MAX; n]; max_time + 1];
        let mut ret = i32::MAX;
        dp[0][0] = passing_fees[0];

        for t in 0..=max_time {
            for edge in &edges {
                let (x, y, time) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);

                if t >= time {
                    if dp[t - time][y] != i32::MAX {
                        dp[t][x] = dp[t][x].min(dp[t - time][y] + passing_fees[x]);
                    }
                    if dp[t - time][x] != i32::MAX {
                        dp[t][y] = dp[t][y].min(dp[t - time][x] + passing_fees[y]);
                    }
                }
            }

            ret = ret.min(dp[t][n - 1]);
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }
}
