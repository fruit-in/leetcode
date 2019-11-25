impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        for i in 0..nums.len() {
            if nums.contains(&(nums[i] + 1)) {
                let mut len = 0;

                for j in 0..nums.len() {
                    if nums[j] == nums[i] || nums[j] == nums[i] + 1 {
                        len += 1;
                    }
                }

                max_len = max_len.max(len);
            }
        }

        max_len as i32
    }
}
