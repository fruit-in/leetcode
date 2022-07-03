impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();

        (0..=nums.len() - k)
            .map(|i| nums[i + k - 1] - nums[i])
            .min()
            .unwrap()
    }
}
