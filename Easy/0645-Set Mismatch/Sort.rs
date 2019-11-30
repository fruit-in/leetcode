impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut dup = 0;
        let mut miss = nums.len() as i32;

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                dup = nums[i];
            } else if nums[i - 1] == nums[i] - 2 {
                miss = nums[i] - 1;
            }
        }

        match nums[0] {
            1 => vec![dup, miss],
            _ => vec![dup, 1],
        }
    }
}
