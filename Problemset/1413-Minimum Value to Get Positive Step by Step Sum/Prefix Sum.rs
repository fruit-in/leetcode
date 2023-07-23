impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        1 - nums.into_iter().min().unwrap().min(0)
    }
}
