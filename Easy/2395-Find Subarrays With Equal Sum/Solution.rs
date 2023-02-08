use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut subarray_sums = HashSet::new();

        for i in 1..nums.len() {
            if subarray_sums.contains(&(nums[i] + nums[i - 1])) {
                return true;
            }
            subarray_sums.insert(nums[i] + nums[i - 1]);
        }

        false
    }
}
