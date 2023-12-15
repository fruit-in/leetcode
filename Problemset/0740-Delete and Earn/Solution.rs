impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut sums = vec![0; *nums.iter().max().unwrap() as usize + 1];
        let mut dp = vec![(0, 0); sums.len()];

        for &num in &nums {
            sums[num as usize] += num;
        }

        dp[1] = (0, sums[1]);

        for i in 2..dp.len() {
            dp[i].0 = dp[i - 1].0.max(dp[i - 1].1);
            dp[i].1 = sums[i] + dp[i - 1].0.max(dp[i - 2].1);
        }

        dp[dp.len() - 1].0.max(dp[dp.len() - 1].1)
    }
}
