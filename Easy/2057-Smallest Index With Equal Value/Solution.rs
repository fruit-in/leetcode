impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .find(|&i| i as i32 % 10 == nums[i])
            .map_or(-1, |x| x as i32)
    }
}
