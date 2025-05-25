impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        for i in 0..=target {
            for &x in &nums {
                if i >= x {
                    dp[i as usize] += dp[(i - x) as usize];
                }
            }
        }

        dp[target as usize]
    }
}
