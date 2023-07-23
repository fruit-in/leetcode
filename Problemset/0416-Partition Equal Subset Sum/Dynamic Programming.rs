impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        let target = sum / 2;
        if sum % 2 == 1 || nums.iter().any(|&x| x as usize > target) {
            return false;
        }
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for x in nums {
            for i in (0..=target - x as usize).rev() {
                dp[i + x as usize] |= dp[i];
            }
        }

        dp[target]
    }
}
