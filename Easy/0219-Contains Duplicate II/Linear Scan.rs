impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let len = nums.len();
        for i in 0..len {
            let mut j = i + 1;
            while j <= i + k as usize && j < len {
                if nums[i] == nums[j] {
                    return true;
                }
                j += 1;
            }
        }
        false
    }
}
