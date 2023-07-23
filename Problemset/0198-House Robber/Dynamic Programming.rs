impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        let mut dp = vec![
            nums[0],
            nums[0].max(nums[1]),
            nums[1].max(nums[0] + nums[2])
        ];
        for i in 3..nums.len() {
            dp.push((dp[i - 3] + nums[i - 1]).max(dp[i - 2] + nums[i]));
        }
        dp.pop().unwrap()
    }
}
