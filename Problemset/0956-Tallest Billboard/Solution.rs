impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let rods = rods.into_iter().map(|r| r as usize).collect::<Vec<_>>();
        let limit = rods.iter().sum::<usize>() / 2;
        let mut dp = vec![vec![0; limit + 1]; rods.len()];

        if rods[0] <= limit {
            dp[0][rods[0]] = rods[0];
        }

        for i in 1..rods.len() {
            for j in 0..=limit {
                if j == 0 || dp[i - 1][j] != 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                    if (j + rods[i]).max(dp[i - 1][j] + rods[i]) <= limit {
                        dp[i][j + rods[i]] = dp[i][j + rods[i]].max(dp[i - 1][j] + rods[i]);
                    }
                    if j >= rods[i] {
                        dp[i][j - rods[i]] = dp[i][j - rods[i]].max(dp[i - 1][j]);
                    } else if dp[i - 1][j] + rods[i] - j <= limit {
                        dp[i][rods[i] - j] = dp[i][rods[i] - j].max(dp[i - 1][j] + rods[i] - j);
                    }
                }
            }
        }

        dp.last().unwrap()[0] as i32
    }
}
