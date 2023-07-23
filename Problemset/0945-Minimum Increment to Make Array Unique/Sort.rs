impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += nums[i].max(nums[i - 1] + 1) - nums[i];
            nums[i] = nums[i].max(nums[i - 1] + 1);
        }

        ret
    }
}
