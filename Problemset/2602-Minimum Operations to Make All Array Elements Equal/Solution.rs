impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let (n, m) = (nums.len(), queries.len());
        let mut nums = nums;
        let mut prefix_sum = vec![0; n];
        let mut answer = vec![0; m];

        nums.sort_unstable();
        prefix_sum[0] = nums[0] as i64;

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
        }

        for i in 0..queries.len() {
            let (j, x) = match nums.binary_search(&queries[i]) {
                Ok(0) | Err(0) => (0, 0),
                Ok(k) | Err(k) => (k as i64, prefix_sum[k - 1]),
            };

            answer[i] = (2 * j - n as i64) * queries[i] as i64 + prefix_sum[n - 1] - 2 * x;
        }

        answer
    }
}
