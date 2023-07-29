impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        let mut ret = vec![0; nums.len()];

        for j in 0..nums.len() {
            if j + 1 < nums.len() && nums[j] == nums[j + 1] {
                nums[j] += nums[j];
                nums[j + 1] = 0;
            }

            if nums[j] != 0 {
                ret[i] = nums[j];
                i += 1;
            }
        }

        ret
    }
}
