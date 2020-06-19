impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp = vec![vec![vec![]; n + 1]; n + 1];
        dp[0][0].push(String::new());

        for i in 0..=n {
            for j in 0..=i {
                if i == n && j == i {
                    return dp[i].pop().unwrap()
                }

                while let Some(s) = dp[i][j].pop() {
                    if i < n {
                        dp[i + 1][j].push(s.clone() + "(");
                    }
                    if i > j {
                        dp[i][j + 1].push(s + ")");
                    }
                }
            }
        }

        vec![]
    }
}
