impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; nums.len() + 1];

        for i in 1..=nums.len() {
            dp[i][1] = dp[i - 1][1] + nums[i - 1];
        }

        for i in 2..=nums.len() {
            for j in 2..=i.min(k as usize) {
                dp[i][j] = i32::MAX;
                for x in j - 1..i {
                    dp[i][j] = dp[i][j].min(dp[x][j - 1].max(dp[i][1] - dp[x][1]));
                }
            }
        }

        dp[nums.len()][k as usize]
    }
}
