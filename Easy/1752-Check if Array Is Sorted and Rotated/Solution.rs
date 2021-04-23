impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        match (1..nums.len()).filter(|&i| nums[i] < nums[i - 1]).count() {
            0 => true,
            1 => nums[0] >= nums[nums.len() - 1],
            _ => false,
        }
    }
}
