impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let houses = houses.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![None; n]; target + 1]; m];

        if houses[0] > 0 {
            dp[0][1][houses[0] as usize - 1] = Some(0);
        } else {
            for j in 0..n {
                dp[0][1][j] = Some(cost[0][j]);
            }
        }

        for i in 1..m {
            for k in 1..=target {
                for j in 0..n {
                    if let Some(x) = dp[i - 1][k][j] {
                        if houses[i] > 0 {
                            if houses[i] - 1 == j {
                                dp[i][k][j] = Some(dp[i][k][j].unwrap_or(i32::MAX).min(x));
                            } else if k + 1 <= target {
                                dp[i][k + 1][houses[i] - 1] =
                                    Some(dp[i][k + 1][houses[i] - 1].unwrap_or(i32::MAX).min(x));
                            }
                        } else {
                            for jj in 0..n {
                                if jj == j {
                                    dp[i][k][jj] =
                                        Some(dp[i][k][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]));
                                } else if k + 1 <= target {
                                    dp[i][k + 1][jj] = Some(
                                        dp[i][k + 1][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        dp[m - 1][target]
            .iter()
            .filter_map(|&x| x)
            .min()
            .unwrap_or(-1)
    }
}
