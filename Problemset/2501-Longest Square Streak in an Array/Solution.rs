use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut nums_set = nums.clone().into_iter().collect::<HashSet<_>>();
        let mut ret = -1;

        nums.sort_unstable();

        for &num in &nums {
            let mut length = 1;
            let mut x = num;

            while x < 317 && nums_set.remove(&(x * x)) {
                x *= x;
                length += 1;
            }

            if length > ret.max(1) {
                ret = length
            }
        }

        ret
    }
}
