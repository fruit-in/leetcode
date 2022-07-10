impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .max()
            .unwrap()
    }
}
