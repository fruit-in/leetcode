impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_index = 0;

        for i in 0..nums.len() {
            if i > max_index as usize {
                return false;
            }
            max_index = max_index.max(i as i32 + nums[i]);
        }

        true
    }
}
