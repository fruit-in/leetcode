use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }

        let stones = stones.into_iter().map(|s| s as usize).collect::<Vec<_>>();
        let mut indices = stones
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<_, _>>();
        let mut dp = vec![vec![false; stones.len()]; stones.len()];
        dp[1][1] = true;

        for i in 1..stones.len() {
            for k in 1..=i {
                if dp[i][k] {
                    if i == stones.len() - 1 {
                        return true;
                    }

                    if let Some(&j) = indices.get(&(stones[i] + k - 1)) {
                        dp[j][k - 1] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k)) {
                        dp[j][k] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k + 1)) {
                        dp[j][k + 1] = true;
                    }
                }
            }
        }

        false
    }
}
