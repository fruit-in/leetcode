impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += (nums[i - 1] - nums[i]) * i as i32;
        }

        ret
    }
}
