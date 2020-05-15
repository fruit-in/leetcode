impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let len = a.len();
        let mut dp = a;

        for i in 1..len {
            for j in 0..len {
                dp[i][j] += *dp[i - 1][(j.max(1) - 1)..(j + 2).min(len)]
                    .iter()
                    .min()
                    .unwrap();
            }
        }

        *dp.last().unwrap().iter().min().unwrap()
    }
}
