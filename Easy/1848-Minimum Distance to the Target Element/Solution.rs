impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;

        for i in 0..nums.len() {
            if nums[(start + i).min(nums.len() - 1)] == target
                || nums[start.saturating_sub(i)] == target
            {
                return i as i32;
            }
        }

        unreachable!()
    }
}
