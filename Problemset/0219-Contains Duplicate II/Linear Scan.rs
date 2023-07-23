impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let len = nums.len();
        for i in 0..len {
            for n in &nums[(i + 1)..len.min(i + 1 + k)] {
                if nums[i] - n == 0 {
                    return true;
                }
            }
        }
        false
    }
}
