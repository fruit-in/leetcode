impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                return true;
            }
        }
        false
    }
}
