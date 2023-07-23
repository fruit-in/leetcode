impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max_len = 0;

        let mut i = 0;
        let mut j = 1;
        while i < nums.len() {
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }

            let k = j;

            if j < nums.len() && nums[j] == nums[i] + 1 {
                while j < nums.len() && nums[j] == nums[i] + 1 {
                    j += 1;
                }

                max_len = max_len.max(j - i);
            }

            i = k;
        }

        max_len as i32
    }
}
