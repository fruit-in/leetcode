impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        while nums.len() > 1 {
            nums = (0..nums.len() - 1)
                .map(|i| (nums[i] + nums[i + 1]) % 10)
                .collect();
        }

        nums[0]
    }
}
