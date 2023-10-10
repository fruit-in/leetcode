impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];

        for i in 0..dp.len() {
            let mut max_val = 0;

            for j in 0..(k as usize).min(dp.len() - i - 1) {
                max_val = max_val.max(arr[i + j]);
                dp[i + j + 1] = dp[i + j + 1].max(dp[i] + max_val * (j as i32 + 1));
            }
        }

        *dp.last().unwrap()
    }
}
