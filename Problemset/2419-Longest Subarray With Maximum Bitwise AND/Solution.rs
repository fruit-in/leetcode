impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_num = nums[0];
        let mut count = 1;
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] > max_num {
                max_num = nums[i];
                count = 1;
                ret = 1;
            } else if nums[i] == max_num && nums[i] != nums[i - 1] {
                count = 1;
            } else if nums[i] == max_num {
                count += 1;
                ret = ret.max(count);
            }
        }

        ret
    }
}
