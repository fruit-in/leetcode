impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}
